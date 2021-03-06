use combined::CombinedValue;
use generate::{GenerateTypeContext, GenerateTypeInfo, TypeDefinition, TypeInfo};
use proc_macro2::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    parse_quote, Ident, LitStr, Token,
};
pub use value::CssIdent;
use value::{PropertyReference, Reference};

mod combined;
mod generate;
mod helpers;
mod multiplier;
mod value;

// TODO use syn::custom_keyword! for keywords

pub struct GenericDefinitionLine<T> {
    pub name: T,
    pub equals_sign: Token![=],
    pub value: CombinedValue,
    pub semicolon: Token![;],
}
impl<T> GenericDefinitionLine<T> {
    pub fn gen_type_info(&self, ctx: &GenerateTypeContext, ident: &Ident) -> syn::Result<TypeInfo> {
        let (ident, ctx) = ctx.fork_namespace(ident)?;
        let value_ty = self.value.gen_type_info(ctx)?;
        let inner_value_ty = value_ty.value_type_unwrap_tuple();
        let ty = parse_quote! { #ident };
        let def = parse_quote! {
            pub struct #ident(#inner_value_ty);
        };
        Ok(TypeInfo::new(ty)
            .with_definition(TypeDefinition::new(ident, def))
            .with_dependencies(vec![value_ty]))
    }
}
impl<T: Parse> Parse for GenericDefinitionLine<T> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let equals_sign = input.parse()?;
        let value = input.parse()?;
        let semicolon = input.parse()?;
        Ok(Self {
            name,
            equals_sign,
            value,
            semicolon,
        })
    }
}

pub struct PropertyDefinition(pub GenericDefinitionLine<PropertyReference>);
impl GenerateTypeInfo for PropertyDefinition {
    fn gen_type_info(&self, ctx: GenerateTypeContext) -> syn::Result<TypeInfo> {
        let ident = self.0.name.prop_ident()?;
        self.0.gen_type_info(&ctx, &ident)
    }
}
impl Parse for PropertyDefinition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse().map(Self)
    }
}
pub struct ValueDefinition(pub GenericDefinitionLine<Reference>);
impl GenerateTypeInfo for ValueDefinition {
    fn gen_type_info(&self, ctx: GenerateTypeContext) -> syn::Result<TypeInfo> {
        let ident = self.0.name.ref_ident()?;
        self.0.gen_type_info(&ctx, &ident)
    }
}
impl Parse for ValueDefinition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse().map(Self)
    }
}

pub enum Definition {
    Property(PropertyDefinition),
    Value(ValueDefinition),
}
impl GenerateTypeInfo for Definition {
    fn gen_type_info(&self, ctx: GenerateTypeContext) -> syn::Result<TypeInfo> {
        match self {
            Self::Property(v) => v.gen_type_info(ctx),
            Self::Value(v) => v.gen_type_info(ctx),
        }
    }
}
impl Parse for Definition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek2(LitStr) {
            input.parse().map(Self::Property)
        } else {
            input.parse().map(Self::Value)
        }
    }
}

pub struct VDS {
    pub definition: Definition,
    pub values: Vec<ValueDefinition>,
}
impl VDS {
    pub fn gen_dependencies(&self) -> syn::Result<Vec<TypeInfo>> {
        self.values
            .iter()
            .map(|v| v.gen_type_info(GenerateTypeContext::empty()))
            .collect()
    }

    pub fn test(&self) -> syn::Result<TokenStream> {
        let info = self.gen_type_info(GenerateTypeContext::empty())?;
        Ok(info.gen_definitions())
    }
}
impl GenerateTypeInfo for VDS {
    fn gen_type_info(&self, ctx: GenerateTypeContext) -> syn::Result<TypeInfo> {
        let mut info = self.definition.gen_type_info(ctx)?;
        info.dependencies.extend(self.gen_dependencies()?);
        Ok(info)
    }
}
impl Parse for VDS {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let property = input.parse()?;
        let mut values = Vec::new();
        while !input.is_empty() {
            values.push(input.parse()?);
        }

        Ok(Self {
            definition: property,
            values,
        })
    }
}
