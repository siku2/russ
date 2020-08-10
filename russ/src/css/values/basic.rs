use super::Calc;
use russ_internal::{CssValue, CssWriter, FromVariants, WriteResult, WriteValue};
use std::{
    cmp::{Ordering, PartialEq, PartialOrd},
    hash::{Hash, Hasher},
    io::Write,
};

// https://developer.mozilla.org/en-US/docs/Web/CSS/custom-ident
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CustomIdent(String);
impl WriteValue for CustomIdent {
    fn write_value(&self, f: &mut CssWriter) -> WriteResult {
        f.write_str(&self.0)
    }
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/string
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

pub type IntegerValueType = i32;

// https://developer.mozilla.org/en-US/docs/Web/CSS/integer
#[derive(Clone, Debug, Eq, Hash, PartialEq, FromVariants)]
pub enum Integer {
    #[from_variant(into)]
    Value(IntegerValueType),
    Calc(Box<Calc>),
}
impl WriteValue for Integer {
    fn write_value(&self, f: &mut CssWriter) -> WriteResult {
        match self {
            Self::Value(v) => write!(f, "{}", v),
            Self::Calc(calc) => calc.write_value(f),
        }
    }
}
impl Default for Integer {
    fn default() -> Self {
        Self::Value(Default::default())
    }
}
impl PartialOrd for Integer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Value(s), Self::Value(o)) => s.partial_cmp(o),
            _ => None,
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
    fn write_value(&self, f: &mut CssWriter) -> WriteResult {
        match self {
            Self::Value(v) => write!(f, "{}", v),
            Self::Calc(calc) => calc.write_value(f),
        }
    }
}
impl Default for Number {
    fn default() -> Self {
        Self::Value(Default::default())
    }
}
impl Eq for Number {}
impl Hash for Number {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Value(v) => v.to_bits().hash(state),
            Self::Calc(v) => v.hash(state),
        }
    }
}
impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // TODO find a method that guarantees that s == o => render(s) == render(o)
            (Self::Value(s), Self::Value(o)) => s.eq(o),
            (Self::Calc(s), Self::Calc(o)) => s.eq(o),
            _ => false,
        }
    }
}
impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Value(s), Self::Value(o)) => s.partial_cmp(o),
            _ => None,
        }
    }
}
impl From<Calc> for Number {
    fn from(v: Calc) -> Self {
        Self::Calc(Box::new(v))
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, CssValue, FromVariants)]
pub enum NumberPercentage {
    #[from_variant(into)]
    Number(Number),
    Percentage(Percentage),
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/percentage
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, PartialOrd, CssValue)]
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

// TODO manual eq, ord implementation so that 16/4 == 4/1
// https://developer.mozilla.org/en-US/docs/Web/CSS/ratio
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, CssValue)]
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
