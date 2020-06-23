use syn::{
    braced,
    parse::{Parse, ParseStream},
    token, LitInt, Token,
};

pub struct ZeroOrMore {
    pub asteriks: Token![*],
}
impl Parse for ZeroOrMore {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let asteriks = input.parse()?;
        Ok(Self { asteriks })
    }
}

pub struct OneOrMore {
    pub plus: Token![+],
}
impl Parse for OneOrMore {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let plus = input.parse()?;
        Ok(Self { plus })
    }
}

pub struct Optional {
    pub question_mark: Token![?],
}
impl Parse for Optional {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let question_mark = input.parse()?;
        Ok(Self { question_mark })
    }
}

pub struct Range {
    pub brace: token::Brace,
    pub min: LitInt,
    pub comma: Option<Token![,]>,
    pub max: Option<LitInt>,
}
impl Parse for Range {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let brace = braced!(content in input);
        let min = content.parse()?;
        let (comma, max) = if content.is_empty() {
            (None, None)
        } else {
            let comma = Some(content.parse()?);
            let max = if content.is_empty() {
                None
            } else {
                Some(content.parse()?)
            };
            (comma, max)
        };

        Ok(Self {
            brace,
            min,
            comma,
            max,
        })
    }
}

pub struct OneOrMoreComma {
    pub hash: Token![#],
}
impl Parse for OneOrMoreComma {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let hash = input.parse()?;
        Ok(Self { hash })
    }
}

pub struct Required {
    pub exclamation: Token![!],
}
impl Parse for Required {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let exclamation = input.parse()?;
        Ok(Self { exclamation })
    }
}

pub enum MultiplierType {
    ZeroOrMore,
    OneOrMore,
    Optional,
    Range,
    OneOrMoreComma,
    Required,
}
impl MultiplierType {
    pub fn peek_variant(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(token::Brace) {
            Ok(Self::Range)
        } else if lookahead.peek(Token![*]) {
            Ok(Self::ZeroOrMore)
        } else if lookahead.peek(Token![+]) {
            Ok(Self::OneOrMore)
        } else if lookahead.peek(Token![?]) {
            Ok(Self::Optional)
        } else if lookahead.peek(Token![#]) {
            Ok(Self::OneOrMoreComma)
        } else if lookahead.peek(Token![!]) {
            Ok(Self::Required)
        } else {
            Err(lookahead.error())
        }
    }
}

pub enum Multiplier {
    ZeroOrMore(ZeroOrMore),
    OneOrMore(OneOrMore),
    Optional(Optional),
    Range(Range),
    OneOrMoreComma(OneOrMoreComma),
    Required(Required),
}
impl Parse for Multiplier {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        match MultiplierType::peek_variant(input)? {
            MultiplierType::ZeroOrMore => input.parse().map(Self::ZeroOrMore),
            MultiplierType::OneOrMore => input.parse().map(Self::OneOrMore),
            MultiplierType::Optional => input.parse().map(Self::Optional),
            MultiplierType::Range => input.parse().map(Self::Range),
            MultiplierType::OneOrMoreComma => input.parse().map(Self::OneOrMoreComma),
            MultiplierType::Required => input.parse().map(Self::Required),
        }
    }
}
