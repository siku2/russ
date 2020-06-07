mod attributes;

use crate::args;
use attributes::CSSValueAttr;
use heck::KebabCase;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, Data, DeriveInput, Fields, Ident, LitStr};

fn is_fields_single_unnamed(fields: &Fields) -> bool {
    match fields {
        Fields::Unnamed(_) => fields.len() == 1,
        _ => false,
    }
}

fn is_fields_unit(fields: &Fields) -> bool {
    match fields {
        Fields::Unit => true,
        _ => false,
    }
}

fn bind_idents_for_fields(fields: &Fields) -> (TokenStream, Vec<Ident>) {
    match fields {
        Fields::Named(fields) => {
            let idents = fields
                .named
                .iter()
                .map(|field| field.ident.clone().unwrap())
                .collect::<Vec<_>>();
            (quote! { {#(#idents),*} }, idents)
        }
        Fields::Unnamed(fields) => {
            let idents = fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, field)| Ident::new(&format!("v{}", i), field.span()))
                .collect::<Vec<_>>();

            (quote! { (#(#idents),*) }, idents)
        }
        Fields::Unit => (TokenStream::new(), Vec::new()),
    }
}

fn generate_write_for_fields_tokens(
    attr: Option<CSSValueAttr>,
    fields: &Fields,
    container_ident: &Ident,
    idents: &[Ident],
) -> syn::Result<TokenStream> {
    if let Some(attr) = attr {
        Ok(match attr {
            CSSValueAttr::Dimension(dimension) => {
                if dimension.zero {
                    if !is_fields_unit(fields) {
                        return Err(syn::Error::new_spanned(
                            fields,
                            "zero dimension must not have any fields",
                        ));
                    }
                    quote! { f.write_char('0') }
                } else {
                    if !is_fields_single_unnamed(fields) {
                        return Err(syn::Error::new_spanned(
                            fields,
                            "dimension must have a single unnamed field",
                        ));
                    }

                    let unit_str = dimension
                        .unit
                        .as_ref()
                        .map(LitStr::value)
                        .unwrap_or_else(|| container_ident.to_string().to_lowercase());
                    let value_ident = idents.first().unwrap();

                    quote! {
                        {
                            ::russ_css::WriteValue::write_value(#value_ident, f)?;
                            f.write_str(#unit_str)
                        }
                    }
                }
            }
            CSSValueAttr::Keyword(keyword) => {
                if !is_fields_unit(fields) {
                    return Err(syn::Error::new_spanned(
                        fields,
                        "keyword must not have any fields",
                    ));
                }

                let value_str = keyword
                    .value
                    .as_ref()
                    .map(LitStr::value)
                    .unwrap_or_else(|| container_ident.to_string().to_kebab_case());

                quote! { f.write_str(#value_str) }
            }
            CSSValueAttr::Value(value) => {
                if idents.is_empty() {
                    return Err(syn::Error::new_spanned(
                        container_ident,
                        "value must have at least one field",
                    ));
                }

                let separator_str = value
                    .separator
                    .as_ref()
                    .map(LitStr::value)
                    .unwrap_or(String::from(" "));

                quote! {
                    {
                        use ::std::io::Write;

                        let mut __buf = Vec::new();
                        let mut __wrote_first = false;
                        #(
                            __buf.clear();
                            if ::russ_css::MaybeWriteValue::maybe_write_value(#idents, &mut ::russ_css::CSSWriter::new(&mut __buf))? {
                                if __wrote_first {
                                    f.write_str(#separator_str)?;
                                } else {
                                    __wrote_first = true;
                                }

                                f.write_all(&__buf)?;
                            }
                        )*
                        Ok(())
                    }
                }
            }
        })
    } else {
        if !is_fields_single_unnamed(fields) {
            return Err(syn::Error::new_spanned(
                fields,
                "expected a single unnamed field",
            ));
        }

        // check above makes sure we have a single field.
        let value_ident = idents.first().unwrap();
        Ok(quote! { ::russ_css::WriteValue::write_value(#value_ident, f) })
    }
}

fn generate_function_body(input: DeriveInput) -> syn::Result<TokenStream> {
    let name_ident = &input.ident;

    Ok(match input.data {
        Data::Struct(data) => {
            let (bind_tokens, idents) = bind_idents_for_fields(&data.fields);
            let write_tokens = generate_write_for_fields_tokens(
                args::parse_single_from_attrs(&input.attrs).transpose()?,
                &data.fields,
                name_ident,
                &idents,
            )?;

            quote! {
                let Self#bind_tokens = self;
                #write_tokens
            }
        }
        Data::Enum(data) => {
            // make sure there are no attributes on the enum itself.
            args::expect_no_attrs::<CSSValueAttr, _>(&input.attrs)?;

            let arms: Vec<_> = data
                .variants
                .iter()
                .map(|variant| -> syn::Result<TokenStream> {
                    let (bind_tokens, idents) = bind_idents_for_fields(&variant.fields);
                    let variant_ident = &variant.ident;
                    let write_tokens = generate_write_for_fields_tokens(
                        args::parse_single_from_attrs(&variant.attrs).transpose()?,
                        &variant.fields,
                        variant_ident,
                        &idents,
                    )?;

                    Ok(quote! {
                        Self::#variant_ident#bind_tokens => { #write_tokens }
                    })
                })
                .collect::<Result<_, _>>()?;

            quote! {
                match self {
                    #(#arms)*
                }
            }
        }
        Data::Union(data) => {
            return Err(syn::Error::new_spanned(
                data.union_token,
                "union types unsupported",
            ))
        }
    })
}

pub fn generate_write_value(input: DeriveInput) -> syn::Result<TokenStream> {
    let type_ident = input.ident.to_token_stream();
    let body = generate_function_body(input)?;
    Ok(quote! {
        impl ::russ_css::WriteValue for #type_ident {
            fn write_value(&self, f: &mut ::russ_css::CSSWriter) -> ::russ_css::WriteResult {
                #body
            }
        }
    })
}
