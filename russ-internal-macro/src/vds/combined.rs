use super::{
    generate::{GenerateTypeContext, GenerateTypeInfo, TypeDefinition, TypeInfo},
    value::{PrimitiveValueType, SingleValue},
};
use quote::format_ident;
use syn::{
    parse::{Parse, ParseStream},
    parse_quote,
    punctuated::Punctuated,
    spanned::Spanned,
    Ident, Token, Type, Variant,
};

pub struct AllOrdered {
    pub components: Vec<SingleValue>,
}
impl AllOrdered {
    pub fn gen_component_types(&self, ctx: GenerateTypeContext) -> syn::Result<Vec<TypeInfo>> {
        self.components
            .iter()
            .enumerate()
            .map(|(i, v)| v.gen_type_info(ctx.fork_index(i)?))
            .collect()
    }
}
impl GenerateTypeInfo for AllOrdered {
    fn gen_type_info(&self, ctx: GenerateTypeContext) -> syn::Result<TypeInfo> {
        if self.components.len() == 1 {
            return self.components.first().unwrap().gen_type_info(ctx);
        }

        let deps = self.gen_component_types(ctx)?;
        let types_it = deps.iter().map(|d| &d.value_type);
        let ty = parse_quote! {
            (#(#types_it),*)
        };
        Ok(TypeInfo::new(ty).with_dependencies(deps))
    }
}
impl Parse for AllOrdered {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut components = Vec::new();
        components.push(input.parse()?);
        while PrimitiveValueType::peek_variant(input).is_ok() {
            components.push(input.parse()?);
        }

        Ok(Self { components })
    }
}

pub struct AllUnordered {
    pub components: Punctuated<AllOrdered, Token![&&]>,
}
impl AllUnordered {
    pub fn gen_component_types(&self, ctx: GenerateTypeContext) -> syn::Result<Vec<TypeInfo>> {
        self.components
            .iter()
            .enumerate()
            .map(|(i, comp)| comp.gen_type_info(ctx.fork_index(i)?))
            .collect()
    }
}
impl GenerateTypeInfo for AllUnordered {
    fn gen_type_info(&self, ctx: GenerateTypeContext) -> syn::Result<TypeInfo> {
        if self.components.len() == 1 {
            return self.components.first().unwrap().gen_type_info(ctx);
        }

        let deps = self.gen_component_types(ctx)?;
        let types_it = deps.iter().map(|d| &d.value_type);
        let ty = parse_quote! {
            (#(#types_it),*)
        };
        Ok(TypeInfo::new(ty).with_dependencies(deps))
    }
}
impl Parse for AllUnordered {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let components = Punctuated::parse_separated_nonempty(input)?;
        Ok(Self { components })
    }
}

pub struct OneOrMoreUnordered {
    pub components: Punctuated<AllUnordered, Token![||]>,
}
impl OneOrMoreUnordered {
    pub fn gen_component_types(&self, ctx: GenerateTypeContext) -> syn::Result<Vec<TypeInfo>> {
        self.components
            .iter()
            .enumerate()
            .map(|(i, comp)| comp.gen_type_info(ctx.fork_index(i)?))
            .collect()
    }
}
impl GenerateTypeInfo for OneOrMoreUnordered {
    fn gen_type_info(&self, ctx: GenerateTypeContext) -> syn::Result<TypeInfo> {
        if self.components.len() == 1 {
            return self.components.first().unwrap().gen_type_info(ctx);
        }

        let deps = self.gen_component_types(ctx)?;
        let opt_types_it = deps.iter().map(|d| -> Type {
            let ty = &d.value_type;
            parse_quote! { ::std::option::Option<#ty> }
        });
        let ty = parse_quote! {
            (#(#opt_types_it),*)
        };
        Ok(TypeInfo::new(ty).with_dependencies(deps))
    }
}
impl Parse for OneOrMoreUnordered {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let components = Punctuated::parse_separated_nonempty(input)?;
        Ok(Self { components })
    }
}

pub struct Enumeration {
    pub components: Punctuated<OneOrMoreUnordered, Token![|]>,
}
impl Enumeration {
    pub fn gen_component_types(&self, ctx: GenerateTypeContext) -> syn::Result<Vec<TypeInfo>> {
        self.components
            .iter()
            .enumerate()
            .map(|(i, comp)| comp.gen_type_info(ctx.fork_index(i)?))
            .collect()
    }

    fn gen_variant_ident(i: usize, info: &TypeInfo) -> Ident {
        info.get_name()
            .unwrap_or_else(|| format_ident!("V{:X}", i, span = info.value_type.span()))
    }

    fn gen_variant(i: usize, info: &TypeInfo) -> syn::Result<Variant> {
        let variant_ident = Self::gen_variant_ident(i, info);
        let variant_body = info.value_type_unwrap_tuple();

        Ok(parse_quote! {
            #variant_ident(#variant_body)
        })
    }

    pub fn gen_variants(deps: &[TypeInfo]) -> syn::Result<Vec<Variant>> {
        deps.iter()
            .enumerate()
            .map(|(i, ty)| Self::gen_variant(i, ty))
            .collect()
    }
}
impl GenerateTypeInfo for Enumeration {
    fn gen_type_info(&self, ctx: GenerateTypeContext) -> syn::Result<TypeInfo> {
        if self.components.len() == 1 {
            return self.components.first().unwrap().gen_type_info(ctx);
        }

        let ident = if let Some(ident) = ctx.get_ident() {
            ident
        } else {
            ctx.create_ident(&format_ident!("Inner"))?
        };
        let deps = self.gen_component_types(ctx)?;
        let variants = Self::gen_variants(&deps)?;
        let definition = parse_quote! {
            pub enum #ident {
                #(#variants),*
            }
        };
        let ty = parse_quote! { #ident };
        Ok(TypeInfo::new(ty)
            .with_definition(TypeDefinition::new(ident, definition))
            .with_dependencies(deps))
    }
}
impl Parse for Enumeration {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let components = Punctuated::parse_separated_nonempty(input)?;
        Ok(Self { components })
    }
}

pub enum CombinedValue {
    Single(SingleValue),
    AllOrdered(AllOrdered),
    AllUnordered(AllUnordered),
    OneOrMoreUnordered(OneOrMoreUnordered),
    Enumeration(Enumeration),
}
impl CombinedValue {
    pub fn len(&self) -> usize {
        match self {
            Self::Single(_) => 1,
            Self::AllOrdered(value) => value.components.len(),
            Self::AllUnordered(value) => value.components.len(),
            Self::OneOrMoreUnordered(value) => value.components.len(),
            Self::Enumeration(value) => value.components.len(),
        }
    }

    pub fn into_components(self) -> Vec<Self> {
        match self {
            Self::Single(_) => vec![],
            Self::AllOrdered(value) => value.components.into_iter().map(Self::from).collect(),
            Self::AllUnordered(value) => value.components.into_iter().map(Self::from).collect(),
            Self::OneOrMoreUnordered(value) => {
                value.components.into_iter().map(Self::from).collect()
            }
            Self::Enumeration(value) => value.components.into_iter().map(Self::from).collect(),
        }
    }
}
impl GenerateTypeInfo for CombinedValue {
    fn gen_type_info(&self, ctx: GenerateTypeContext) -> syn::Result<TypeInfo> {
        match self {
            Self::Single(value) => value.gen_type_info(ctx),
            Self::AllOrdered(value) => value.gen_type_info(ctx),
            Self::AllUnordered(value) => value.gen_type_info(ctx),
            Self::OneOrMoreUnordered(value) => value.gen_type_info(ctx),
            Self::Enumeration(value) => value.gen_type_info(ctx),
        }
    }
}
impl Parse for CombinedValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Enumeration>().map(Self::from)
    }
}
impl From<SingleValue> for CombinedValue {
    fn from(value: SingleValue) -> Self {
        Self::Single(value)
    }
}
impl From<AllOrdered> for CombinedValue {
    fn from(value: AllOrdered) -> Self {
        if value.components.len() == 1 {
            Self::from(value.components.into_iter().next().unwrap())
        } else {
            Self::AllOrdered(value)
        }
    }
}
impl From<AllUnordered> for CombinedValue {
    fn from(value: AllUnordered) -> Self {
        if value.components.len() == 1 {
            Self::from(value.components.into_iter().next().unwrap())
        } else {
            Self::AllUnordered(value)
        }
    }
}
impl From<OneOrMoreUnordered> for CombinedValue {
    fn from(value: OneOrMoreUnordered) -> Self {
        if value.components.len() == 1 {
            Self::from(value.components.into_iter().next().unwrap())
        } else {
            Self::OneOrMoreUnordered(value)
        }
    }
}
impl From<Enumeration> for CombinedValue {
    fn from(value: Enumeration) -> Self {
        if value.components.len() == 1 {
            Self::from(value.components.into_iter().next().unwrap())
        } else {
            Self::Enumeration(value)
        }
    }
}
