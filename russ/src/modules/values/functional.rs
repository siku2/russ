use super::{
    Angle, AnglePercentage, Frequency, FrequencyPercentage, Integer, IntegerValueType, Length,
    LengthPercentage, Number, NumberPercentage, NumberValueType, Percentage, Time, TimePercentage,
};
/// <https://www.w3.org/TR/css-values-3/#functional-notations>
use russ_internal::{CssValue, CssWriter, WriteResult, WriteValue};

/// <https://www.w3.org/TR/css-values-3/#calc-notation>
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Calc<T>(CalcValue<T>, Vec<CalcPart<T>>);
impl<T: ResolvedType> Calc<T> {
    fn new_unary(v: impl Into<T>) -> Self {
        Self(CalcValue::Value(v.into()), Vec::new())
    }

    fn into_unary(self) -> Self {
        if self.is_unary() {
            self
        } else {
            Self(CalcValue::Calc(Box::new(self)), Vec::new())
        }
    }

    pub fn is_unary(&self) -> bool {
        self.1.is_empty()
    }

    pub fn bin_add(a: impl Into<Self>, b: impl Into<Self>) -> Self {
        let mut res = a.into();
        res.push_add(b);
        res
    }

    pub fn bin_sub(a: impl Into<Self>, b: impl Into<Self>) -> Self {
        let mut res = a.into();
        res.push_sub(b);
        res
    }

    pub fn bin_mul(a: impl Into<Self>, b: impl Into<Calc<Number>>) -> Self {
        let mut res = a.into().into_unary();
        res.push_mul(b);
        res
    }

    pub fn bin_div(a: impl Into<Self>, b: impl Into<Calc<Number>>) -> Self {
        let mut res = a.into().into_unary();
        res.push_div(b);
        res
    }

    pub fn push_add(&mut self, other: impl Into<Self>) {
        let other = other.into();
        self.1.push(CalcPart::Add(other.0));
        self.1.extend(other.1);
    }

    pub fn push_sub(&mut self, other: impl Into<Self>) {
        let other = other.into();
        let value = if other.is_unary() {
            other.0
        } else {
            CalcValue::Calc(Box::new(other))
        };
        self.1.push(CalcPart::Sub(value));
    }

    pub fn push_mul(&mut self, other: impl Into<Calc<Number>>) {
        let value = other.into().into_unary().0;
        self.1.push(CalcPart::Mul(value));
    }

    pub fn push_div(&mut self, other: impl Into<Calc<Number>>) {
        let value = other.into().into_unary().0;
        self.1.push(CalcPart::Div(value));
    }
}
impl<T: WriteValue> Calc<T> {
    fn write_inner_value(&self, f: &mut CssWriter) -> WriteResult {
        self.0.write_value(f)?;
        for part in &self.1 {
            part.write_value(f)?;
        }

        Ok(())
    }
}
impl<T: WriteValue> WriteValue for Calc<T> {
    fn write_value(&self, f: &mut CssWriter) -> WriteResult {
        f.write_str("calc(")?;
        self.write_inner_value(f)?;
        f.write_char(')')
    }
}

impl<T> From<T> for Calc<T>
where
    T: ResolvedType,
{
    fn from(v: T) -> Self {
        Self::new_unary(v)
    }
}

macro_rules! calc_impl_from {
    ($r:ty : $t:ty ) => {
        impl From<$t> for Calc<$r> {
            fn from(v: $t) -> Self {
                Self::new_unary(v)
            }
        }
    };
    ($r:ty : $($t:ty),+ ) => {
        $(
            calc_impl_from!($r: $t);
        )*
    };
}

calc_impl_from!(Number: NumberValueType, IntegerValueType);
calc_impl_from!(Integer: IntegerValueType);
calc_impl_from!(LengthPercentage: Length, Percentage);
calc_impl_from!(FrequencyPercentage: Frequency, Percentage);
calc_impl_from!(AnglePercentage: Angle, Percentage);
calc_impl_from!(TimePercentage: Time, Percentage);
calc_impl_from!(NumberPercentage: Number, Percentage);

#[derive(Clone, Debug, Eq, Hash, PartialEq, CssValue)]
enum CalcPart<T> {
    #[value(prefix = " + ")]
    Add(CalcValue<T>),
    #[value(prefix = " - ")]
    Sub(CalcValue<T>),
    #[value(prefix = " * ")]
    Mul(CalcValue<Number>),
    #[value(prefix = " / ")]
    Div(CalcValue<Number>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, CssValue)]
pub enum CalcValue<T> {
    Value(T),
    #[value(write_fn = "Self::write_calc")]
    Calc(Box<Calc<T>>),
}
impl<T: WriteValue> CalcValue<T> {
    fn write_calc(f: &mut CssWriter, v: &Box<Calc<T>>) -> WriteResult {
        v.write_inner_value(f)
    }
}

pub trait ResolvedType {}

macro_rules! mark_resolved_type {
    ($t:ty) => {
        impl ResolvedType for $t {}
    };
    ($($t:ty),+) => {
        $(
            mark_resolved_type!($t);
        )*
    };
}

mark_resolved_type!(
    Length,
    LengthPercentage,
    Frequency,
    FrequencyPercentage,
    Angle,
    AnglePercentage,
    Time,
    TimePercentage,
    Percentage,
    Number,
    NumberPercentage,
    Integer
);

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

#[cfg(test)]
mod tests {
    use super::*;
    use russ_internal::render_to_string;

    #[test]
    fn calc() -> WriteResult {
        assert_eq!(
            render_to_string(Calc::<LengthPercentage>::bin_sub(
                Percentage::from(100),
                Length::px(80)
            ))?,
            "calc(100% - 80px)"
        );
        assert_eq!(
            render_to_string(Calc::<Length>::bin_div(
                Calc::bin_div(Length::px(100), 2),
                2
            ))?,
            "calc(100px / 2 / 2)"
        );

        Ok(())
    }
}
