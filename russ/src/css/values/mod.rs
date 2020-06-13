// https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Types

mod basic;
mod calc;
mod dimensions;
mod image;
mod position;

use super::{Multiple, OneToFour};
pub use basic::*;
pub use calc::*;
pub use dimensions::*;
pub use image::*;
pub use position::*;
use russ_internal::{
    CSSValue, CSSWriter, FromVariants, VariantConstructors, WriteResult, WriteValue,
};
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

// TODO BasicShape could use some refactoring

// https://developer.mozilla.org/en-US/docs/Web/CSS/basic-shape
#[derive(Clone, Debug, CSSValue)]
pub enum BasicShape {
    #[function(separator = " ")]
    Inset(
        OneToFour<LengthPercentage>,
        // TODO Optional<borderradius>
    ),
    #[function(separator = " ")]
    Circle(
        Option<BasicShapeRadius>,
        #[field(option, prefix = "at ")] Option<Position>,
    ),
    #[function(separator = " ")]
    Ellipse(
        #[field(option, write_fn = "Self::write_ellipse_shape")]
        Option<(BasicShapeRadius, BasicShapeRadius)>,
        #[field(option, prefix = "at ")] Option<Position>,
    ),
    // TODO fill-rule
    #[function]
    Polygon(
        #[field(write_fn = "Self::write_polygon_vertices")]
        Multiple<(LengthPercentage, LengthPercentage)>,
    ),
    // TODO fill-rule
    #[function]
    Path(CSSString),
}
impl BasicShape {
    fn write_ellipse_shape(
        f: &mut CSSWriter,
        (rx, ry): &(BasicShapeRadius, BasicShapeRadius),
    ) -> WriteResult {
        rx.write_value(f)?;
        f.write_str(" ")?;
        ry.write_value(f)?;
        Ok(())
    }

    fn write_polygon_vertices(
        f: &mut CSSWriter,
        vertices: &[(LengthPercentage, LengthPercentage)],
    ) -> WriteResult {
        let write_vertex = |f: &mut CSSWriter, (x, y): &(LengthPercentage, LengthPercentage)| {
            x.write_value(f)?;
            f.write_str(" ")?;
            y.write_value(f)
        };

        if let Some((first, others)) = vertices.split_first() {
            write_vertex(f, first)?;
            for v in others {
                f.write_char(',')?;
                write_vertex(f, v)?;
            }
        }
        Ok(())
    }
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

pub type HexValueType = u32;

// https://developer.mozilla.org/en-US/docs/Web/CSS/color_value
#[derive(Clone, Debug, CSSValue)]
pub enum Color {
    #[value(prefix = "#", write_fn = "Self::write_hex")]
    Hex(HexValueType),
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
    /// Only 6 character hex colors are supported.
    pub fn hex(hex: HexValueType) -> Self {
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

    fn write_hex(f: &mut CSSWriter, hex: &HexValueType) -> WriteResult {
        write!(f, "{:06X}", hex)
    }
}

// https://drafts.csswg.org/css-backgrounds-3/#typedef-box
#[derive(Clone, Debug, CSSValue, VariantConstructors)]
pub enum CSSBox {
    #[keyword]
    BorderBox,
    #[keyword]
    PaddingBox,
    #[keyword]
    ContentBox,
}

// https://drafts.csswg.org/css-easing/#typedef-easing-function
#[derive(Clone, Debug, CSSValue, VariantConstructors)]
pub enum EasingFunction {
    #[keyword]
    Ease,
    #[keyword]
    EaseIn,
    #[keyword]
    EaseOut,
    #[keyword]
    EaseInOut,

    #[function]
    CubicBezier(Number, Number, Number, Number),
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

// https://drafts.csswg.org/css-shapes-1/#typedef-shape-box
#[derive(Clone, Debug, CSSValue, FromVariants)]
pub enum ShapeBox {
    Box(CSSBox),

    #[keyword]
    MarginBox,
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/transform-function
// #[derive(Clone, Debug, CSSValue)]
pub enum TransformFunction {
    // TODO
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
