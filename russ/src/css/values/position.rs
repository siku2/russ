use super::LengthPercentage;
use russ_internal::{CSSValue, FromVariants};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, CSSValue)]
pub enum PositionHorizontalAnchor {
    #[keyword]
    Left,
    #[keyword]
    Right,
}
#[derive(Clone, Debug, Eq, Hash, PartialEq, CSSValue, FromVariants)]
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, CSSValue)]
pub enum PositionVerticalAnchor {
    #[keyword]
    Top,
    #[keyword]
    Bottom,
}
#[derive(Clone, Debug, Eq, Hash, PartialEq, CSSValue, FromVariants)]
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
#[derive(Clone, Debug, Eq, Hash, PartialEq, CSSValue)]
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
