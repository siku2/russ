/// <https://www.w3.org/TR/css-values-3/#numeric-types>
use super::{Angle, Calc, Frequency, Length, Time};
use russ_internal::{CssValue, CssWriter, FromVariants, WriteResult, WriteValue};
use std::{
    cmp::{Ordering, PartialEq, PartialOrd},
    hash::{Hash, Hasher},
    io::Write,
};

pub(crate) type IntegerValueType = i32;

/// <https://www.w3.org/TR/css-values-3/#integers>
#[derive(Clone, Debug, Eq, Hash, PartialEq, FromVariants)]
pub enum Integer {
    #[from_variant(into)]
    Value(IntegerValueType),
    Calc(Box<Calc<Self>>),
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

impl From<Calc<Integer>> for Integer {
    fn from(v: Calc<Self>) -> Self {
        Self::Calc(Box::new(v))
    }
}

pub(crate) type NumberValueType = f64;

//. <https://www.w3.org/TR/css-values-3/#numbers>
#[derive(Clone, Debug, FromVariants)]
pub enum Number {
    #[from_variant(into)]
    Value(NumberValueType),
    Calc(Box<Calc<Number>>),
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

impl From<Calc<Number>> for Number {
    fn from(v: Calc<Number>) -> Self {
        Self::Calc(Box::new(v))
    }
}

/// <https://www.w3.org/TR/css-values-3/#percentages>
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, PartialOrd, CssValue)]
#[dimension(unit = "%")]
pub struct Percentage(Number);

impl<T> From<T> for Percentage
where
    T: Into<Number>,
{
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

macro_rules! value_percentage {
    ($type_name:ident, $field_name:ident, $t:ty) => {
        #[derive(Clone, Debug, Eq, Hash, PartialEq, CssValue)]
        pub enum $type_name {
            $field_name($t),
            Percentage(Percentage),
        }

        impl<T> From<T> for $type_name
        where
            T: Into<$t>,
        {
            fn from(v: T) -> Self {
                Self::$field_name(v.into())
            }
        }
        impl From<Percentage> for $type_name {
            fn from(v: Percentage) -> Self {
                Self::Percentage(v)
            }
        }
    };
    ($type_name:ident, $name:ident) => {
        value_percentage!($type_name, $name, $name);
    };
}

value_percentage!(LengthPercentage, Length);
value_percentage!(FrequencyPercentage, Frequency);
value_percentage!(AnglePercentage, Angle);
value_percentage!(TimePercentage, Time);
value_percentage!(NumberPercentage, Number);

#[cfg(test)]
mod tests {
    use super::*;
    use russ_internal::render_to_string;

    #[test]
    fn integer() -> WriteResult {
        assert_eq!(render_to_string(Integer::from(5))?, "5");
        assert_eq!(render_to_string(Integer::from(-100))?, "-100");

        Ok(())
    }

    #[test]
    fn number() -> WriteResult {
        assert_eq!(render_to_string(Number::from(5))?, "5");
        assert_eq!(render_to_string(Number::from(5.5))?, "5.5");
        assert_eq!(render_to_string(Number::from(-5e10))?, "-50000000000");

        Ok(())
    }

    #[test]
    fn percentage() -> WriteResult {
        assert_eq!(render_to_string(Percentage::from(13))?, "13%");

        Ok(())
    }
}
