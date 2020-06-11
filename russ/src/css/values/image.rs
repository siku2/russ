use super::{Angle, CSSString, Color, LengthPercentage, Percentage, Position, Resolution, Url};
use russ_internal::{CSSValue, CSSWriter, FromVariants, WriteResult, WriteValue};

#[derive(Clone, Debug, CSSValue)]
#[value]
pub struct GradientColorStop(Color, Option<LengthPercentage>, Option<LengthPercentage>);
impl<C> From<C> for GradientColorStop
where
    C: Into<Color>,
{
    fn from(c: C) -> Self {
        Self(c.into(), None, None)
    }
}
impl<C, L> From<(C, L)> for GradientColorStop
where
    C: Into<Color>,
    L: Into<LengthPercentage>,
{
    fn from((c, l): (C, L)) -> Self {
        Self(c.into(), Some(l.into()), None)
    }
}
impl<C, L1, L2> From<(C, L1, L2)> for GradientColorStop
where
    C: Into<Color>,
    L1: Into<LengthPercentage>,
    L2: Into<LengthPercentage>,
{
    fn from((c, l1, l2): (C, L1, L2)) -> Self {
        Self(c.into(), Some(l1.into()), Some(l2.into()))
    }
}

#[derive(Clone, Debug, CSSValue)]
#[value(separator = ",")]
pub struct GradientColorStopHint(GradientColorStop, Option<LengthPercentage>);
impl<S> From<(S, Option<LengthPercentage>)> for GradientColorStopHint
where
    S: Into<GradientColorStop>,
{
    fn from((stop, hint): (S, Option<LengthPercentage>)) -> Self {
        Self(stop.into(), hint)
    }
}
impl<S, H> From<(S, H)> for GradientColorStopHint
where
    S: Into<GradientColorStop>,
    H: Into<LengthPercentage>,
{
    fn from((stop, hint): (S, H)) -> Self {
        Self::from((stop, Some(hint.into())))
    }
}

#[derive(Clone, Debug, CSSValue)]
#[value(separator = ",")]
pub struct GradientColorStops(pub Vec<GradientColorStopHint>, pub GradientColorStop);
impl GradientColorStops {
    pub fn build<SH, IT, S>(stops: IT, final_stop: S) -> Self
    where
        SH: Into<GradientColorStopHint>,
        IT: IntoIterator<Item = SH>,
        S: Into<GradientColorStop>,
    {
        Self(
            stops.into_iter().map(Into::into).collect(),
            final_stop.into(),
        )
    }
}

#[derive(Clone, Debug, CSSValue)]
pub enum GradientShapeSize {
    #[keyword]
    ClosestSide,
    #[keyword]
    ClosestCorner,
    #[keyword]
    FarthestSide,
    #[keyword]
    FarthestCorner,

    #[value]
    Size(LengthPercentage, Option<LengthPercentage>),
}

#[derive(Clone, Debug, CSSValue)]
pub enum GradientEndingShape {
    #[keyword]
    Circle,
    #[keyword]
    Ellipse,
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/gradient
#[derive(Clone, Debug, CSSValue)]
pub enum Gradient {
    /// side-or-corner (to left) isn't supported. Use angles instead.
    #[function(name = "linear-gradient")]
    Linear(Option<Angle>, GradientColorStops),

    #[function(name = "radial-gradient")]
    // TODO more tests
    Radial {
        shape: Option<GradientEndingShape>,
        size: Option<GradientShapeSize>,
        #[field(option, write_fn = "Self::radial_write_position")]
        position: Option<Position>,
        colors: GradientColorStops,
    },

    #[function(name = "repeating-gradient")]
    // TODO
    Repeating(),
}
impl Gradient {
    pub fn linear<IT, SH, S>(angle: Option<Angle>, stops: IT, final_stop: S) -> Self
    where
        IT: IntoIterator<Item = SH>,
        SH: Into<GradientColorStopHint>,
        S: Into<GradientColorStop>,
    {
        Self::Linear(angle, GradientColorStops::build(stops, final_stop))
    }

    pub fn radial_at<IT, SH, S>(position: Option<Position>, stops: IT, final_stop: S) -> Self
    where
        IT: IntoIterator<Item = SH>,
        SH: Into<GradientColorStopHint>,
        S: Into<GradientColorStop>,
    {
        Self::Radial {
            shape: None,
            size: None,
            position,
            colors: GradientColorStops::build(stops, final_stop),
        }
    }

    pub fn radial_size<IT, SH, S>(size: GradientShapeSize, stops: IT, final_stop: S) -> Self
    where
        IT: IntoIterator<Item = SH>,
        SH: Into<GradientColorStopHint>,
        S: Into<GradientColorStop>,
    {
        Self::Radial {
            shape: None,
            size: Some(size),
            position: None,
            colors: GradientColorStops::build(stops, final_stop),
        }
    }

    pub fn radial_ellipse_at<IT, SH, S>(
        position: Option<Position>,
        stops: IT,
        final_stop: S,
    ) -> Self
    where
        IT: IntoIterator<Item = SH>,
        SH: Into<GradientColorStopHint>,
        S: Into<GradientColorStop>,
    {
        Self::Radial {
            shape: Some(GradientEndingShape::Ellipse),
            size: None,
            position,
            colors: GradientColorStops::build(stops, final_stop),
        }
    }

    fn radial_write_position(f: &mut CSSWriter, position: &Position) -> WriteResult {
        f.write_str("at ")?;
        position.write_value(f)
    }
}

#[derive(Clone, Debug, CSSValue)]
pub enum ImageTags {
    #[keyword]
    Ltr,
    #[keyword]
    Rtl,
}

#[derive(Clone, Debug, CSSValue, FromVariants)]
pub enum ImageSrc {
    Url(Url),
    Str(CSSString),
}

#[derive(Clone, Debug, CSSValue)]
#[value]
pub struct CrossFadeMixingImage(Option<Percentage>, Image);
#[derive(Clone, Debug, CSSValue, FromVariants)]
pub enum CrossFadeFinalImage {
    Image(Image),
    Color(Color),
}

#[derive(Clone, Debug, CSSValue)]
#[value]
pub struct ImageSetOption(Image, Resolution);

// https://developer.mozilla.org/en-US/docs/Web/CSS/image
#[derive(Clone, Debug, CSSValue, FromVariants)]
pub enum Image {
    Url(Url),
    Gradient(Gradient),
    // TODO
    // Element(),
    #[function]
    Image(Option<ImageTags>, Vec<ImageSrc>, Option<Color>),
    #[function]
    CrossFade(Vec<CrossFadeMixingImage>, Option<Box<CrossFadeFinalImage>>),
    #[function]
    ImageSet(Vec<ImageSetOption>),
}
