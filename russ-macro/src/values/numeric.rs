use proc_macro2::Span;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    LitFloat, LitInt, Token,
};

// https://www.w3.org/TR/css-values-4/#numeric-types

pub struct Integer {
    raw: (Option<Token![+]>, LitInt),
    pub value: i32,
}
impl Parse for Integer {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let plus = if input.peek(Token![+]) {
            Some(input.parse()?)
        } else {
            None
        };
        let lit: LitInt = input.parse()?;
        if plus.is_some() && lit.base10_digits().starts_with('-') {
            proc_macro_error::emit_error!(
                quote! { #plus#lit },
                "unexpected `-` at the start of a positive integer";
                hint = "remove the `+` if you want a negative integer or the `-` if you want a positive one";
            );
        }

        let value = lit.base10_parse()?;

        Ok(Self {
            raw: (plus, lit),
            value,
        })
    }
}
impl Spanned for Integer {
    fn span(&self) -> Span {
        let Self {
            raw: (plus, lit), ..
        } = self;
        (quote! { #plus#lit }).span()
    }
}

pub struct Number {
    raw: (Option<Token![+]>, LitFloat),
    pub value: f64,
}
impl Parse for Number {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let plus = if input.peek(Token![+]) {
            Some(input.parse()?)
        } else {
            None
        };
        // TODO does this also handle integers? TEST!
        let lit: LitFloat = input.parse()?;
        if plus.is_some() && lit.base10_digits().starts_with('-') {
            proc_macro_error::emit_error!(
                quote! { #plus#lit },
                "unexpected `-` at the start of a positive number";
                hint = "remove the `+` if you want a negative number or the `-` if you want a positive one";
            );
        }

        let value = lit.base10_parse()?;

        Ok(Self {
            raw: (plus, lit),
            value,
        })
    }
}
impl Spanned for Number {
    fn span(&self) -> Span {
        let Self {
            raw: (plus, lit), ..
        } = self;
        (quote! { #plus#lit }).span()
    }
}

pub struct Dimension<U> {
    pub value: Number,
    pub unit: U,
}
impl<U: Parse> Parse for Dimension<U> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let value = input.parse()?;
        let unit = U::parse(input)?;
        Ok(Self { value, unit })
    }
}
impl<U: Spanned> Spanned for Dimension<U> {
    fn span(&self) -> Span {
        let Self { value, unit } = self;
        let v_span = value.span();
        v_span.join(unit.span()).unwrap_or(v_span)
    }
}

pub struct Percentage {
    pub value: Number,
    pub percent: Token![%],
}
impl Parse for Percentage {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let value = input.parse()?;
        let percent = input.parse()?;
        Ok(Self { value, percent })
    }
}
impl Spanned for Percentage {
    fn span(&self) -> Span {
        let Self { value, percent } = self;
        let v_span = value.span();
        v_span.join(percent.span()).unwrap_or(v_span)
    }
}
