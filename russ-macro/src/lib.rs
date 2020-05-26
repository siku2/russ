mod css;

use css::Styles;
use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use quote::ToTokens;
use syn::parse_macro_input;

#[proc_macro_hack]
pub fn static_css(input: TokenStream) -> TokenStream {
    TokenStream::from(parse_macro_input!(input as Styles).into_token_stream())
}
