use proc_macro2::Span;
use syn::{parse_quote, Expr, Ident, Type};

/// Create an `Ident` from the given string making sure that the resulting identifier is valid.
pub fn parse_ident_with_span(name: &str, span: Span) -> syn::Result<Ident> {
    // make sure the ident is valid but create it manually to preserve the span
    syn::parse_str::<Ident>(name)?;
    Ok(Ident::new(&name, span))
}

pub fn gen_parse_type(ty: Type, get_parse_stream: Expr) -> Expr {
    parse_quote! {
        <#ty as ::syn::parse::Parse>::parse(#get_parse_stream)
    }
}
