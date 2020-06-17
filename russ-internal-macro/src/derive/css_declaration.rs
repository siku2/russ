use heck::KebabCase;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn generate_write_declaration(input: DeriveInput) -> syn::Result<TokenStream> {
    let name_ident = input.ident;
    let property_name_str = name_ident.to_string().to_kebab_case();

    Ok(quote! {
        impl ::russ_internal::WriteDeclaration for #name_ident {
            fn write_property(&self, f: &mut ::russ_internal::CSSWriter) -> ::russ_internal::WriteResult {
                f.write_str(#property_name_str)
            }
        }
    })
}
