/// <https://www.w3.org/TR/css-values-3/#textual-values>
use russ_internal::{CssValue, CssWriter, WriteResult, WriteValue};
use std::{
    cmp::{PartialEq, PartialOrd},
    hash::Hash,
    io::Write,
};

// TODO: this needs to be supported by every prop
// TODO: this should be contained in the cascade (https://www.w3.org/TR/css-cascade-3/) module
/// <https://www.w3.org/TR/css-values-3/#common-keywords>
pub enum Keyword {
    Initial,
    Inherit,
    Unset,
}

// TODO verify string contents
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

// TODO verify url tokens
/// <https://www.w3.org/TR/css-values-3/#urls>
#[derive(Clone, Debug, Eq, Hash, PartialEq, CssValue)]
#[function]
pub struct Url(
    CssString,
    // TODO allow VDS shortcuts like "*" instead of this
    #[field(iter, iter_separator = " ")] Vec<UrlModifier>,
);

impl<T> From<T> for Url
where
    T: Into<CssString>,
{
    fn from(v: T) -> Self {
        Self(v.into(), Vec::new())
    }
}

// TODO modifiers can be <ident> or a function
#[derive(Clone, Debug, Eq, Hash, PartialEq, CssValue)]
pub struct UrlModifier(CssString);

#[cfg(test)]
mod tests {
    use super::*;
    use russ_internal::render_to_string;

    #[test]
    fn string() -> WriteResult {
        assert_eq!(
            render_to_string(CssString::from("hello world"))?,
            "\"hello world\""
        );
        assert_eq!(
            render_to_string(CssString::from(r#" "'" "#))?,
            r#"" \"'\" ""#
        );

        Ok(())
    }

    #[test]
    fn url() -> WriteResult {
        assert_eq!(
            render_to_string(Url::from("http://www.example.com/pinkish.gif"))?,
            r#"url("http://www.example.com/pinkish.gif")"#
        );

        Ok(())
    }
}
