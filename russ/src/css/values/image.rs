use super::{
    Angle, AnglePercentage, CSSString, Color, LengthPercentage, Percentage, Position, Resolution,
    Url,
};
use russ_internal::{CSSValue, FromVariants};

#[derive(Clone, Debug, CSSValue)]
#[value]
pub struct GradientColorStop<T>(Color, Option<T>, Option<T>);
impl<T> GradientColorStop<T> {
    pub fn build(color: Color, l1: Option<T>, l2: Option<T>) -> Option<Self> {
        if l1.is_none() && l2.is_some() {
            None
        } else {
            Some(Self(color, l1, l2))
        }
    }
}
impl<T, C> From<C> for GradientColorStop<T>
where
    C: Into<Color>,
{
    fn from(c: C) -> Self {
        Self(c.into(), None, None)
    }
}
impl<T, C, L> From<(C, L)> for GradientColorStop<T>
where
    C: Into<Color>,
    L: Into<T>,
{
    fn from((c, l): (C, L)) -> Self {
        Self(c.into(), Some(l.into()), None)
    }
}
impl<T, C, L1, L2> From<(C, L1, L2)> for GradientColorStop<T>
where
    C: Into<Color>,
    L1: Into<T>,
    L2: Into<T>,
{
    fn from((c, l1, l2): (C, L1, L2)) -> Self {
        Self(c.into(), Some(l1.into()), Some(l2.into()))
    }
}

#[derive(Clone, Debug, CSSValue)]
#[value(separator = ",")]
pub struct GradientColorStopHint<T>(GradientColorStop<T>, Option<T>);
impl<T> GradientColorStopHint<T> {
    pub fn new(stop: GradientColorStop<T>, hint: Option<T>) -> Self {
        Self(stop, hint)
    }

    pub fn stop(stop: impl Into<GradientColorStop<T>>) -> Self {
        Self::new(stop.into(), None)
    }

    pub fn hint(stop: impl Into<GradientColorStop<T>>, hint: impl Into<T>) -> Self {
        Self::new(stop.into(), Some(hint.into()))
    }
}
impl<T, S> From<S> for GradientColorStopHint<T>
where
    S: Into<GradientColorStop<T>>,
{
    fn from(stop: S) -> Self {
        Self::stop(stop)
    }
}

#[derive(Clone, Debug, CSSValue)]
#[value(separator = ",")]
pub struct GradientColorStopList<T>(pub Vec<GradientColorStopHint<T>>, pub GradientColorStop<T>);
impl<T> GradientColorStopList<T> {
    pub fn build<SH, IT, S>(stops: IT, final_stop: S) -> Self
    where
        SH: Into<GradientColorStopHint<T>>,
        IT: IntoIterator<Item = SH>,
        S: Into<GradientColorStop<T>>,
    {
        Self(
            stops.into_iter().map(Into::into).collect(),
            final_stop.into(),
        )
    }
}

pub type LinearColorStop = GradientColorStop<LengthPercentage>;
pub type LinearColorStopHint = GradientColorStopHint<LengthPercentage>;
pub type LinearColorStopList = GradientColorStopList<LengthPercentage>;

pub type AngularColorStop = GradientColorStop<AnglePercentage>;
pub type AngularColorStopHint = GradientColorStopHint<AnglePercentage>;
pub type AngularColorStopList = GradientColorStopList<AnglePercentage>;

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
impl<L> From<L> for GradientShapeSize
where
    L: Into<LengthPercentage>,
{
    fn from(l: L) -> Self {
        Self::Size(l.into(), None)
    }
}
impl<L1, L2> From<(L1, L2)> for GradientShapeSize
where
    L1: Into<LengthPercentage>,
    L2: Into<LengthPercentage>,
{
    fn from((l1, l2): (L1, L2)) -> Self {
        Self::Size(l1.into(), Some(l2.into()))
    }
}

#[derive(Clone, Debug, CSSValue)]
pub enum GradientEndingShape {
    #[keyword]
    Circle,
    #[keyword]
    Ellipse,
}

#[derive(Clone, Debug, CSSValue)]
#[value]
// at least one of the three MUST be specified
pub struct GradientRadialDefinition {
    shape: Option<GradientEndingShape>,
    size: Option<GradientShapeSize>,
    #[field(option, prefix = "at ")]
    position: Option<Position>,
}
impl GradientRadialDefinition {
    pub fn build(
        shape: Option<GradientEndingShape>,
        size: Option<GradientShapeSize>,
        position: Option<Position>,
    ) -> Option<Self> {
        if matches!((&shape, &size, &position), (None, None, None)) {
            None
        } else {
            Some(Self {
                shape,
                size,
                position,
            })
        }
    }
}

#[derive(Clone, Debug, CSSValue)]
#[value]
// at least one value must not be None
pub struct GradientConicDefinition {
    #[field(option, prefix = "from ")]
    from: Option<Angle>,
    #[field(option, prefix = "at ")]
    at: Option<Position>,
}
impl GradientConicDefinition {
    pub fn build(from: Option<Angle>, at: Option<Position>) -> Option<Self> {
        if matches!((&from, &at), (None, None)) {
            None
        } else {
            Some(Self { from, at })
        }
    }
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/gradient
#[derive(Clone, Debug, CSSValue)]
pub enum Gradient {
    /// side-or-corner (to left) isn't supported. Use angles instead.
    #[function(name = "linear-gradient")]
    Linear(Option<Angle>, LinearColorStopList),

    #[function(name = "radial-gradient")]
    Radial(Option<GradientRadialDefinition>, LinearColorStopList),

    #[function(name = "conic-gradient")]
    Conic(Option<GradientConicDefinition>, AngularColorStopList),
    // TODO add support for repeating gradients which is just the same 3 again but the function name is "repeating-<name>"
}
impl Gradient {
    pub fn linear<IT, SH, S>(angle: Option<Angle>, stops: IT, final_stop: S) -> Self
    where
        IT: IntoIterator<Item = SH>,
        SH: Into<LinearColorStopHint>,
        S: Into<LinearColorStop>,
    {
        Self::Linear(angle, LinearColorStopList::build(stops, final_stop))
    }

    pub fn radial<IT, SH, S>(
        size: Option<GradientShapeSize>,
        position: Option<Position>,
        stops: IT,
        final_stop: S,
    ) -> Self
    where
        IT: IntoIterator<Item = SH>,
        SH: Into<LinearColorStopHint>,
        S: Into<LinearColorStop>,
    {
        Self::Radial(
            GradientRadialDefinition::build(None, size, position),
            LinearColorStopList::build(stops, final_stop),
        )
    }

    pub fn radial_ellipse<IT, SH, S>(
        size: Option<GradientShapeSize>,
        position: Option<Position>,
        stops: IT,
        final_stop: S,
    ) -> Self
    where
        IT: IntoIterator<Item = SH>,
        SH: Into<LinearColorStopHint>,
        S: Into<LinearColorStop>,
    {
        Self::Radial(
            GradientRadialDefinition::build(Some(GradientEndingShape::Ellipse), size, position),
            LinearColorStopList::build(stops, final_stop),
        )
    }

    pub fn conic<IT, SH, S>(
        from: Option<Angle>,
        at: Option<Position>,
        stops: IT,
        final_stop: S,
    ) -> Self
    where
        IT: IntoIterator<Item = SH>,
        SH: Into<AngularColorStopHint>,
        S: Into<AngularColorStop>,
    {
        Self::Conic(
            GradientConicDefinition::build(from, at),
            AngularColorStopList::build(stops, final_stop),
        )
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

// TODO test image

// https://developer.mozilla.org/en-US/docs/Web/CSS/image
#[derive(Clone, Debug, CSSValue, FromVariants)]
pub enum Image {
    Url(Url),
    Gradient(Box<Gradient>),
    // TODO
    // Element(),
    #[function]
    Image(Option<ImageTags>, Vec<ImageSrc>, Option<Color>),
    #[function]
    CrossFade(Vec<CrossFadeMixingImage>, Option<Box<CrossFadeFinalImage>>),
    #[function]
    ImageSet(Vec<ImageSetOption>),
}
