// https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Types

use super::{CSSWriter, WriteResult, WriteValue};
use lazy_static::lazy_static;
use regex::Regex;
use russ_css::CSSValue;
use russ_css::{FromVariants, VariantConstructors};
use std::io::Write;

// https://developer.mozilla.org/en-US/docs/Web/CSS/angle
#[derive(Clone, Copy, Debug, CSSValue, VariantConstructors)]
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
#[derive(Clone, Copy, Debug, CSSValue, FromVariants)]
pub enum AnglePercentage {
    #[from_variant(into)]
    Angle(Angle),
    Percentage(Percentage),
}

#[derive(Clone, Copy, Debug, CSSValue, FromVariants)]
pub enum BasicShapeArg {
    #[from_variant(into)]
    Length(Length),
    Percentage(Percentage),
}

#[derive(Clone, Copy, Debug, CSSValue, FromVariants)]
pub enum BasicShapeRadius {
    #[from_variant(into)]
    Length(Length),
    Percentage(Percentage),

    #[keyword]
    ClosestSide,
    #[keyword]
    FarthestSide,
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/basic-shape
// #[derive(Clone, Debug, CSSValue)]
pub enum BasicShape {
    Inset(
        BasicShapeArg,
        Option<BasicShapeArg>,
        Option<BasicShapeArg>,
        Option<BasicShapeArg>,
        // TODO Optional<borderradius>
    ),
    Circle(Option<BasicShapeRadius>, Option<Position>),
    Ellipse(
        Option<(BasicShapeRadius, BasicShapeRadius)>,
        Option<Position>,
    ),
    // TODO fill-rule
    Polygon(Vec<(BasicShapeArg, BasicShapeArg)>),
    // TODO fill-rule
    Path(CSSString),
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/blend-mode
#[derive(Clone, Copy, Debug, CSSValue)]
pub enum BlendMode {
    #[keyword]
    Normal,
    #[keyword]
    Multiply,
    #[keyword]
    Screen,
    #[keyword]
    Overlay,
    #[keyword]
    Darken,
    #[keyword]
    Lighten,
    #[keyword]
    ColorDodge,
    #[keyword]
    ColorBurn,
    #[keyword]
    HardLight,
    #[keyword]
    SoftLight,
    #[keyword]
    Difference,
    #[keyword]
    Exclusion,
    #[keyword]
    Hue,
    #[keyword]
    Saturation,
    #[keyword]
    Color,
    #[keyword]
    Luminosity,
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/color_value
// #[derive(Clone, Debug, CSSValue)]
pub enum Color {
    // #[value(prefix = "#")]
    Hex(Integer),
    // #[function]
    Rgb {
        r: NumberPercentage,
        g: NumberPercentage,
        b: NumberPercentage,
        a: Option<NumberPercentage>,
    },
    // #[function]
    Hsl {
        h: Angle,
        s: Percentage,
        l: Percentage,
        a: Option<NumberPercentage>,
    },

    // #[keyword]
    Transparent,
    // #[keyword]
    CurrentColor,
}
impl Color {
    pub fn hex(hex: Integer) -> Self {
        Self::Hex(hex)
    }

    pub fn rgb(
        r: impl Into<NumberPercentage>,
        g: impl Into<NumberPercentage>,
        b: impl Into<NumberPercentage>,
    ) -> Self {
        Self::Rgb {
            r: r.into(),
            g: g.into(),
            b: b.into(),
            a: None,
        }
    }

    pub fn hsl(h: impl Into<Angle>, s: impl Into<Percentage>, l: impl Into<Percentage>) -> Self {
        Self::Hsl {
            h: h.into(),
            s: s.into(),
            l: l.into(),
            a: None,
        }
    }
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/custom-ident
#[derive(Clone, Debug, CSSValue)]
pub struct CustomIdent(CSSString);

// https://developer.mozilla.org/en-US/docs/Web/CSS/string
#[derive(Clone, Debug)]
pub struct CSSString(String);
impl WriteValue for CSSString {
    fn write_value(&self, f: &mut CSSWriter) -> WriteResult {
        lazy_static! {
            static ref RE: Regex = Regex::new("\"").unwrap();
        }

        write!(f, "\"{}\"", RE.replace_all(&self.0, "\\\""))
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

// https://developer.mozilla.org/en-US/docs/Web/CSS/filter-function
// #[derive(Clone, Debug, CSSValue)]
pub enum FilterFunction {
    // TODO
    Blur(),
    Brightness(),
    Contrast(),
    DropShadow(),
    Grayscale(),
    HueRotate(),
    Invert(),
    Opacity(),
    Saturate(),
    Sepia(),
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/flex_value
#[derive(Clone, Copy, Debug, CSSValue)]
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
#[derive(Clone, Copy, Debug, CSSValue, VariantConstructors)]
pub enum Frequency {
    #[dimension(unit = "Hz")]
    Hz(Number),
    #[dimension(unit = "kHz")]
    Khz(Number),
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/frequency-percentage
#[derive(Clone, Copy, Debug, CSSValue, FromVariants)]
pub enum FrequencyPercentage {
    #[from_variant(into)]
    Frequency(Frequency),
    Percentage(Percentage),
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/gradient
// #[derive(Clone, Debug, CSSValue)]
pub enum Gradient {
    // TODO
    Linear(),
    Radial(),
    Repeating(),
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/image
#[derive(Clone, Debug, CSSValue, FromVariants)]
pub enum Image {
    Url(Url),
    // Gradient(Gradient),
    // TODO
    // Element(),
    // Image(),
    // CrossFade(),
    // ImageSet(),
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/integer
#[derive(Clone, Copy, Debug)]
pub struct Integer(i32);
impl WriteValue for Integer {
    fn write_value(&self, f: &mut CSSWriter) -> WriteResult {
        write!(f, "{}", self.0)
    }
}
impl<T> From<T> for Integer
where
    T: Into<i32>,
{
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/length
#[derive(Clone, Copy, Debug, CSSValue, VariantConstructors)]
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
    #[dimension]
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
#[derive(Clone, Copy, Debug, CSSValue, FromVariants)]
pub enum LengthPercentage {
    #[from_variant(into)]
    Length(Length),
    Percentage(Percentage),
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/number
#[derive(Clone, Copy, Debug)]
pub struct Number(f64);
impl WriteValue for Number {
    fn write_value(&self, f: &mut CSSWriter) -> WriteResult {
        write!(f, "{}", self.0)
    }
}
impl<T> From<T> for Number
where
    T: Into<f64>,
{
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

#[derive(Clone, Copy, Debug, CSSValue, FromVariants)]
pub enum NumberPercentage {
    #[from_variant(into)]
    Number(Number),
    Percentage(Percentage),
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/percentage
#[derive(Clone, Copy, Debug, CSSValue)]
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

#[derive(Clone, Copy, Debug, CSSValue)]
pub enum PositionHorizontalAnchor {
    #[keyword]
    Left,
    #[keyword]
    Right,
}
#[derive(Clone, Copy, Debug, CSSValue, FromVariants)]
pub enum PositionHorizontal {
    Anchor(PositionHorizontalAnchor),
    #[value]
    Offset(Option<PositionHorizontalAnchor>, LengthPercentage),
    #[keyword]
    Center,
}
impl<T> From<T> for PositionHorizontal
where
    T: Into<LengthPercentage>,
{
    fn from(v: T) -> Self {
        Self::Offset(None, v.into())
    }
}
impl<T> From<(PositionHorizontalAnchor, T)> for PositionHorizontal
where
    T: Into<LengthPercentage>,
{
    fn from((anchor, offset): (PositionHorizontalAnchor, T)) -> Self {
        Self::Offset(Some(anchor), offset.into())
    }
}

#[derive(Clone, Copy, Debug, CSSValue)]
pub enum PositionVerticalAnchor {
    #[keyword]
    Top,
    #[keyword]
    Bottom,
}
#[derive(Clone, Copy, Debug, CSSValue, FromVariants)]
pub enum PositionVertical {
    Anchor(PositionVerticalAnchor),
    #[value]
    Offset(Option<PositionVerticalAnchor>, LengthPercentage),
    #[keyword]
    Center,
}
impl<T> From<T> for PositionVertical
where
    T: Into<LengthPercentage>,
{
    fn from(v: T) -> Self {
        Self::Offset(None, v.into())
    }
}
impl<T> From<(PositionVerticalAnchor, T)> for PositionVertical
where
    T: Into<LengthPercentage>,
{
    fn from((anchor, offset): (PositionVerticalAnchor, T)) -> Self {
        Self::Offset(Some(anchor), offset.into())
    }
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/position_value
#[derive(Clone, Debug, CSSValue)]
#[value]
pub struct Position {
    horizontal: Option<PositionHorizontal>,
    vertical: Option<PositionVertical>,
}
impl Position {
    fn new(mut horizontal: Option<PositionHorizontal>, vertical: Option<PositionVertical>) -> Self {
        // `x(None) y(5rem)` would generate `5rem` which would be interpreted as `x(5rem) y(center)`.
        // This check makes sure that we generate `x(center) y(5rem)` instead.
        if horizontal.is_none() && matches!(&vertical, Some(PositionVertical::Offset(None, _))) {
            horizontal = Some(PositionHorizontal::Center);
        }
        Self {
            horizontal,
            vertical,
        }
    }

    pub fn center() -> Self {
        Self::x(PositionHorizontal::Center)
    }
    pub fn x(horizontal: impl Into<PositionHorizontal>) -> Self {
        Self::new(Some(horizontal.into()), None)
    }
    pub fn y(vertical: impl Into<PositionVertical>) -> Self {
        Self::new(None, Some(vertical.into()))
    }
    pub fn xy(
        horizontal: impl Into<PositionHorizontal>,
        vertical: impl Into<PositionVertical>,
    ) -> Self {
        Self::new(Some(horizontal.into()), Some(vertical.into()))
    }

    pub fn xy_option(
        horizontal: Option<PositionHorizontal>,
        vertical: Option<PositionVertical>,
    ) -> Option<Self> {
        if horizontal.is_some() || vertical.is_some() {
            Some(Self::new(horizontal, vertical))
        } else {
            None
        }
    }
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/ratio
#[derive(Clone, Copy, Debug, CSSValue)]
#[value(separator = "/")]
pub struct Ratio(pub Integer, pub Integer);
impl Ratio {
    pub fn width(&self) -> Integer {
        self.0
    }

    pub fn height(&self) -> Integer {
        self.0
    }
}

// TODO resolution
// TODO shape-box

// https://developer.mozilla.org/en-US/docs/Web/CSS/time
#[derive(Clone, Copy, Debug, CSSValue, VariantConstructors)]
pub enum Time {
    #[dimension]
    S(Number),
    #[dimension]
    Ms(Number),
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/time-percentage
#[derive(Clone, Copy, Debug, CSSValue, FromVariants)]
pub enum TimePercentage {
    #[from_variant(into)]
    Time(Time),
    Percentage(Percentage),
}

// TODO timing-function

// https://developer.mozilla.org/en-US/docs/Web/CSS/transform-function
// #[derive(Clone, Debug, CSSValue)]
pub enum TransformFunction {
    Matrix(),
    Matrix3d(),

    Perspective(),

    Rotate(),
    Rotate3d(),
    RotateX(),
    RotateY(),
    RotateZ(),

    Scale(),
    Scale3d(),
    ScaleX(),
    ScaleY(),
    ScaleZ(),

    Skew(),
    SkewX(),
    SkewY(),

    Translate(),
    Translate3d(),
    TranslateX(),
    TranslateY(),
    TranslateZ(),
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/url
#[derive(Clone, Debug, CSSValue)]
pub struct Url(pub CSSString);
