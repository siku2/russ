/// <https://www.w3.org/TR/css-values-3/#textual-values>
use russ_internal::{CssValue, CssWriter, WriteResult, WriteValue};
use std::{
    cmp::{PartialEq, PartialOrd},
    hash::Hash,
    io::Write,
};

// TODO: this needs to be supported by every prop
/// <https://www.w3.org/TR/css-values-3/#common-keywords>
pub enum Keyword {
    Initial,
    Inherit,
    Unset,
}

/// <https://www.w3.org/TR/css-values-3/#custom-idents>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CustomIdent(String);
impl WriteValue for CustomIdent {
    fn write_value(&self, f: &mut CssWriter) -> WriteResult {
        f.write_str(&self.0)
    }
}

/// <https://www.w3.org/TR/css-values-3/#strings>
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CssString(String);
impl WriteValue for CssString {
    fn write_value(&self, f: &mut CssWriter) -> WriteResult {
        write!(f, "\"{}\"", self.0.replace("\"", "\\\""))
    }
}

impl<T> From<T> for CssString
where
    T: Into<String>,
{
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

/// <https://www.w3.org/TR/css-values-3/#urls>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, CssValue)]
#[function]
pub struct Url(CssString);

impl<T> From<T> for Url
where
    T: Into<CssString>,
{
    fn from(v: T) -> Self {
        Self(v.into())
    }
}
