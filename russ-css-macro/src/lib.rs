mod args;
mod css_value;
mod from_variants;

use heck::KebabCase;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(CSSValue, attributes(dimension, function, keyword, value))]
pub fn derive_value(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    proc_macro::TokenStream::from(match css_value::generate_write_value(input) {
        Ok(tokens) => tokens,
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

#[proc_macro_derive(FromVariants, attributes(from_variant))]
pub fn derive_from_variants(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    proc_macro::TokenStream::from(match from_variants::generate_from_variants(input) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    })
}
