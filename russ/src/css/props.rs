use super::values::{Color, Image, LengthPercentage, Position};
use russ_internal::{CSSDeclaration, CSSValue};

#[derive(CSSValue)]
pub enum GlobalValue {
    #[keyword]
    Inherit,
    #[keyword]
    Initial,
    #[keyword]
    Unset,
}

#[derive(CSSValue)]
#[value]
pub struct BackgroundLayer {
    image: Option<BackgroundImage>,
    position: Option<BackgroundPosition>,
    size: Option<BackgroundSize>,
    // TODO pretty sure there was something about a slash here
    repeat_style: Option<BackgroundRepeatStyle>,
    attachment: Option<BackgroundAttachment>,
    origin: Option<BackgroundOrigin>,
    clip: Option<BackgroundClip>,
}

#[derive(CSSValue)]
#[value]
pub struct BackgroundLayerFinal {
    layer: BackgroundLayer,
    color: BackgroundColor,
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/background
// #[derive(CSSDeclaration, CSSValue)]
pub struct Background(Vec<BackgroundLayer>, BackgroundLayerFinal);

#[derive(CSSValue)]
pub enum Attachment {
    #[keyword]
    Fixed,
    #[keyword]
    Local,
    #[keyword]
    Scroll,
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/background-color
#[derive(CSSDeclaration, CSSValue)]
pub struct BackgroundColor(Color);

// https://developer.mozilla.org/en-US/docs/Web/CSS/background-attachment
#[derive(CSSDeclaration, CSSValue)]
pub struct BackgroundAttachment(Vec<Attachment>);

#[derive(CSSValue)]
pub enum BackgroundClipItem {
    #[keyword]
    BorderBox,
    #[keyword]
    PaddingBox,
    #[keyword]
    ContentBox,
    // still experimental
    #[keyword]
    Border,
    #[keyword]
    Text,
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/background-clip
#[derive(CSSDeclaration, CSSValue)]
pub struct BackgroundClip(Vec<BackgroundClipItem>);

// https://developer.mozilla.org/en-US/docs/Web/CSS/background-image
#[derive(CSSDeclaration, CSSValue)]
pub struct BackgroundImage(#[field(iter_option)] Vec<Option<Image>>);

#[derive(CSSValue)]
pub enum BackgroundOriginItem {
    #[keyword]
    BorderBox,
    #[keyword]
    PaddingBox,
    #[keyword]
    ContentBox,
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/background-origin
#[derive(CSSDeclaration, CSSValue)]
pub struct BackgroundOrigin(Vec<BackgroundOriginItem>);

// https://developer.mozilla.org/en-US/docs/Web/CSS/background-position
#[derive(CSSDeclaration, CSSValue)]
pub struct BackgroundPosition(Vec<Position>);

#[derive(CSSValue)]
pub enum BackgroundRepeatStyle {
    #[keyword]
    Repeat,
    #[keyword]
    Space,
    #[keyword]
    Round,
    #[keyword]
    NoRepeat,
}

#[derive(CSSValue)]
pub enum BackgroundRepeatItem {
    #[value]
    XY(BackgroundRepeatStyle, Option<BackgroundRepeatStyle>),
    #[keyword]
    RepeatX,
    #[keyword]
    RepeatY,
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/background-repeat
#[derive(CSSDeclaration, CSSValue)]
pub struct BackgroundRepeat(Vec<BackgroundRepeatItem>);

#[derive(CSSValue)]
pub enum BackgroundSizeItemValue {
    LengthPercentage(LengthPercentage),
    #[keyword]
    Auto,
}

#[derive(CSSValue)]
pub enum BackgroundSizeItem {
    #[value]
    XY(BackgroundSizeItemValue, Option<BackgroundSizeItemValue>),
    #[keyword]
    Cover,
    #[keyword]
    Contain,
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/background-size
#[derive(CSSDeclaration, CSSValue)]
pub struct BackgroundSize(Vec<BackgroundSizeItem>);
