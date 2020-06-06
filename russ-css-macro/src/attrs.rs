use crate::args::Args;
use quote::ToTokens;
use syn::{spanned::Spanned, Attribute, LitStr};

trait ParseAttr
where
    Self: Sized,
{
    fn attr_path() -> &'static str;
    fn from_args(attr: Attribute, args: &Args) -> syn::Result<Self>;

    fn parse_attr(attr: &Attribute) -> Option<syn::Result<Self>> {
        if attr.path.is_ident(Self::attr_path()) {
            Some(Args::from_attribute(attr).and_then(|args| Self::from_args(attr.clone(), &args)))
        } else {
            None
        }
    }
}

pub struct DimensionAttr {
    attr: Attribute,
    pub zero: bool,
    pub unit: Option<LitStr>,
}
impl ParseAttr for DimensionAttr {
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
impl ParseAttr for KeywordAttr {
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

pub enum Attr {
    Dimension(DimensionAttr),
    Keyword(KeywordAttr),
    None,
}
impl Attr {
    pub fn parse_attr(attr: &Attribute) -> syn::Result<Self> {
        if let Some(attr) = DimensionAttr::parse_attr(attr) {
            return attr.map(Self::Dimension);
        }

        if let Some(attr) = KeywordAttr::parse_attr(attr) {
            return attr.map(Self::Keyword);
        }

        Ok(Self::None)
    }

    pub fn first_from_attrs<'a>(
        attrs: impl IntoIterator<Item = &'a Attribute>,
    ) -> syn::Result<Self> {
        attrs
            .into_iter()
            .map(Self::parse_attr)
            .next()
            .unwrap_or(Ok(Self::None))
    }

    pub fn single_from_attrs<'a>(
        attrs: impl IntoIterator<Item = &'a Attribute>,
    ) -> syn::Result<Self> {
        let mut attrs_iter = attrs.into_iter();
        let first = Self::first_from_attrs(&mut attrs_iter)?;

        // check if there's another attribute
        match Self::first_from_attrs(&mut attrs_iter)? {
            Self::None => {}
            attr => {
                return Err(syn::Error::new_spanned(
                    attr,
                    "must only specify one attribute",
                ))
            }
        }

        Ok(first)
    }

    pub fn expect_no_attrs<'a>(attrs: impl IntoIterator<Item = &'a Attribute>) -> syn::Result<()> {
        let attr = Self::first_from_attrs(attrs)?;
        if matches!(attr, Self::None) {
            Ok(())
        } else {
            Err(syn::Error::new(attr.span(), "attribute not allowed here"))
        }
    }
}

impl ToTokens for Attr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::Dimension(attr) => attr.to_tokens(tokens),
            Self::Keyword(attr) => attr.to_tokens(tokens),
            Self::None => {}
        }
    }
}
