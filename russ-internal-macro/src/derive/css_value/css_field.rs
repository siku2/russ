use super::args::{self, Args, FromArgs};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::{Attribute, ExprPath, Field, Ident, LitStr, Path, Type};

pub struct FieldAttr {
    attr: Attribute,
    pub prefix: Option<LitStr>,
    pub suffix: Option<LitStr>,
    pub write_fn: Option<LitStr>,
    pub option: bool,
    pub iter: bool,
    pub iter_option: bool,
    pub iter_separator: Option<LitStr>,
}
impl FieldAttr {
    /// Assumes `io::Write` is in scope.
    fn gen_write_str(s: &str) -> TokenStream {
        quote! {
            f.write_str(#s)?;
        }
    }

    fn gen_write_separator(attr: &Option<Self>) -> TokenStream {
        let iter_separator = attr
            .as_ref()
            .and_then(|attr| attr.iter_separator.as_ref())
            .map_or_else(|| ",".to_string(), LitStr::value);
        Self::gen_write_str(&iter_separator)
    }

    fn gen_write_prefix(attr: &Option<Self>) -> Option<TokenStream> {
        attr.as_ref()
            .and_then(|attr| attr.prefix.as_ref())
            .map(|v| Self::gen_write_str(&v.value()))
    }

    fn gen_write_suffix(attr: &Option<Self>) -> Option<TokenStream> {
        attr.as_ref()
            .and_then(|attr| attr.suffix.as_ref())
            .map(|v| Self::gen_write_str(&v.value()))
    }
}
impl FromArgs for FieldAttr {
    fn attr_path() -> &'static str {
        "field"
    }

    fn from_args(attr: Attribute, args: &Args) -> syn::Result<Self> {
        Ok(Self {
            attr,
            prefix: args.get_kwarg_str("prefix")?,
            suffix: args.get_kwarg_str("suffix")?,
            write_fn: args.get_kwarg_str("write_fn")?,
            option: args.has_flag("option"),
            iter: args.has_flag("iter"),
            iter_option: args.has_flag("iter_option"),
            iter_separator: args.get_kwarg_str("iter_separator")?,
        })
    }
}
impl ToTokens for FieldAttr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.attr.to_tokens(tokens)
    }
}

fn path_is_eq_to(path: &Path, name: &str) -> bool {
    if let Some(last) = path.segments.last() {
        last.ident == name
    } else {
        false
    }
}

fn type_is_eq_to(ty: &Type, name: &str) -> bool {
    match ty {
        Type::Path(ty) => path_is_eq_to(&ty.path, name),
        _ => false,
    }
}

// TODO empty vectors can cause problems

pub struct CssField {
    pub bind_ident: Ident,
    pub attr: Option<FieldAttr>,
    is_option: bool,
    is_iter: bool,
}
impl CssField {
    pub fn from_field(bind_ident: Ident, field: &Field) -> syn::Result<Self> {
        let is_option;
        let is_iter;

        let attr: Option<FieldAttr> = args::parse_single_from_attrs(&field.attrs).transpose()?;
        if let Some(attr) = &attr {
            is_option = attr.option;
            is_iter = attr.iter;
        } else {
            is_option = type_is_eq_to(&field.ty, "Option");
            is_iter = type_is_eq_to(&field.ty, "Vec");
        }

        Ok(Self {
            bind_ident,
            attr,
            is_option,
            is_iter,
        })
    }

    /// Assumes `io::Write` is in scope.
    fn _gen_write_inner_value(&self, value_ident: &Ident) -> syn::Result<TokenStream> {
        let Self {
            bind_ident, attr, ..
        } = self;

        if let Some(FieldAttr {
            write_fn: Some(write_fn),
            ..
        }) = attr
        {
            let fn_path: ExprPath = write_fn.parse()?;
            return Ok(quote! {
                #fn_path(f, #value_ident)?;
            });
        }
        if self.is_iter {
            let write_separator = FieldAttr::gen_write_separator(attr);

            Ok(quote_spanned! {bind_ident.span()=>
                let mut __v_iter = ::std::iter::IntoIterator::into_iter(#value_ident);
                if let ::std::option::Option::Some(__v) = __v_iter.next() {
                    ::russ_internal::WriteValue::write_value(__v, f)?;
                }
                for __v in __v_iter {
                    #write_separator
                    ::russ_internal::WriteValue::write_value(__v, f)?;
                }
            })
        } else if matches!(
            attr,
            Some(FieldAttr {
                iter_option: true, ..
            })
        ) {
            let write_separator = FieldAttr::gen_write_separator(attr);

            Ok(quote_spanned! {bind_ident.span()=>
                let mut __v_iter = ::std::iter::IntoIterator::into_iter(#value_ident);
                for __maybe_v in &mut __v_iter {
                    if let ::std::option::Option::Some(__v) = __maybe_v {
                        ::russ_internal::WriteValue::write_value(__v, f)?;
                        break;
                    }
                }

                for __maybe_v in __v_iter {
                    if let ::std::option::Option::Some(__v) = __maybe_v {
                        #write_separator
                        ::russ_internal::WriteValue::write_value(__v, f)?;
                    }
                }
            })
        } else {
            Ok(quote_spanned! {bind_ident.span()=>
                ::russ_internal::WriteValue::write_value(#value_ident, f)?;
            })
        }
    }

    /// Assumes `io::Write` is in scope.
    fn gen_write_value(&self, value_ident: &Ident) -> syn::Result<TokenStream> {
        let write_value = self._gen_write_inner_value(value_ident)?;
        let write_prefix = FieldAttr::gen_write_prefix(&self.attr);
        let write_suffix = FieldAttr::gen_write_suffix(&self.attr);

        Ok(quote! {
            #write_prefix
            #write_value
            #write_suffix
        })
    }

    /// Assumes `io::Write` is in scope.
    fn gen_write_with_before_write(&self, tokens: &TokenStream) -> syn::Result<TokenStream> {
        let Self {
            bind_ident,
            is_option,
            is_iter,
            ..
        } = self;

        assert!(
            !(*is_option && *is_iter),
            "can't be `is_iter` and `option` at the same time"
        );

        if *is_option {
            let ident = Ident::new("__v", bind_ident.span());
            let write_value = self.gen_write_value(&ident)?;
            // using a semicolon at the end of the if statement to suppress `clippy::suspicious_else_formatting`
            Ok(quote_spanned! {bind_ident.span()=>
                if let ::std::option::Option::Some(#ident) = #bind_ident {
                    #tokens
                    #write_value
                };
            })
        } else if *is_iter {
            let write_value = self.gen_write_value(bind_ident)?;
            Ok(quote_spanned! {bind_ident.span()=>
                if !(#bind_ident).is_empty() {
                    #tokens
                    #write_value
                };
            })
        } else {
            let write_value = self.gen_write_value(bind_ident)?;
            Ok(quote_spanned! {bind_ident.span()=>
                #tokens
                #write_value
            })
        }
    }

    /// Assumes `io::Write` is in scope.
    pub fn gen_write(&self) -> syn::Result<TokenStream> {
        self.gen_write_with_before_write(&quote! {})
    }
}

/// Assumes `io::Write` is in scope.
pub fn gen_join_fields(fields: &[CssField], separator: &str) -> syn::Result<TokenStream> {
    gen_join_fields_with_write_separator(
        fields,
        quote! {
            f.write_str(#separator)?;
        },
    )
}

/// Assumes `io::Write` is in scope.
pub fn gen_join_fields_with_write_separator(
    fields: &[CssField],
    write_separator: impl ToTokens,
) -> syn::Result<TokenStream> {
    if fields.iter().all(|field| !field.is_option) {
        let write_value_vec = fields
            .iter()
            .map(|field| field.gen_write_value(&field.bind_ident))
            .collect::<Result<Vec<_>, _>>()?;
        let tokens = if let Some((first, others)) = write_value_vec.split_first() {
            quote! {
                #first
                #(
                    #write_separator
                    #others
                )*
            }
        } else {
            quote! {}
        };

        return Ok(tokens);
    }

    let before_write = quote! {
        if __wrote_first {
            #write_separator
        } else {
            __wrote_first = true;
        };
    };
    let write_values = fields
        .iter()
        .map(|field| field.gen_write_with_before_write(&before_write))
        .collect::<Result<TokenStream, _>>()?;

    Ok(quote! {
        let mut __wrote_first = false;
        #write_values
    })
}
