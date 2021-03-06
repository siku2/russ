use super::args::{Args, FromArgs, ParseAttr};
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
            unit: args.get_kwarg_str("unit")?,
        })
    }
}
impl ToTokens for DimensionAttr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.attr.to_tokens(tokens)
    }
}

pub struct FunctionAttr {
    attr: Attribute,
    pub name: Option<LitStr>,
    pub separator: Option<LitStr>,
}
impl FromArgs for FunctionAttr {
    fn attr_path() -> &'static str {
        "function"
    }

    fn from_args(attr: Attribute, args: &Args) -> syn::Result<Self> {
        Ok(Self {
            attr,
            name: args.get_kwarg_str("name")?,
            separator: args.get_kwarg_str("separator")?,
        })
    }
}
impl ToTokens for FunctionAttr {
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
            value: args.get_kwarg_str("value")?,
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
    pub prefix: Option<LitStr>,
    pub suffix: Option<LitStr>,
    pub separator: Option<LitStr>,
    pub write_fn: Option<LitStr>,
}
impl FromArgs for ValueAttr {
    fn attr_path() -> &'static str {
        "value"
    }

    fn from_args(attr: Attribute, args: &Args) -> syn::Result<Self> {
        Ok(Self {
            attr,
            prefix: args.get_kwarg_str("prefix")?,
            suffix: args.get_kwarg_str("suffix")?,
            separator: args.get_kwarg_str("separator")?,
            write_fn: args.get_kwarg_str("write_fn")?,
        })
    }
}
impl ToTokens for ValueAttr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.attr.to_tokens(tokens)
    }
}

pub enum CssValueAttr {
    Dimension(DimensionAttr),
    Function(FunctionAttr),
    Keyword(KeywordAttr),
    Value(ValueAttr),
}
impl ParseAttr for CssValueAttr {
    fn parse_attr(attr: &Attribute) -> Option<syn::Result<Self>> {
        Some({
            if let Some(attr) = DimensionAttr::parse_attr(attr) {
                attr.map(Self::Dimension)
            } else if let Some(attr) = FunctionAttr::parse_attr(attr) {
                attr.map(Self::Function)
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

impl ToTokens for CssValueAttr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::Dimension(attr) => attr.to_tokens(tokens),
            Self::Function(attr) => attr.to_tokens(tokens),
            Self::Keyword(attr) => attr.to_tokens(tokens),
            Self::Value(attr) => attr.to_tokens(tokens),
        }
    }
}
