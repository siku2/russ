mod value;

use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use std::fmt::{self, Display, Formatter};
use syn::{
    braced,
    ext::IdentExt,
    parse::{self, Parse, ParseStream},
    punctuated::Punctuated,
    token, Ident, Token,
};
use value::Value;

pub struct Property(Punctuated<Ident, Token![-]>);

impl Display for Property {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut it = self.0.iter();
        if let Some(first) = it.next() {
            first.fmt(f)?;
        }

        for extended in it {
            f.write_str("-")?;
            extended.fmt(f)?;
        }

        Ok(())
    }
}

impl Parse for Property {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let mut parts = Punctuated::new();
        parts.push_value(input.call(Ident::parse_any)?);
        while let Ok(punct) = input.parse() {
            parts.push_punct(punct);
            parts.push_value(input.call(Ident::parse_any)?);
        }

        Ok(Self(parts))
    }
}

pub struct Declaration {
    property: Property,
    _colon: Token![:],
    value: Value,
}

impl Display for Declaration {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self {
            property, value, ..
        } = self;
        write!(f, "{}:{}", property, value.into_token_stream().to_string())
    }
}

impl Parse for Declaration {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        Ok(Self {
            property: input.parse()?,
            _colon: input.parse()?,
            value: input.parse()?,
        })
    }
}

pub struct DeclarationBlock {
    brace_token: token::Brace,
    declarations: Punctuated<Declaration, Token![;]>,
}

impl DeclarationBlock {
    fn span(&self) -> Span {
        self.brace_token.span
    }
}

impl Display for DeclarationBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self { declarations, .. } = self;

        write!(f, "{{")?;
        for decl in declarations.iter() {
            decl.fmt(f)?;
            f.write_str(";")?;
        }
        write!(f, "}}")
    }
}

impl Parse for DeclarationBlock {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let content;
        let brace_token = braced!(content in input);
        let declarations = content.parse_terminated(Declaration::parse)?;
        Ok(Self {
            brace_token,
            declarations,
        })
    }
}

pub struct RuleSet {
    name: Ident,
    block: DeclarationBlock,
}

impl Parse for RuleSet {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            block: input.parse()?,
        })
    }
}

impl ToTokens for RuleSet {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { name, block } = self;

        let name_str = {
            let s = name.to_string();
            quote_spanned! {name.span()=> #s}
        };
        let block_str = {
            let s = block.to_string();
            quote_spanned! {block.span()=> #s}
        };

        tokens.extend(quote! {
            ::russ::RuleSet::_new_from_macro(#name_str, #block_str)
        });
    }
}

pub struct Styles {
    rule_sets: Vec<RuleSet>,
}

impl Parse for Styles {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let mut rule_sets = Vec::new();
        while !input.cursor().eof() {
            rule_sets.push(input.parse()?);
        }

        Ok(Self { rule_sets })
    }
}

impl ToTokens for Styles {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { rule_sets } = self;

        let decl_iter = rule_sets.iter().map(|rule_set| {
            let RuleSet { name, .. } = rule_set;
            quote! {#name: &'static str}
        });

        let inst_iter = rule_sets.iter().map(|rule_set| {
            let RuleSet { name, .. } = rule_set;
            quote! {#name: ::std::stringify!(#name)}
        });

        let css = rule_sets.first().unwrap().to_token_stream();

        tokens.extend(quote! {
            {
                const __css: ::russ::RuleSet = #css;

                struct Classes {#(#decl_iter),*}
                Classes {#(#inst_iter),*}
            }
        });
    }
}
