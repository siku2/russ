use super::types::{Color, Image, LengthPercentage, Position};
use russ_css::{CSSDeclaration, CSSValue};

#[derive(CSSValue)]
pub enum GlobalValue {
    #[keyword]
    Inherit,
    #[keyword]
    Initial,
    #[keyword]
    Unset,
}

// #[derive(CSSValue)]
pub struct BackgroundLayer {
    image: Option<BackgroundImage>,
    position: Option<BackgroundPosition>,
    size: Option<BackgroundSize>,
    repeat_style: Option<BackgroundRepeatStyle>,
    attachment: Option<BackgroundAttachment>,
    origin: Option<BackgroundOrigin>,
    clip: Option<BackgroundClip>,
}

#[derive(CSSValue)]
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
pub struct BackgroundImage(Vec<Option<Image>>);

#[derive(CSSValue)]
pub enum BackgroundOriginItem {
    BorderBox,
    PaddingBox,
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
    XY(BackgroundSizeItemValue, Option<BackgroundSizeItemValue>),
    #[keyword]
    Cover,
    #[keyword]
    Contain,
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/background-size
#[derive(CSSDeclaration, CSSValue)]
pub struct BackgroundSize(Vec<BackgroundSizeItem>);
