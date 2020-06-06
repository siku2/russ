use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Attribute, Lit, LitStr, Token,
};

pub struct FlagArg {
    pub flag: Ident,
}
impl Parse for FlagArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            flag: input.parse()?,
        })
    }
}
impl ToTokens for FlagArg {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self { flag } = self;
        tokens.extend(quote! {#flag})
    }
}

pub struct KwArg {
    pub key: Ident,
    pub equals: Token![=],
    pub value: Lit,
}
impl Parse for KwArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            key: input.parse()?,
            equals: input.parse()?,
            value: input.parse()?,
        })
    }
}
impl ToTokens for KwArg {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self { key, equals, value } = self;
        tokens.extend(quote! {#key#equals#value})
    }
}

pub enum Arg {
    Flag(FlagArg),
    Keyword(KwArg),
}
impl Arg {
    pub fn is_ident<I: ?Sized>(&self, ident: &I) -> bool
    where
        Ident: PartialEq<I>,
    {
        match &self {
            Self::Flag(arg) => &arg.flag == ident,
            Self::Keyword(arg) => &arg.key == ident,
        }
    }
}
impl Parse for Arg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek2(Token![=]) {
            Ok(Self::Keyword(input.parse()?))
        } else {
            Ok(Self::Flag(input.parse()?))
        }
    }
}

pub struct Args(Punctuated<Arg, Token![,]>);
impl Args {
    pub fn new() -> Self {
        Self(Punctuated::new())
    }

    pub fn from_attribute(attr: &Attribute) -> syn::Result<Self> {
        if attr.tokens.is_empty() {
            Ok(Self::new())
        } else {
            attr.parse_args::<Self>()
        }
    }

    pub fn iter(&self) -> syn::punctuated::Iter<Arg> {
        self.0.iter()
    }

    pub fn get_arg<I: ?Sized>(&self, ident: &I) -> Option<&Arg>
    where
        Ident: PartialEq<I>,
    {
        self.iter().find(|arg| arg.is_ident(ident))
    }

    pub fn get_flag<I: ?Sized>(&self, flag: &I) -> Option<&FlagArg>
    where
        Ident: PartialEq<I>,
    {
        self.get_arg(flag).and_then(|arg| match arg {
            Arg::Flag(arg) => Some(arg),
            _ => None,
        })
    }

    pub fn has_flag<I: ?Sized>(&self, flag: &I) -> bool
    where
        Ident: PartialEq<I>,
    {
        self.get_flag(flag).is_some()
    }

    pub fn get_kwarg<I: ?Sized>(&self, key: &I) -> Option<&KwArg>
    where
        Ident: PartialEq<I>,
    {
        self.get_arg(key).and_then(|arg| match arg {
            Arg::Keyword(arg) => Some(arg),
            _ => None,
        })
    }

    pub fn get_kwarg_str<I: ?Sized>(&self, key: &I) -> Option<syn::Result<&LitStr>>
    where
        Ident: PartialEq<I>,
    {
        self.get_kwarg(key).map(|kwarg| match &kwarg.value {
            Lit::Str(v) => Ok(v),
            _ => Err(syn::Error::new_spanned(kwarg, "expected string literal")),
        })
    }
}
impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.call(Punctuated::parse_terminated).map(Self)
    }
}
