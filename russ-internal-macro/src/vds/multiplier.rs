use super::generate::TypeInfo;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_quote,
    spanned::Spanned,
    token, Expr, Ident, LitInt, Token, Type,
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
impl Optional {}
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
impl Range {
    pub fn is_exact(&self) -> bool {
        self.comma.is_none()
    }

    pub fn get_range(&self) -> syn::Result<(usize, Option<usize>)> {
        if self.is_exact() {
            let n = self.min.base10_parse()?;
            return Ok((n, Some(n)));
        }

        let min = self.min.base10_parse()?;
        let max = self.max.as_ref().map(LitInt::base10_parse).transpose()?;
        Ok((min, max))
    }

    /// Generates a type representing the range.
    /// This will be a vector for infinite ranges and a tuple for finite ones.
    pub fn wrap_type(&self, ty: &Type) -> syn::Result<Type> {
        let (lo, hi) = self.get_range()?;
        if let Some(hi) = hi {
            let required_fields = (0..lo).map(|_| ty.clone());
            let optional_fields = (lo..hi).map(|_| parse_quote! { ::std::option::Option<#ty> });
            let fields = required_fields.chain(optional_fields);

            Ok(parse_quote! { (#(#fields),*) })
        } else {
            Ok(parse_quote! { ::std::vec::Vec<#ty> })
        }
    }

    fn gen_parse_finite_range(
        &self,
        lo: usize,
        hi: usize,
        value_ty: &Type,
        parse: &Expr,
        can_continue: &Expr,
    ) -> syn::Result<Expr> {
        if lo > hi {
            return Err(syn::Error::new(
                self.max.span(),
                "upper bound must be higher than lower bound",
            ));
        }

        let required_idents: Vec<_> = (0..lo)
            .map(|i| Ident::new(&format!("__v{}", i), value_ty.span()))
            .collect();
        let handle_required_it = required_idents.iter().map(|ident| -> Expr {
            parse_quote! {
                let #ident: #value_ty = #parse?;
            }
        });

        let optional_idents: Vec<_> = (lo..hi)
            .map(|i| Ident::new(&format!("__v{}_opt", i), value_ty.span()))
            .collect();
        let handle_optional_it = required_idents
            .iter()
            .enumerate()
            .map(|(i, ident)| -> Expr {
                let prev_ident = i
                    .checked_sub(1)
                    .and_then(|prev_index| optional_idents.get(prev_index));
                let continue_check = if let Some(prev_ident) = prev_ident {
                    parse_quote! { #prev_ident.is_some() && #can_continue }
                } else {
                    can_continue.clone()
                };
                parse_quote! {
                    let #ident: ::std::option::Option<#value_ty> = if #continue_check {
                        ::std::option::Option::Some(#parse?)
                    } else {
                        None
                    };
                }
            });

        let all_idents_it = required_idents.iter().chain(optional_idents.iter());
        Ok(parse_quote! {
            (|| {
                #(#handle_required_it)*
                #(#handle_optional_it)*

                ::std::result::Result::Ok((#(#all_idents_it),*))
            })()
        })
    }

    fn gen_parse_infinite_range(
        &self,
        lo: usize,
        value_ty: &Type,
        parse: &Expr,
        can_continue: &Expr,
    ) -> syn::Result<Expr> {
        let ty = self.wrap_type(value_ty)?;
        let parse_required_it = (0..lo).map(|_| -> Expr {
            parse_quote! { #parse? }
        });
        Ok(parse_quote! {
            (|| {
                let mut __v: #ty = ::std::vec![
                    #(#parse_required_it),*
                ];
                while #can_continue {
                    __v.push(#parse?);
                }

                ::std::result::Result::Ok(__v)
            })()
        })
    }

    /// Generates an expression returning `syn::Result<T>` where T is the type returned by `gen_type`.
    /// `parse` is an expression that should return `syn::Result<T>` where T is `value_ty`.
    pub fn gen_parse(
        &self,
        value_ty: &Type,
        parse: &Expr,
        can_continue: &Expr,
    ) -> syn::Result<Expr> {
        let (lo, hi) = self.get_range()?;
        if lo == 0 {
            return Err(syn::Error::new(
                self.min.span(),
                "lower bound must not be 0",
            ));
        }

        if let Some(hi) = hi {
            self.gen_parse_finite_range(lo, hi, value_ty, parse, can_continue)
        } else {
            self.gen_parse_infinite_range(lo, value_ty, parse, can_continue)
        }
    }
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
impl OneOrMoreComma {}
impl Parse for OneOrMoreComma {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let hash = input.parse()?;
        Ok(Self { hash })
    }
}

pub struct Required {
    pub exclamation: Token![!],
}
impl Required {}
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
impl Multiplier {
    pub fn modify_type_info(&self, mut info: TypeInfo) -> syn::Result<TypeInfo> {
        let ty = info.value_type;
        info.value_type = match self {
            Self::ZeroOrMore(_) | Self::OneOrMore(_) | Self::OneOrMoreComma(_) => {
                parse_quote! { ::std::vec::Vec<#ty> }
            }
            Self::Optional(_) => parse_quote! { ::std::option::Option<#ty> },
            Self::Range(value) => value.wrap_type(&ty)?,
            Self::Required(_) => ty,
        };
        Ok(info)
    }
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
