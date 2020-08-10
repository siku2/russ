use super::{
    Angle, Flex, Frequency, Length, Number, NumberValueType, Percentage, Resolution, Time,
};
use russ_internal::{CssValue, CssWriter, FromVariants, WriteResult, WriteValue};

// TODO perhaps use Calc<T> where T is a dimension (Angle, Length) so that CalcValue only allows that particular dimension
//      Need to verify if this is actually valid though.

// https://developer.mozilla.org/en-US/docs/Web/CSS/calc
#[derive(Clone, Debug, Eq, Hash, PartialEq, CssValue)]
#[function]
pub struct Calc(CalcSum);
impl Calc {
    pub fn unary(value: impl Into<CalcSum>) -> Self {
        Self(value.into())
    }

    fn bin_sum(a: impl Into<CalcProduct>, b: CalcSumPart) -> Self {
        Self::unary(CalcSum::binary(a, b))
    }

    pub fn bin_add(a: impl Into<CalcProduct>, b: impl Into<CalcProduct>) -> Self {
        Self::bin_sum(a, CalcSumPart::Add(b.into()))
    }

    pub fn bin_sub(a: impl Into<CalcProduct>, b: impl Into<CalcProduct>) -> Self {
        Self::bin_sum(a, CalcSumPart::Sub(b.into()))
    }

    fn bin_product(a: impl Into<CalcValue>, b: CalcProductPart) -> Self {
        Self::unary(CalcProduct::binary(a, b))
    }

    pub fn bin_mul(a: impl Into<CalcValue>, b: impl Into<CalcValue>) -> Self {
        Self::bin_product(a, CalcProductPart::Mul(b.into()))
    }

    pub fn bin_div(a: impl Into<CalcValue>, b: impl Into<Number>) -> Self {
        Self::bin_product(a, CalcProductPart::Div(b.into()))
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, CssValue)]
pub enum CalcSumPart {
    #[value(prefix = " + ")]
    Add(CalcProduct),
    #[value(prefix = " - ")]
    Sub(CalcProduct),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CalcSum(pub CalcProduct, pub Vec<CalcSumPart>);
impl CalcSum {
    pub fn unary(value: impl Into<CalcProduct>) -> Self {
        Self(value.into(), Vec::new())
    }

    fn binary(a: impl Into<CalcProduct>, b: CalcSumPart) -> Self {
        Self(a.into(), vec![b])
    }
}
impl WriteValue for CalcSum {
    fn write_value(&self, f: &mut CssWriter) -> WriteResult {
        self.0.write_value(f)?;
        for part in &self.1 {
            part.write_value(f)?;
        }
        Ok(())
    }
}
impl<T> From<T> for CalcSum
where
    T: Into<CalcProduct>,
{
    fn from(v: T) -> Self {
        Self::unary(v)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, CssValue)]
pub enum CalcProductPart {
    #[value(prefix = " * ")]
    Mul(CalcValue),
    #[value(prefix = " / ")]
    Div(Number),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CalcProduct(pub CalcValue, pub Vec<CalcProductPart>);
impl CalcProduct {
    pub fn unary(value: impl Into<CalcValue>) -> Self {
        Self(value.into(), Vec::new())
    }

    fn binary(a: impl Into<CalcValue>, b: CalcProductPart) -> Self {
        Self(a.into(), vec![b])
    }
}
impl WriteValue for CalcProduct {
    fn write_value(&self, f: &mut CssWriter) -> WriteResult {
        self.0.write_value(f)?;
        for part in &self.1 {
            part.write_value(f)?;
        }
        Ok(())
    }
}
impl<T> From<T> for CalcProduct
where
    T: Into<CalcValue>,
{
    fn from(v: T) -> Self {
        Self::unary(v)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, CssValue, FromVariants)]
pub enum CalcValue {
    Number(Number),
    #[value(prefix = "(", suffix = ")")]
    CalcSum(Box<CalcSum>),
    // dimensions
    Angle(Angle),
    Flex(Flex),
    Frequency(Frequency),
    Length(Length),
    Percentage(Percentage),
    Resolution(Resolution),
    Time(Time),
}
impl<T> From<T> for CalcValue
where
    T: Into<NumberValueType>,
{
    fn from(v: T) -> Self {
        Self::Number(Number::from(v))
    }
}
impl From<Calc> for CalcValue {
    fn from(v: Calc) -> Self {
        Self::CalcSum(Box::new(v.0))
    }
}
