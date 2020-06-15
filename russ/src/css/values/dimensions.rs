use super::{Number, Percentage};
use russ_internal::{CSSValue, FromVariants, VariantConstructors};

// https://developer.mozilla.org/en-US/docs/Web/CSS/angle
#[derive(Clone, Debug, Eq, Hash, PartialEq, CSSValue, VariantConstructors)]
pub enum Angle {
    #[dimension]
    Deg(Number),
    #[dimension]
    Grad(Number),
    #[dimension]
    Rad(Number),
    #[dimension]
    Turn(Number),

    #[dimension(zero)]
    Zero,
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/angle-percentage
#[derive(Clone, Debug, Eq, Hash, PartialEq, CSSValue, FromVariants)]
pub enum AnglePercentage {
    #[from_variant(into)]
    Angle(Angle),
    Percentage(Percentage),
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/flex_value
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, CSSValue)]
#[dimension(unit = "fr")]
pub struct Flex(pub Number);
impl<T> From<T> for Flex
where
    T: Into<Number>,
{
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/frequency
#[derive(Clone, Debug, Eq, Hash, PartialEq, CSSValue, VariantConstructors)]
pub enum Frequency {
    #[dimension(unit = "Hz")]
    Hz(Number),
    #[dimension(unit = "kHz")]
    Khz(Number),
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/frequency-percentage
#[derive(Clone, Debug, Eq, Hash, PartialEq, CSSValue, FromVariants)]
pub enum FrequencyPercentage {
    #[from_variant(into)]
    Frequency(Frequency),
    Percentage(Percentage),
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/length
#[derive(Clone, Debug, Eq, Hash, PartialEq, CSSValue, VariantConstructors)]
pub enum Length {
    #[dimension]
    Cap(Number),
    #[dimension]
    Ch(Number),
    #[dimension]
    Em(Number),
    #[dimension]
    Ex(Number),
    #[dimension]
    Ic(Number),
    #[dimension]
    Lh(Number),
    #[dimension]
    Rem(Number),
    #[dimension]
    Rlh(Number),

    #[dimension]
    Vh(Number),
    #[dimension]
    Vw(Number),
    #[dimension]
    Vi(Number),
    #[dimension]
    Vb(Number),
    #[dimension]
    VMin(Number),
    #[dimension]
    VMax(Number),

    #[dimension]
    Px(Number),
    #[dimension]
    Cm(Number),
    #[dimension]
    Mm(Number),
    #[dimension(unit = "Q")]
    Q(Number),
    #[constructor(name = "inches")]
    #[dimension]
    In(Number),
    #[dimension]
    Pc(Number),
    #[dimension]
    Pt(Number),

    #[dimension(zero)]
    Zero,
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/length-percentage
#[derive(Clone, Debug, Eq, Hash, PartialEq, CSSValue, FromVariants)]
pub enum LengthPercentage {
    #[from_variant(into)]
    Length(Length),
    Percentage(Percentage),
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/resolution
#[derive(Clone, Debug, Eq, Hash, PartialEq, CSSValue, VariantConstructors)]
pub enum Resolution {
    #[dimension]
    Dpi(Number),
    #[dimension]
    Dpcm(Number),
    #[dimension]
    Dppx(Number),
}
impl Resolution {
    pub fn x(v: impl Into<Number>) -> Self {
        Self::Dppx(v.into())
    }
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/time
#[derive(Clone, Debug, Eq, Hash, PartialEq, CSSValue, VariantConstructors)]
pub enum Time {
    #[dimension]
    S(Number),
    #[dimension]
    Ms(Number),
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/time-percentage
#[derive(Clone, Debug, Eq, Hash, PartialEq, CSSValue, FromVariants)]
pub enum TimePercentage {
    #[from_variant(into)]
    Time(Time),
    Percentage(Percentage),
}
