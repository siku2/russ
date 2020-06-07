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

pub trait ParseAttr
where
    Self: Sized,
{
    fn parse_attr(attr: &Attribute) -> Option<syn::Result<Self>>;
}

pub trait FromArgs
where
    Self: Sized,
{
    fn attr_path() -> &'static str;
    fn from_args(attr: Attribute, args: &Args) -> syn::Result<Self>;
}
impl<T> ParseAttr for T
where
    T: FromArgs,
{
    fn parse_attr(attr: &Attribute) -> Option<syn::Result<Self>> {
        if attr.path.is_ident(Self::attr_path()) {
            Some(Args::from_attribute(attr).and_then(|args| Self::from_args(attr.clone(), &args)))
        } else {
            None
        }
    }
}

pub fn parse_first_from_attrs<'a, T, IT>(attrs: IT) -> Option<syn::Result<T>>
where
    T: ParseAttr,
    IT: IntoIterator<Item = &'a Attribute>,
{
    attrs.into_iter().flat_map(T::parse_attr).next()
}

pub fn parse_single_from_attrs<'a, T, IT>(attrs: IT) -> Option<syn::Result<T>>
where
    T: ParseAttr + ToTokens,
    IT: IntoIterator<Item = &'a Attribute>,
{
    let mut attrs_iter = attrs.into_iter();
    let first = match parse_first_from_attrs(&mut attrs_iter) {
        Some(Ok(v)) => v,
        v => return v,
    };

    // check if there's another attribute
    match parse_first_from_attrs::<T, _>(&mut attrs_iter) {
        None => {}
        Some(Ok(attr)) => {
            return Some(Err(syn::Error::new_spanned(
                attr,
                "must only specify a single attribute",
            )))
        }
        Some(Err(err)) => return Some(Err(err)),
    }

    Some(Ok(first))
}

pub fn expect_no_attrs<'a, T, IT>(attrs: IT) -> syn::Result<()>
where
    T: ParseAttr + ToTokens,
    IT: IntoIterator<Item = &'a Attribute>,
{
    match parse_first_from_attrs::<T, IT>(attrs) {
        Some(Ok(attr)) => Err(syn::Error::new_spanned(attr, "attribute not allowed here")),
        Some(Err(err)) => Err(err),
        None => Ok(()),
    }
}
