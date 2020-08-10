/// <https://www.w3.org/TR/css-values-3/#numeric-types>
use super::units::{Angle, Frequency, Length, Time};
use russ_internal::{CssValue, CssWriter, FromVariants, WriteResult, WriteValue};
use std::{
    cmp::{Ordering, PartialEq, PartialOrd},
    hash::{Hash, Hasher},
    io::Write,
};

pub type IntegerValueType = i32;

/// <https://www.w3.org/TR/css-values-3/#integers>
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

//. <https://www.w3.org/TR/css-values-3/#numbers>
#[derive(Clone, Debug, FromVariants)]
pub enum Number {
    #[from_variant(into)]
    Value(NumberValueType),
    // TODO is this really correct?
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

/// <https://www.w3.org/TR/css-values-3/#percentages>
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

// TODO fix codegen
/// <https://www.w3.org/TR/css-values-3/#mixed-percentages>
#[derive(Clone, Debug, Eq, Hash, PartialEq, CssValue, FromVariants)]
pub enum ValuePercentage<T> {
    #[from_variant(into)]
    Value(T),
    Percentage(Percentage),
}

pub type LengthPercentage = ValuePercentage<Length>;
pub type FrequencyPercentage = ValuePercentage<Frequency>;
pub type AnglePercentage = ValuePercentage<Angle>;
pub type TimePercentage = ValuePercentage<Time>;
pub type NumberPercentage = ValuePercentage<Number>;
