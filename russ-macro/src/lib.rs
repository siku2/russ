use proc_macro_hack::proc_macro_hack;
use quote::ToTokens;
use syn::Expr;

mod props;
mod values;

use values::Value;

trait ToRussRepr {
    fn to_russ_repr(&self) -> Expr;
}

#[proc_macro_hack]
pub fn css_value_macro(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let value = syn::parse_macro_input!(tokens as Value);
    proc_macro::TokenStream::from(value.to_russ_repr().into_token_stream())
}
