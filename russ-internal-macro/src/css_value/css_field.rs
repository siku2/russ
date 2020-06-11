use crate::args::{self, Args, FromArgs};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::{Attribute, ExprPath, Field, Ident, LitStr, Path, Type};

pub struct FieldAttr {
    attr: Attribute,
    pub write_fn: Option<LitStr>,
    pub option: bool,
    pub iter_option: bool,
    pub iter_separator: Option<LitStr>,
}
impl FromArgs for FieldAttr {
    fn attr_path() -> &'static str {
        "field"
    }

    fn from_args(attr: Attribute, args: &Args) -> syn::Result<Self> {
        Ok(Self {
            attr,
            write_fn: args.get_kwarg_str("write_fn").transpose()?.cloned(),
            option: args.has_flag("option"),
            iter_option: args.has_flag("iter_option"),
            iter_separator: args.get_kwarg_str("iter_separator").transpose()?.cloned(),
        })
    }
}
impl ToTokens for FieldAttr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.attr.to_tokens(tokens)
    }
}

fn path_is_option(path: &Path) -> bool {
    if let Some(last) = path.segments.last() {
        last.ident == "Option"
    } else {
        false
    }
}

fn type_is_option(ty: &Type) -> bool {
    match ty {
        Type::Path(ty) => path_is_option(&ty.path),
        _ => false,
    }
}

// TODO handle empty vectors

pub struct CSSField {
    pub bind_ident: Ident,
    pub attr: Option<FieldAttr>,
    assume_option: bool,
    iter_separator: String,
}
impl CSSField {
    pub fn from_field(bind_ident: Ident, field: &Field) -> syn::Result<Self> {
        let assume_option;
        let mut iter_separator = String::from(",");

        let attr: Option<FieldAttr> = args::parse_single_from_attrs(&field.attrs).transpose()?;
        if let Some(attr) = &attr {
            assume_option = attr.option;
            if let Some(sep) = &attr.iter_separator {
                iter_separator = sep.value();
            }
        } else {
            assume_option = type_is_option(&field.ty);
        }

        Ok(Self {
            bind_ident,
            attr,
            assume_option,
            iter_separator,
        })
    }

    fn gen_write_value(&self, value_ident: &Ident) -> syn::Result<TokenStream> {
        let Self {
            bind_ident,
            attr,
            iter_separator,
            ..
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

        if matches!(
            attr,
            Some(FieldAttr {
                iter_option: true, ..
            })
        ) {
            let write_separator = quote! {
                use ::std::io::Write;
                f.write_str(#iter_separator)?;
            };

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

    fn gen_write_with_before_write(&self, tokens: TokenStream) -> syn::Result<TokenStream> {
        let Self {
            bind_ident,
            assume_option,
            ..
        } = self;

        if *assume_option {
            let ident = Ident::new("__v", bind_ident.span());
            let write_value = self.gen_write_value(&ident)?;
            Ok(quote_spanned! {bind_ident.span()=>
                if let ::std::option::Option::Some(#ident) = #bind_ident {
                    #tokens
                    #write_value
                }
            })
        } else {
            let write_value = self.gen_write_value(bind_ident)?;
            Ok(quote_spanned! {bind_ident.span()=>
                #tokens
                #write_value
            })
        }
    }

    pub fn gen_write(&self) -> syn::Result<TokenStream> {
        self.gen_write_with_before_write(quote! {})
    }
}

pub fn gen_join_fields(fields: &[CSSField], separator: &str) -> syn::Result<TokenStream> {
    gen_join_fields_with_write_separator(
        fields,
        quote! {
            use ::std::io::Write;
            f.write_str(#separator)?;
        },
    )
}

pub fn gen_join_fields_with_write_separator(
    fields: &[CSSField],
    write_separator: impl ToTokens,
) -> syn::Result<TokenStream> {
    if fields.iter().all(|field| !field.assume_option) {
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
        }
    };
    let write_values = fields
        .iter()
        .map(|field| field.gen_write_with_before_write(before_write.clone()))
        .collect::<Result<TokenStream, _>>()?;

    Ok(quote! {
        let mut __wrote_first = false;
        #write_values
    })
}
