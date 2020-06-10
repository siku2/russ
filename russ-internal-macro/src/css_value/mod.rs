mod attributes;
mod css_field;

use crate::args;
use attributes::CSSValueAttr;
use css_field::CSSField;
use heck::KebabCase;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, Data, DeriveInput, ExprPath, Fields, Ident, LitStr};

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

fn bind_idents_for_fields(fields: &Fields) -> syn::Result<(TokenStream, Vec<CSSField>)> {
    Ok(match fields {
        Fields::Named(fields) => {
            let mut idents = Vec::new();
            let mut css_fields = Vec::new();

            for field in &fields.named {
                let ident = field.ident.clone().unwrap();
                idents.push(ident.to_token_stream());
                css_fields.push(CSSField::from_field(ident, field)?);
            }
            (quote! { {#(#idents),*} }, css_fields)
        }
        Fields::Unnamed(fields) => {
            let mut idents = Vec::new();
            let mut css_fields = Vec::new();

            for (i, field) in fields.unnamed.iter().enumerate() {
                let ident = Ident::new(&format!("v{}", i), field.span());
                idents.push(ident.to_token_stream());
                css_fields.push(CSSField::from_field(ident, field)?);
            }

            (quote! { (#(#idents),*) }, css_fields)
        }
        Fields::Unit => (TokenStream::new(), Vec::new()),
    })
}

fn generate_write_for_fields_tokens(
    attr: Option<CSSValueAttr>,
    fields: &Fields,
    container_ident: &Ident,
    css_fields: &[CSSField],
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
                    let write_value = css_fields.first().unwrap().gen_write()?;

                    quote! {
                        #write_value
                        f.write_str(#unit_str)
                    }
                }
            }
            CSSValueAttr::Function(function) => {
                let separator_str = function
                    .separator
                    .as_ref()
                    .map(LitStr::value)
                    .unwrap_or(String::from(","));
                let write_arguments = css_field::gen_join_fields(css_fields, &separator_str)?;
                let fn_name_str = function
                    .name
                    .as_ref()
                    .map(LitStr::value)
                    .unwrap_or_else(|| container_ident.to_string().to_kebab_case());

                quote! {
                    f.write_str(#fn_name_str)?;
                    f.write_char('(')?;
                    #write_arguments
                    f.write_char(')')?;
                    Ok(())
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
                let write_prefix = value.prefix.map(|value| quote! { f.write_str(#value)?; });
                let write_suffix = value.suffix.map(|value| quote! { f.write_str(#value)?; });

                let write_value = if let Some(write_fn) = value.write_fn {
                    let fn_path = syn::parse_str::<ExprPath>(&write_fn.value())?;
                    let idents = css_fields
                        .iter()
                        .map(|field| &field.bind_ident)
                        .collect::<Vec<_>>();
                    quote! {
                        #fn_path(f, #(#idents),*)?;
                    }
                } else {
                    if css_fields.is_empty() {
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

                    css_field::gen_join_fields(css_fields, &separator_str)?
                };

                quote! {
                    #write_prefix
                    #write_value
                    #write_suffix
                    Ok(())
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
        let write_tokens = css_fields.first().unwrap().gen_write()?;
        Ok(quote! {
            #write_tokens
            Ok(())
        })
    }
}

fn generate_function_body(input: DeriveInput) -> syn::Result<TokenStream> {
    let name_ident = &input.ident;

    Ok(match input.data {
        Data::Struct(data) => {
            let (bind_tokens, idents) = bind_idents_for_fields(&data.fields)?;
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
                    let (bind_tokens, idents) = bind_idents_for_fields(&variant.fields)?;
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
        impl ::russ_internal::WriteValue for #type_ident {
            fn write_value(&self, f: &mut ::russ_internal::CSSWriter) -> ::russ_internal::WriteResult {
                #body
            }
        }
    })
}