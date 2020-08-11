use super::{Angle, Frequency, Integer, Length, Number, Percentage, Time};
/// <https://www.w3.org/TR/css-values-3/#functional-notations>
use russ_internal::{CssValue, CssWriter, WriteResult, WriteValue};

/// <https://www.w3.org/TR/css-values-3/#calc-notation>
#[derive(Clone, Debug, Eq, Hash, PartialEq, CssValue)]
#[function]
pub struct Calc<T: ResolvedType>(CalcSum<T>);

#[derive(Clone, Debug, Eq, Hash, PartialEq, CssValue)]
enum CalcSumPart<T> {
    #[value(prefix = " + ")]
    Add(CalcProduct<T>),
    #[value(prefix = " - ")]
    Sub(CalcProduct<T>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CalcSum<T>(CalcProduct<T>, Vec<CalcSumPart<T>>);
impl<T: WriteValue> WriteValue for CalcSum<T> {
    fn write_value(&self, f: &mut CssWriter) -> WriteResult {
        self.0.write_value(f)?;
        for part in &self.1 {
            part.write_value(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, CssValue)]
enum CalcProductPart<T> {
    #[value(prefix = " * ")]
    Mul(T),
    #[value(prefix = " / ")]
    Div(Number),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CalcProduct<T>(T, Vec<CalcProductPart<T>>);
impl<T: WriteValue> WriteValue for CalcProduct<T> {
    fn write_value(&self, f: &mut CssWriter) -> WriteResult {
        self.0.write_value(f)?;
        for part in &self.1 {
            part.write_value(f)?;
        }
        Ok(())
    }
}

pub trait ResolvedType {}
impl ResolvedType for Length {}
impl ResolvedType for Frequency {}
impl ResolvedType for Angle {}
impl ResolvedType for Time {}
impl ResolvedType for Percentage {}
impl ResolvedType for Number {}
impl ResolvedType for Integer {}

// TODO finish implementation
/// <https://www.w3.org/TR/css-values-3/#attr-notation>
#[derive(Clone, Debug, Eq, Hash, PartialEq, CssValue)]
#[function]
pub struct Attr {
    // pub name: String,
    pub kind: Option<AttrKind>,
    // TODO fallback can be any value
    // pub fallback: Option<String>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, CssValue)]
pub enum AttrKind {
    #[keyword]
    String,
    #[keyword]
    Color,
    #[keyword]
    Url,
    #[keyword]
    Integer,
    #[keyword]
    Number,
    #[keyword]
    Length,
    #[keyword]
    Angle,
    #[keyword]
    Time,
    #[keyword]
    Frequency,
    // TODO support for '%': https://www.w3.org/TR/css-values-4/#valdef-type-or-value
}
