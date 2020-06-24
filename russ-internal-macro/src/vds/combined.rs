use super::value::{PrimitiveValueType, SingleValue};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Token,
};

pub struct AllOrdered {
    pub components: Vec<SingleValue>,
}
impl AllOrdered {
    pub fn unpack_one(&self) -> Option<&SingleValue> {
        match self.components.as_slice() {
            [first] => Some(first),
            _ => None,
        }
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
    pub fn unpack_one(&self) -> Option<&AllOrdered> {
        let comps = &self.components;
        if let Some(first) = comps.first() {
            if comps.len() == 1 {
                return Some(first);
            }
        }
        None
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
    pub fn unpack_one(&self) -> Option<&AllUnordered> {
        let comps = &self.components;
        if let Some(first) = comps.first() {
            if comps.len() == 1 {
                return Some(first);
            }
        }
        None
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
    pub fn unpack_one(&self) -> Option<&OneOrMoreUnordered> {
        let comps = &self.components;
        if let Some(first) = comps.first() {
            if comps.len() == 1 {
                return Some(first);
            }
        }
        None
    }
}
impl Parse for Enumeration {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let components = Punctuated::parse_separated_nonempty(input)?;
        Ok(Self { components })
    }
}

#[derive(Clone, Debug)]
pub enum CombinedValueType {
    AllOrdered,
    AllUnordered,
    OneOrMoreUnordered,
    Enumeration,
}
impl CombinedValueType {
    pub fn peek_variant_separator(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![&&]) {
            Ok(Self::AllUnordered)
        } else if lookahead.peek(Token![||]) {
            Ok(Self::OneOrMoreUnordered)
        } else if lookahead.peek(Token![|]) {
            Ok(Self::Enumeration)
        } else if !input.is_empty() {
            Ok(Self::AllOrdered)
        } else {
            Err(lookahead.error())
        }
    }

    pub fn peek_variant(input: ParseStream) -> syn::Result<Self> {
        let input = input.fork();
        input.parse::<SingleValue>()?;
        Self::peek_variant_separator(&input)
    }
}

pub enum CombinedValue {
    AllOrdered(AllOrdered),
    AllUnordered(AllUnordered),
    OneOrMoreUnordered(OneOrMoreUnordered),
    Enumeration(Enumeration),
}
impl Parse for CombinedValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        match CombinedValueType::peek_variant(input)? {
            CombinedValueType::AllOrdered => input.parse().map(Self::AllOrdered),
            CombinedValueType::AllUnordered => input.parse().map(Self::AllUnordered),
            CombinedValueType::OneOrMoreUnordered => input.parse().map(Self::OneOrMoreUnordered),
            CombinedValueType::Enumeration => input.parse().map(Self::Enumeration),
        }
    }
}
