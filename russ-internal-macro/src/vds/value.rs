use super::{
    combined::CombinedValue,
    generate::{GenerateTypeContext, GenerateTypeInfo, TypeDefinition, TypeInfo},
    helpers,
    multiplier::{Multiplier, MultiplierType},
};
use heck::CamelCase;
use proc_macro2::{Span, TokenStream};
use std::{
    fmt::{self, Debug, Formatter},
    str::FromStr,
};
use syn::{
    bracketed,
    ext::IdentExt,
    parse::{Parse, ParseStream},
    parse_quote,
    punctuated::Punctuated,
    spanned::Spanned,
    token, Expr, Ident, LitInt, LitStr, Token,
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

    pub fn ident_camel_case(&self) -> syn::Result<Ident> {
        let name = self.value().to_camel_case();
        helpers::parse_ident_with_span(&name, self.0.span())
    }

    pub fn gen_parse_any_ident(get_parse_stream: Expr) -> Expr {
        helpers::gen_parse_type(
            parse_quote! {  ::russ_internal_macro::vds::CSSIdent },
            get_parse_stream,
        )
    }

    pub fn gen_parse(&self, get_parse_stream: Expr) -> Expr {
        let parse_ident = Self::gen_parse_any_ident(get_parse_stream);
        let value = self.value();
        let err_msg = format!("expected `{}`", value);
        parse_quote! {
            (#parse_ident).and_then(|__ident| {
                if __ident.value() == #value {
                    ::std::result::Result::Ok(__ident)
                } else {
                    ::std::result::Result::Err(::syn::Error::new_spanned(::syn::spanned::Spanned::span(&__ident), #err_msg))
                }
            })
        }
    }
}
impl Debug for CSSIdent {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("CSSIdent")
            .field("value", &self.value())
            .finish()
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
impl Spanned for CSSIdent {
    fn span(&self) -> Span {
        self.0.span()
    }
}

pub struct Keyword(pub CSSIdent);
impl GenerateTypeInfo for Keyword {
    fn gen_type_info(&self, ctx: &GenerateTypeContext) -> syn::Result<TypeInfo> {
        let name = self.0.value().to_camel_case();
        let ident = ctx.propose_ident(&name)?;
        let definition = parse_quote! { pub struct #ident; };
        let ty = parse_quote! { #ident };
        Ok(TypeInfo::new(ty).with_definition(TypeDefinition::new(ident, definition)))
    }
}
impl Parse for Keyword {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse().map(Self)
    }
}

pub struct Literal(pub LitStr);
impl Literal {
    pub fn gen_parse(&self, get_parse_stream: Expr) -> syn::Result<Expr> {
        let value = TokenStream::from_str(&self.0.value())?;
        // TODO handle non-punctuation
        Ok(helpers::gen_parse_type(
            parse_quote! { ::syn::Token![#value] },
            get_parse_stream,
        ))
    }
}
impl GenerateTypeInfo for Literal {
    fn gen_type_info(&self, _ctx: &GenerateTypeContext) -> syn::Result<TypeInfo> {
        //! FIXME this isn't even nearly the correct type
        Ok(TypeInfo::new(parse_quote! { ::syn::LitStr }))
    }
}
impl Parse for Literal {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse().map(Self)
    }
}

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
    fn format_range(&self) -> String {
        let min = self
            .min
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_else(|| "-inf".to_string());
        let max = self
            .max
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_else(|| "+inf".to_string());
        format!("[{},{}]", min, max)
    }

    /// Generates an expression that returns `syn::Result<()>`.
    pub fn gen_range_check(&self, value_ident: Ident) -> Expr {
        let Self { min, max, .. } = self;
        let err_msg = format!("value must be in closed range {}", self.format_range());

        let result_ok: Expr = parse_quote! { ::std::result::Result::Ok(()) };
        let low_check = min.as_ref().map(|min| {
            parse_quote! {
                if #value_ident < #min {
                    ::std::result::Result::Err(::syn::Error::new_spanned(::syn::spanned::Spanned::span(&#value_ident), #err_msg))
                } else {
                    #result_ok
                }
            }
        }).unwrap_or(result_ok.clone());
        let high_check = max.as_ref().map(|max| {
            parse_quote! {
                if #value_ident > #max {
                    ::std::result::Result::Err(::syn::Error::new(::syn::spanned::Spanned::span(&#value_ident), #err_msg))
                } else {
                    #result_ok
                }
            }
        }).unwrap_or(result_ok);

        parse_quote! {
            (#low_check).and_then(|| #high_check)
        }
    }

    fn parse_bound(input: ParseStream) -> syn::Result<Option<LitInt>> {
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
        let min = Self::parse_bound(&content)?;
        let comma = content.parse()?;
        let max = Self::parse_bound(&content)?;

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

pub struct Reference(pub AngleBracketed<InnerReference>);
impl Reference {
    pub fn ref_ident_raw(&self) -> &CSSIdent {
        &self.0.content.ident
    }
    pub fn ref_ident(&self) -> syn::Result<Ident> {
        self.ref_ident_raw().ident_camel_case()
    }
}
impl GenerateTypeInfo for Reference {
    fn gen_type_info(&self, _ctx: &GenerateTypeContext) -> syn::Result<TypeInfo> {
        let ident = self.ref_ident()?;
        Ok(TypeInfo::new(parse_quote! { #ident }))
    }
}
impl Parse for Reference {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse().map(Self)
    }
}

pub struct PropertyReference(pub AngleBracketed<LitStr>);
impl PropertyReference {
    pub fn prop_lit(&self) -> &LitStr {
        &self.0.content
    }

    pub fn prop_ident(&self) -> syn::Result<Ident> {
        let lit = self.prop_lit();
        helpers::parse_ident_with_span(&lit.value().to_camel_case(), lit.span())
    }
}
impl GenerateTypeInfo for PropertyReference {
    fn gen_type_info(&self, _ctx: &GenerateTypeContext) -> syn::Result<TypeInfo> {
        let ident = self.prop_ident()?;
        Ok(TypeInfo::new(parse_quote! { #ident }))
    }
}
impl Parse for PropertyReference {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse().map(Self)
    }
}

pub struct Group {
    pub bracket: token::Bracket,
    pub value: Box<CombinedValue>,
}
impl GenerateTypeInfo for Group {
    fn gen_type_info(&self, ctx: &GenerateTypeContext) -> syn::Result<TypeInfo> {
        self.value.gen_type_info(ctx)
    }
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
    Literal(Literal),
    Reference(Reference),
    PropertyReference(PropertyReference),
    Group(Group),
}
impl GenerateTypeInfo for PrimitiveValue {
    fn gen_type_info(&self, ctx: &GenerateTypeContext) -> syn::Result<TypeInfo> {
        match self {
            Self::Keyword(value) => value.gen_type_info(ctx),
            Self::Literal(value) => value.gen_type_info(ctx),
            Self::Reference(value) => value.gen_type_info(ctx),
            Self::PropertyReference(value) => value.gen_type_info(ctx),
            Self::Group(value) => value.gen_type_info(ctx),
        }
    }
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
impl GenerateTypeInfo for SingleValue {
    fn gen_type_info(&self, ctx: &GenerateTypeContext) -> syn::Result<TypeInfo> {
        let info = self.value.gen_type_info(ctx)?;
        if let Some(multiplier) = &self.multiplier {
            multiplier.modify_type_info(info)
        } else {
            Ok(info)
        }
    }
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
