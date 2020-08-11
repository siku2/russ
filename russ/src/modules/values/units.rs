use super::Number;
use russ_internal::{CssValue, VariantConstructors};

// TODO: Calc<T> needs to be supported for these too

/// <https://www.w3.org/TR/css-values-3/#lengths>
#[derive(Clone, Debug, Eq, Hash, PartialEq, CssValue, VariantConstructors)]
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

/// <https://www.w3.org/TR/css-values-3/#angles>
#[derive(Clone, Debug, Eq, Hash, PartialEq, CssValue, VariantConstructors)]
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

/// <https://www.w3.org/TR/css-values-3/#time>
#[derive(Clone, Debug, Eq, Hash, PartialEq, CssValue, VariantConstructors)]
pub enum Time {
    #[dimension]
    S(Number),
    #[dimension]
    Ms(Number),
}

/// <https://www.w3.org/TR/css-values-3/#frequency>
#[derive(Clone, Debug, Eq, Hash, PartialEq, CssValue, VariantConstructors)]
pub enum Frequency {
    #[dimension(unit = "Hz")]
    Hz(Number),
    #[dimension(unit = "kHz")]
    Khz(Number),
}

/// <https://www.w3.org/TR/css-values-3/#resolution>
#[derive(Clone, Debug, Eq, Hash, PartialEq, CssValue, VariantConstructors)]
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
