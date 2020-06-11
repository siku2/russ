// https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Types

mod basic;
mod calc;
mod dimensions;
mod image;
mod position;

use super::{CSSWriter, WriteResult};
pub use basic::*;
pub use calc::*;
pub use dimensions::*;
pub use image::*;
pub use position::*;
use russ_internal::{CSSValue, FromVariants, VariantConstructors};
use std::io::Write;

#[derive(Clone, Debug, CSSValue, FromVariants)]
pub enum BasicShapeRadius {
    #[from_variant(into)]
    LengthPercentage(Length),

    #[keyword]
    ClosestSide,
    #[keyword]
    FarthestSide,
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/basic-shape
// #[derive(Clone, Debug, CSSValue)]
pub enum BasicShape {
    // TODO separate arguments with SPACE
    // #[function]
    Inset(
        LengthPercentage,
        Option<LengthPercentage>,
        Option<LengthPercentage>,
        Option<LengthPercentage>,
        // TODO Optional<borderradius>
    ),
    // #[function]
    Circle(Option<BasicShapeRadius>, Option<Position>),
    // #[function]
    Ellipse(
        Option<(BasicShapeRadius, BasicShapeRadius)>,
        Option<Position>,
    ),
    // TODO fill-rule
    // #[function]
    Polygon(Vec<(LengthPercentage, LengthPercentage)>),
    // TODO fill-rule
    // #[function]
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
#[derive(Clone, Debug, CSSValue)]
pub enum Color {
    #[value(prefix = "#", write_fn = "Self::write_hex")]
    Hex(usize),
    #[function()]
    Rgb {
        r: NumberPercentage,
        g: NumberPercentage,
        b: NumberPercentage,
        a: Option<NumberPercentage>,
    },
    #[function()]
    Hsl {
        h: Angle,
        s: Percentage,
        l: Percentage,
        a: Option<NumberPercentage>,
    },

    #[keyword]
    Transparent,
    #[keyword(value = "currentcolor")]
    CurrentColor,
}
impl Color {
    pub fn hex(hex: usize) -> Self {
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

    pub fn rgba(
        r: impl Into<NumberPercentage>,
        g: impl Into<NumberPercentage>,
        b: impl Into<NumberPercentage>,
        a: impl Into<NumberPercentage>,
    ) -> Self {
        Self::Rgb {
            r: r.into(),
            g: g.into(),
            b: b.into(),
            a: Some(a.into()),
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

    pub fn hsla(
        h: impl Into<Angle>,
        s: impl Into<Percentage>,
        l: impl Into<Percentage>,
        a: impl Into<NumberPercentage>,
    ) -> Self {
        Self::Hsl {
            h: h.into(),
            s: s.into(),
            l: l.into(),
            a: Some(a.into()),
        }
    }

    fn write_hex(f: &mut CSSWriter, hex: &usize) -> WriteResult {
        write!(f, "{:X}", hex)
    }
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/filter-function
#[derive(Clone, Debug, CSSValue, VariantConstructors)]
pub enum FilterFunction {
    #[function]
    Blur(Length),
    #[function]
    Brightness(NumberPercentage),
    #[function]
    Contrast(NumberPercentage),
    #[function(separator = " ")]
    DropShadow {
        offset_x: Length,
        offset_y: Length,
        blur_radius: Option<Length>,
        color: Option<Color>,
    },
    #[function]
    Grayscale(NumberPercentage),
    #[function]
    HueRotate(Angle),
    #[function]
    Invert(NumberPercentage),
    #[function]
    Opacity(NumberPercentage),
    #[function]
    Saturate(NumberPercentage),
    #[function]
    Sepia(NumberPercentage),
}

// TODO shape-box

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
