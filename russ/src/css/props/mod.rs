mod background;
pub use background::*;
use russ_internal::CSSValue;

// TODO find a way to integrate these cleanly.

// https://www.w3.org/TR/css-values-4/#css-wide-keywords
#[derive(CSSValue)]
pub enum GlobalValue {
    #[keyword]
    Inherit,
    #[keyword]
    Initial,
    #[keyword]
    Unset,
}
