use super::{
    combined::CombinedValue,
    multiplier::{Multiplier, MultiplierType},
};
use syn::{
    bracketed,
    ext::IdentExt,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token, Ident, LitInt, LitStr, Token,
};

#[derive(Clone)]
pub struct CSSIdent(Punctuated<Ident, Token![-]>);
impl CSSIdent {
    pub fn value(&self) -> String {
        self.0.pairs().fold(String::new(), |mut s, pair| {
            s.push_str(&pair.value().to_string());
            if matches!(pair.punct(), Some(_)) {
                s.push('-');
            }
            s
        })
    }
}
impl Parse for CSSIdent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(Punctuated::parse_separated_nonempty_with(
            input,
            Ident::parse_any,
        )?))
    }
}

pub type Keyword = CSSIdent;

pub struct AngleBracketed<T> {
    pub lt: Token![<],
    pub content: T,
    pub gt: Token![>],
}
impl<T: Parse> Parse for AngleBracketed<T> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lt = input.parse()?;
        let content = T::parse(input)?;
        let gt = input.parse()?;
        Ok(Self { lt, content, gt })
    }
}

pub struct ClosedRange {
    pub bracket: token::Bracket,
    pub min: Option<LitInt>,
    pub comma: Token![,],
    pub max: Option<LitInt>,
}
impl ClosedRange {
    fn parse_end(input: ParseStream) -> syn::Result<Option<LitInt>> {
        if let Ok(s) = input.parse::<Ident>() {
            return if s.to_string().to_ascii_lowercase() == "inf" {
                Ok(None)
            } else {
                Err(syn::Error::new_spanned(s, "expected number or `inf`"))
            };
        }

        input.parse().map(Some)
    }
}
impl Parse for ClosedRange {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let bracket = bracketed!(content in input);
        let min = Self::parse_end(&content)?;
        let comma = content.parse()?;
        let max = Self::parse_end(&content)?;

        if !content.is_empty() {
            return Err(content.error("unexpected tokens in closed range"));
        }

        Ok(Self {
            bracket,
            min,
            comma,
            max,
        })
    }
}

pub struct InnerReference {
    pub ident: CSSIdent,
    pub range: Option<ClosedRange>,
}
impl Parse for InnerReference {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        let range = if input.peek(token::Bracket) {
            Some(input.parse()?)
        } else {
            None
        };

        Ok(Self { ident, range })
    }
}

pub type Reference = AngleBracketed<InnerReference>;

pub type PropertyReference = AngleBracketed<LitStr>;

pub struct Group {
    pub bracket: token::Bracket,
    pub value: Box<CombinedValue>,
}
impl Parse for Group {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let bracket = bracketed!(content in input);
        let value = Box::new(content.parse()?);
        if !content.is_empty() {
            return Err(content.error("unexpected tokens in group"));
        }

        Ok(Self { bracket, value })
    }
}

pub enum PrimitiveValueType {
    Keyword,
    Literal,
    Reference,
    PropertyReference,
    Group,
}
impl PrimitiveValueType {
    pub fn peek_variant(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![<]) {
            if input.peek2(LitStr) {
                Ok(Self::PropertyReference)
            } else {
                Ok(Self::Reference)
            }
        } else if lookahead.peek(token::Bracket) {
            Ok(Self::Group)
        } else if lookahead.peek(Ident) {
            Ok(Self::Keyword)
        } else if lookahead.peek(LitStr) {
            Ok(Self::Literal)
        } else {
            Err(lookahead.error())
        }
    }
}

pub enum PrimitiveValue {
    Keyword(Keyword),
    Literal(LitStr),
    Reference(Reference),
    PropertyReference(PropertyReference),
    Group(Group),
}
impl Parse for PrimitiveValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        match PrimitiveValueType::peek_variant(input)? {
            PrimitiveValueType::Keyword => input.parse().map(Self::Keyword),
            PrimitiveValueType::Literal => input.parse().map(Self::Literal),
            PrimitiveValueType::Reference => input.parse().map(Self::Reference),
            PrimitiveValueType::PropertyReference => input.parse().map(Self::PropertyReference),
            PrimitiveValueType::Group => input.parse().map(Self::Group),
        }
    }
}

pub struct SingleValue {
    pub value: PrimitiveValue,
    pub multiplier: Option<Multiplier>,
    pub comma: Option<Token![,]>,
}
impl Parse for SingleValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let value = input.parse()?;
        let multiplier = if MultiplierType::peek_variant(input).is_ok() {
            Some(input.parse()?)
        } else {
            None
        };
        let comma = input.parse().ok();
        Ok(Self {
            value,
            multiplier,
            comma,
        })
    }
}
