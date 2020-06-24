use super::value::{PrimitiveValueType, SingleValue};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Token,
};

pub struct AllOrdered {
    pub components: Vec<SingleValue>,
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
impl Parse for AllUnordered {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let components = Punctuated::parse_separated_nonempty(input)?;
        Ok(Self { components })
    }
}

pub struct OneOrMoreUnordered {
    pub components: Punctuated<AllUnordered, Token![||]>,
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
