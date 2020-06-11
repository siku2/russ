use super::{Angle, CSSString, Color, LengthPercentage, Percentage, Resolution, Url};
use russ_internal::{CSSValue, FromVariants};

#[derive(Clone, Debug, CSSValue)]
#[value]
pub struct LinearColorStop(Color, Option<LengthPercentage>, Option<LengthPercentage>);
impl<C> From<C> for LinearColorStop
where
    C: Into<Color>,
{
    fn from(c: C) -> Self {
        Self(c.into(), None, None)
    }
}
impl<C, L> From<(C, L)> for LinearColorStop
where
    C: Into<Color>,
    L: Into<LengthPercentage>,
{
    fn from((c, l): (C, L)) -> Self {
        Self(c.into(), Some(l.into()), None)
    }
}
impl<C, L1, L2> From<(C, L1, L2)> for LinearColorStop
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
pub struct LinearColorStopWithColorHint(LinearColorStop, Option<LengthPercentage>);
impl<S> From<(S, Option<LengthPercentage>)> for LinearColorStopWithColorHint
where
    S: Into<LinearColorStop>,
{
    fn from((stop, hint): (S, Option<LengthPercentage>)) -> Self {
        Self(stop.into(), hint)
    }
}
impl<S, H> From<(S, H)> for LinearColorStopWithColorHint
where
    S: Into<LinearColorStop>,
    H: Into<LengthPercentage>,
{
    fn from((stop, hint): (S, H)) -> Self {
        Self::from((stop, Some(hint.into())))
    }
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/gradient
#[derive(Clone, Debug, CSSValue)]
pub enum Gradient {
    /// side-or-corner (to left) isn't supported. Use angles instead.
    #[function(name = "linear-gradient")]
    Linear(
        Option<Angle>,
        Vec<LinearColorStopWithColorHint>,
        LinearColorStop,
    ),
    #[function(name = "radial-gradient")]
    Radial(),
    #[function(name = "repeating-gradient")]
    Repeating(),
}
impl Gradient {
    pub fn linear<LCH, L>(angle: Option<Angle>, mut stops: Vec<LCH>, final_stop: L) -> Self
    where
        LCH: Into<LinearColorStopWithColorHint>,
        L: Into<LinearColorStop>,
    {
        Self::Linear(
            angle,
            stops.drain(..).map(Into::into).collect(),
            final_stop.into(),
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
