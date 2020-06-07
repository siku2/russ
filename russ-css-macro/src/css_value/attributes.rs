use crate::args::{Args, FromArgs, ParseAttr};
use quote::ToTokens;
use syn::{Attribute, LitStr};

pub struct DimensionAttr {
    attr: Attribute,
    pub zero: bool,
    pub unit: Option<LitStr>,
}
impl FromArgs for DimensionAttr {
    fn attr_path() -> &'static str {
        "dimension"
    }

    fn from_args(attr: Attribute, args: &Args) -> syn::Result<Self> {
        Ok(Self {
            attr,
            zero: args.has_flag("zero"),
            unit: args.get_kwarg_str("unit").transpose()?.cloned(),
        })
    }
}
impl ToTokens for DimensionAttr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.attr.to_tokens(tokens)
    }
}

pub struct KeywordAttr {
    attr: Attribute,
    pub value: Option<LitStr>,
}
impl FromArgs for KeywordAttr {
    fn attr_path() -> &'static str {
        "keyword"
    }
    fn from_args(attr: Attribute, args: &Args) -> syn::Result<Self> {
        Ok(Self {
            attr,
            value: args.get_kwarg_str("value").transpose()?.cloned(),
        })
    }
}
impl ToTokens for KeywordAttr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.attr.to_tokens(tokens)
    }
}

pub struct ValueAttr {
    attr: Attribute,
    pub separator: Option<LitStr>,
}
impl FromArgs for ValueAttr {
    fn attr_path() -> &'static str {
        "value"
    }
    fn from_args(attr: Attribute, args: &Args) -> syn::Result<Self> {
        Ok(Self {
            attr,
            separator: args.get_kwarg_str("separator").transpose()?.cloned(),
        })
    }
}
impl ToTokens for ValueAttr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.attr.to_tokens(tokens)
    }
}

pub enum CSSValueAttr {
    Dimension(DimensionAttr),
    Keyword(KeywordAttr),
    Value(ValueAttr),
}
impl ParseAttr for CSSValueAttr {
    fn parse_attr(attr: &Attribute) -> Option<syn::Result<Self>> {
        Some({
            if let Some(attr) = DimensionAttr::parse_attr(attr) {
                attr.map(Self::Dimension)
            } else if let Some(attr) = KeywordAttr::parse_attr(attr) {
                attr.map(Self::Keyword)
            } else if let Some(attr) = ValueAttr::parse_attr(attr) {
                attr.map(Self::Value)
            } else {
                return None;
            }
        })
    }
}

impl ToTokens for CSSValueAttr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::Dimension(attr) => attr.to_tokens(tokens),
            Self::Keyword(attr) => attr.to_tokens(tokens),
            Self::Value(attr) => attr.to_tokens(tokens),
        }
    }
}
