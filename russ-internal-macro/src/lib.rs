mod derive;
mod vds;

use syn::{parse_macro_input, DeriveInput};

fn to_token_stream(res: syn::Result<proc_macro2::TokenStream>) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(match res {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    })
}

#[proc_macro_derive(CssValue, attributes(dimension, function, keyword, value, field))]
pub fn derive_value(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    to_token_stream(derive::generate_write_value(input))
}

#[proc_macro_derive(CssDeclaration)]
pub fn derive_declaration(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    to_token_stream(derive::generate_write_declaration(input))
}

#[proc_macro_derive(FromVariants, attributes(from_variant))]
pub fn derive_from_variants(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    to_token_stream(derive::generate_from_variants(input))
}

#[proc_macro_derive(VariantConstructors, attributes(constructor))]
pub fn derive_variant_constructors(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    to_token_stream(derive::generate_variant_constructors(input))
}

#[proc_macro]
pub fn vds(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let v = parse_macro_input!(input as vds::VDS);
    to_token_stream(v.test())
}
