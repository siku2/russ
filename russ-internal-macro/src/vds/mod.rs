use combined::CombinedValue;
use syn::{
    parse::{Parse, ParseStream},
    Token,
};
use value::{PropertyReference, Reference};

mod combined;
mod multiplier;
mod value;

pub struct DefinitionLine<T> {
    pub name: T,
    pub equals_sign: Token![=],
    pub value: CombinedValue,
    pub semicolon: Token![;],
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

pub type PropertyDefinition = DefinitionLine<PropertyReference>;
pub type ValueDefinition = DefinitionLine<Reference>;

pub struct VDS {
    pub property: PropertyDefinition,
    pub values: Vec<ValueDefinition>,
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
