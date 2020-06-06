mod args;
mod attrs;

use attrs::Attr;
use heck::KebabCase;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Fields, Ident, LitStr};

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

fn bind_idents_for_fields(fields: &Fields) -> (proc_macro2::TokenStream, Vec<Ident>) {
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
        Fields::Unit => (proc_macro2::TokenStream::new(), Vec::new()),
    }
}

fn generate_write_for_fields_tokens(
    attr: &Attr,
    fields: &Fields,
    container_ident: &Ident,
    values: &[Ident],
) -> syn::Result<proc_macro2::TokenStream> {
    let write_values_tokens = quote! { #( ::russ_css::WriteValue::write_value(#values, f)?; )* };

    Ok(match attr {
        Attr::Dimension(dimension) => {
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

                quote! {
                    {
                        #write_values_tokens
                        f.write_str(#unit_str)
                    }
                }
            }
        }
        Attr::Keyword(keyword) => {
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
        Attr::None => {
            if !is_fields_single_unnamed(fields) {
                return Err(syn::Error::new_spanned(
                    fields,
                    "expected a single unnamed field",
                ));
            }

            quote! {
                {
                    #write_values_tokens
                    Ok(())
                }
            }
        }
    })
}

fn generate_function_body(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name_ident = &input.ident;

    Ok(match &input.data {
        Data::Struct(data) => {
            let (bind_tokens, idents) = bind_idents_for_fields(&data.fields);
            // TODO handle error
            let write_tokens = generate_write_for_fields_tokens(
                &Attr::single_from_attrs(&input.attrs).unwrap(),
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
            Attr::expect_no_attrs(&input.attrs)?;

            let arms: Vec<_> = data
                .variants
                .iter()
                .map(|variant| -> syn::Result<proc_macro2::TokenStream> {
                    let (bind_tokens, idents) = bind_idents_for_fields(&variant.fields);
                    let variant_ident = &variant.ident;
                    let write_tokens = generate_write_for_fields_tokens(
                        &Attr::single_from_attrs(&variant.attrs)?,
                        &variant.fields,
                        variant_ident,
                        &idents,
                    )?;

                    Ok(quote! {
                        #name_ident::#variant_ident#bind_tokens => { #write_tokens }
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
            return Err(syn::Error::new(
                data.union_token.span(),
                "union types unsupported",
            ))
        }
    })
}

#[proc_macro_derive(CSSValue, attributes(dimension, function, keyword))]
pub fn derive_value(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name_ident = &input.ident;

    proc_macro::TokenStream::from(match generate_function_body(&input) {
        Ok(body) => quote! {
            impl ::russ_css::WriteValue for #name_ident {
                fn write_value(&self, f: &mut ::russ_css::CSSWriter) -> ::russ_css::WriteResult {
                    #body
                }
            }
        },
        Err(err) => err.to_compile_error(),
    })
}

#[proc_macro_derive(CSSDeclaration)]
pub fn derive_declaration(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name_ident = input.ident;
    let property_name_str = name_ident.to_string().to_kebab_case();

    proc_macro::TokenStream::from(quote! {
        impl ::russ_css::WriteDeclaration for #name_ident {
            fn write_property(&self, f: &mut ::russ_css::CSSWriter) -> ::russ_css::WriteResult {
                f.write_str(#property_name_str)
            }
        }
    })
}
