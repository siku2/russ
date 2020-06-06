// https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Types

use super::{CSSWriter, WriteResult, WriteValue};
use russ_css::CSSValue;
use std::io::Write;

// https://developer.mozilla.org/en-US/docs/Web/CSS/angle
#[derive(Clone, Copy, CSSValue)]
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
#[derive(Clone, Copy, CSSValue)]
pub enum AnglePercentage {
    Angle(Angle),
    Percentage(Percentage),
}

#[derive(Clone, Copy, CSSValue)]
pub enum BasicShapeArg {
    Length(Length),
    Percentage(Percentage),
}

#[derive(Clone, Copy, CSSValue)]
pub enum BasicShapeRadius {
    Length(Length),
    Percentage(Percentage),

    #[keyword]
    ClosestSide,
    #[keyword]
    FarthestSide,
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/basic-shape
// #[derive(CSSValue)]
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
#[derive(Clone, Copy, CSSValue)]
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
// #[derive(CSSValue)]
pub enum Color {
    Hex(Integer),
    Rgb {
        r: NumberPercentage,
        g: NumberPercentage,
        b: NumberPercentage,
        a: Option<NumberPercentage>,
    },
    Hsl {
        h: NumberPercentage,
        s: NumberPercentage,
        l: NumberPercentage,
        a: Option<NumberPercentage>,
    },

    // #[keyword]
    Transparent,
    // #[keyword]
    CurrentColor,
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/custom-ident
#[derive(CSSValue)]
pub struct CustomIdent(CSSString);

// https://developer.mozilla.org/en-US/docs/Web/CSS/string
pub struct CSSString(String);
impl WriteValue for CSSString {
    fn write_value(&self, f: &mut CSSWriter) -> WriteResult {
        // TODO escape " inside of string, maybe do this when creating the string instead?
        write!(f, r#""{}""#, self.0)
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
// #[derive(CSSValue)]
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
#[derive(Clone, Copy, CSSValue)]
#[dimension(unit = "fr")]
pub struct Flex(Number);

// https://developer.mozilla.org/en-US/docs/Web/CSS/frequency
#[derive(Clone, Copy, CSSValue)]
pub enum Frequency {
    #[dimension(unit = "Hz")]
    Hz(Number),
    #[dimension(unit = "kHz")]
    Khz(Number),
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/frequency-percentage
#[derive(Clone, Copy, CSSValue)]
pub enum FrequencyPercentage {
    Frequency(Frequency),
    Percentage(Percentage),
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/gradient
// #[derive(CSSValue)]
pub enum Gradient {
    // TODO
    Linear(),
    Radial(),
    Repeating(),
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/image
// #[derive(CSSValue)]
pub enum Image {
    Url(Url),
    Gradient(Gradient),
    // TODO
    Element(),
    Image(),
    CrossFade(),
    ImageSet(),
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/integer
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy, CSSValue)]
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
#[derive(Clone, Copy, CSSValue)]
pub enum LengthPercentage {
    Length(Length),
    Percentage(Percentage),
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/number
#[derive(Clone, Copy)]
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

#[derive(Clone, Copy, CSSValue)]
pub enum NumberPercentage {
    Number(Number),
    Percentage(Percentage),
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/percentage
#[derive(Clone, Copy, CSSValue)]
#[dimension(unit = "%")]
pub struct Percentage(Number);

#[derive(Clone, Copy, CSSValue)]
pub enum PositionHorizontalKeyword {
    #[keyword]
    Left,
    #[keyword]
    Center,
    #[keyword]
    Right,
}

#[derive(Clone, Copy, CSSValue)]
pub enum PositionVerticalKeyword {
    #[keyword]
    Top,
    #[keyword]
    Center,
    #[keyword]
    Bottom,
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/position_value
#[derive(CSSValue)]
pub struct Position {
    // TODO "center" cannot be used for 4 values
    horizontal_anchor: Option<PositionHorizontalKeyword>,
    horizontal_offset: Option<LengthPercentage>,
    vertical_anchor: Option<PositionVerticalKeyword>,
    vertical_offset: Option<LengthPercentage>,
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/ratio
#[derive(Clone, Copy, CSSValue)]
// TODO join by /
#[join("/")]
pub struct Ratio(Integer, Integer);

// TODO resolution
// TODO shape-box

// https://developer.mozilla.org/en-US/docs/Web/CSS/time
#[derive(Clone, Copy, CSSValue)]
pub enum Time {
    #[dimension]
    S(Number),
    #[dimension]
    Ms(Number),
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/time-percentage
#[derive(Clone, Copy, CSSValue)]
pub enum TimePercentage {
    Time(Time),
    Percentage(Percentage),
}

// TODO timing-function

// https://developer.mozilla.org/en-US/docs/Web/CSS/transform-function
// #[derive(CSSValue)]
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
#[derive(CSSValue)]
pub struct Url(CSSString);
