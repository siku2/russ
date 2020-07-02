use crate::ToRussRepr;
use std::iter;
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    token, Ident, LitStr,
};

// https://www.w3.org/TR/css-values-4/#textual-values

// TODO use CSSIdent from vds by moving it to another crate (css-vds).
pub type CssIdent = Ident;

pub type CustomIdent = CssIdent;
impl ToRussRepr for CustomIdent {
    fn to_russ_repr(&self) -> syn::Expr {
        todo!()
    }
}

pub type CssString = LitStr;
impl ToRussRepr for CssString {
    fn to_russ_repr(&self) -> syn::Expr {
        todo!()
    }
}

pub enum UrlModifier {
    Ident(CssIdent),
    Function(),
}
impl Parse for UrlModifier {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // TODO Function syntax needs to be supported @ russ first.
        input.parse().map(Self::Ident)
    }
}

mod kw {
    syn::custom_keyword!(url);
}

pub struct Url {
    pub url_token: kw::url,
    pub paren_token: token::Paren,
    pub location: Option<CssString>,
    pub modifiers: Vec<UrlModifier>,
}
impl Url {
    pub fn peek(input: ParseStream) -> bool {
        input.peek(kw::url) && input.peek2(token::Paren)
    }
}
impl Parse for Url {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let url_token = input.parse()?;

        let content;
        let paren_token = syn::parenthesized!(content in input);
        let location = if content.peek(LitStr) {
            Some(content.parse()?)
        } else {
            None
        };
        let modifiers = iter::from_fn(|| {
            if content.is_empty() {
                None
            } else {
                Some(content.parse())
            }
        })
        .collect::<Result<_, _>>()?;

        Ok(Self {
            url_token,
            paren_token,
            location,
            modifiers,
        })
    }
}
impl Spanned for Url {
    fn span(&self) -> proc_macro2::Span {
        let Self {
            url_token,
            paren_token,
            ..
        } = self;
        let url_span = url_token.span();
        url_span.join(paren_token.span).unwrap_or(url_span)
    }
}
