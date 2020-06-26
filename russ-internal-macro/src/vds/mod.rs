use combined::CombinedValue;
use generate::{GenerateTypeContext, GenerateTypeInfo, TypeDefinition, TypeInfo};
use proc_macro2::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    parse_quote, Ident, Token,
};
use value::{PropertyReference, Reference};

mod combined;
mod generate;
mod helpers;
mod multiplier;
mod value;

pub struct DefinitionLine<T> {
    pub name: T,
    pub equals_sign: Token![=],
    pub value: CombinedValue,
    pub semicolon: Token![;],
}
impl<T> DefinitionLine<T> {
    pub fn gen_type_info(&self, ctx: &GenerateTypeContext, ident: Ident) -> syn::Result<TypeInfo> {
        let ident = ctx.propose_ident(&ident.to_string())?;
        let value_ty = self.value.gen_type_info(ctx)?;
        let inner_value_ty = &value_ty.value_type;
        let ty = parse_quote! { #ident };
        let def = parse_quote! {
            pub struct #ident(#inner_value_ty);
        };
        Ok(TypeInfo::new(ty)
            .with_definition(TypeDefinition::new(ident, def))
            .with_dependencies(vec![value_ty]))
    }
}
impl<T: Parse> Parse for DefinitionLine<T> {
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

pub struct PropertyDefinition(pub DefinitionLine<PropertyReference>);
impl GenerateTypeInfo for PropertyDefinition {
    fn gen_type_info(&self, ctx: &GenerateTypeContext) -> syn::Result<TypeInfo> {
        let ident = self.0.name.prop_ident()?;
        self.0.gen_type_info(ctx, ident)
    }
}
impl Parse for PropertyDefinition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse().map(Self)
    }
}
pub struct ValueDefinition(pub DefinitionLine<Reference>);
impl GenerateTypeInfo for ValueDefinition {
    fn gen_type_info(&self, ctx: &GenerateTypeContext) -> syn::Result<TypeInfo> {
        let ident = self.0.name.ref_ident()?;
        self.0.gen_type_info(ctx, ident)
    }
}
impl Parse for ValueDefinition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse().map(Self)
    }
}

pub struct VDS {
    pub property: PropertyDefinition,
    pub values: Vec<ValueDefinition>,
}
impl VDS {
    pub fn gen_dependencies(&self, ctx: &GenerateTypeContext) -> syn::Result<Vec<TypeInfo>> {
        self.values.iter().map(|v| v.gen_type_info(ctx)).collect()
    }

    pub fn test(&self) -> syn::Result<TokenStream> {
        let info = self.gen_type_info(&GenerateTypeContext::empty())?;
        Ok(info.gen_definitions())
    }
}
impl GenerateTypeInfo for VDS {
    fn gen_type_info(&self, ctx: &GenerateTypeContext) -> syn::Result<TypeInfo> {
        let mut info = self.property.gen_type_info(ctx)?;
        info.dependencies.extend(self.gen_dependencies(ctx)?);
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

        Ok(Self { property, values })
    }
}
