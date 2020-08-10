mod background;
pub use background::*;
use russ_internal::CssValue;

// TODO find a way to integrate these globals cleanly.

// https://www.w3.org/TR/css-values-4/#css-wide-keywords
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, CssValue)]
pub enum GlobalValue {
    #[keyword]
    Inherit,
    #[keyword]
    Initial,
    #[keyword]
    Unset,
}
