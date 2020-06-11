use super::Calc;
use russ_internal::{CSSValue, CSSWriter, FromVariants, WriteResult, WriteValue};
use std::io::Write;

// https://developer.mozilla.org/en-US/docs/Web/CSS/custom-ident
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CustomIdent(String);
impl WriteValue for CustomIdent {
    fn write_value(&self, f: &mut CSSWriter) -> WriteResult {
        f.write_str(&self.0)
    }
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/string
#[derive(Clone, Debug)]
pub struct CSSString(String);
impl WriteValue for CSSString {
    fn write_value(&self, f: &mut CSSWriter) -> WriteResult {
        write!(f, "\"{}\"", self.0.replace("\"", "\\\""))
    }
}
impl<T> From<T> for CSSString
where
    T: Into<String>,
{
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

pub type IntegerValueType = i32;

// https://developer.mozilla.org/en-US/docs/Web/CSS/integer
#[derive(Clone, Debug, FromVariants)]
pub enum Integer {
    #[from_variant(into)]
    Value(IntegerValueType),
    Calc(Box<Calc>),
}
impl WriteValue for Integer {
    fn write_value(&self, f: &mut CSSWriter) -> WriteResult {
        match self {
            Self::Value(v) => write!(f, "{}", v),
            Self::Calc(calc) => calc.write_value(f),
        }
    }
}
impl From<Calc> for Integer {
    fn from(v: Calc) -> Self {
        Self::Calc(Box::new(v))
    }
}

pub type NumberValueType = f64;

// https://developer.mozilla.org/en-US/docs/Web/CSS/number
#[derive(Clone, Debug, FromVariants)]
pub enum Number {
    #[from_variant(into)]
    Value(NumberValueType),
    Calc(Box<Calc>),
}
impl WriteValue for Number {
    fn write_value(&self, f: &mut CSSWriter) -> WriteResult {
        match self {
            Self::Value(v) => write!(f, "{}", v),
            Self::Calc(calc) => calc.write_value(f),
        }
    }
}
impl From<Calc> for Number {
    fn from(v: Calc) -> Self {
        Self::Calc(Box::new(v))
    }
}

#[derive(Clone, Debug, CSSValue, FromVariants)]
pub enum NumberPercentage {
    #[from_variant(into)]
    Number(Number),
    Percentage(Percentage),
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/percentage
#[derive(Clone, Debug, CSSValue)]
#[dimension(unit = "%")]
pub struct Percentage(pub Number);
impl<T> From<T> for Percentage
where
    T: Into<Number>,
{
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/ratio
#[derive(Clone, Debug, CSSValue)]
#[value(separator = "/")]
pub struct Ratio(pub Integer, pub Integer);
impl<W, H> From<(W, H)> for Ratio
where
    W: Into<Integer>,
    H: Into<Integer>,
{
    fn from((w, h): (W, H)) -> Self {
        Self(w.into(), h.into())
    }
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/url
#[derive(Clone, Debug, CSSValue)]
#[function]
pub struct Url(CSSString);
impl<T> From<T> for Url
where
    T: Into<CSSString>,
{
    fn from(v: T) -> Self {
        Self(v.into())
    }
}
