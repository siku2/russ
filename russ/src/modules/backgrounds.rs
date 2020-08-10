/// <https://drafts.csswg.org/css-backgrounds-3>
use super::{values::LengthPercentage, Multiple, OneToFour};
use russ_internal::{CssDeclaration, CssValue, CssWriter, FromVariants, WriteResult, WriteValue};

//TODO redo

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, CssValue)]
#[value]
pub struct BackgroundLayer {
    image: Option<BackgroundImage>,
    // TODO combine position / size into a single value.
    position: Option<BackgroundPosition>,
    #[field(option, prefix = "/ ")]
    size: Option<BackgroundSize>,
    repeat_style: Option<BackgroundRepeatStyle>,
    attachment: Option<BackgroundAttachment>,
    origin: Option<BackgroundOrigin>,
    clip: Option<BackgroundClip>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, CssValue)]
#[value]
pub struct BackgroundLayerFinal {
    pub layer: BackgroundLayer,
    pub color: BackgroundColor,
}

/// <https://developer.mozilla.org/en-US/docs/Web/CSS/background>
#[derive(Clone, Debug, Eq, Hash, PartialEq, CssDeclaration, CssValue)]
#[value]
pub struct Background(pub Vec<BackgroundLayer>, pub BackgroundLayerFinal);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, CssValue)]
pub enum Attachment {
    #[keyword]
    Fixed,
    #[keyword]
    Local,
    #[keyword]
    Scroll,
}

/// <https://developer.mozilla.org/en-US/docs/Web/CSS/background-color>
#[derive(Clone, Debug, Eq, Hash, PartialEq, CssDeclaration, CssValue)]
pub struct BackgroundColor(pub Color);

/// <https://developer.mozilla.org/en-US/docs/Web/CSS/background-attachment>
#[derive(Clone, Debug, Eq, Hash, PartialEq, CssDeclaration, CssValue)]
pub struct BackgroundAttachment(pub Multiple<Attachment>);

/// <https://drafts.csswg.org/css-backgrounds-4/#typedef-box>
#[derive(Clone, Debug, Eq, Hash, PartialEq, CssValue, FromVariants)]
pub enum BackgroundClipItem {
    Box(CssBox),
    // still experimental
    #[keyword]
    Border,
    #[keyword]
    Text,
}

/// <https://developer.mozilla.org/en-US/docs/Web/CSS/background-clip>
#[derive(Clone, Debug, Eq, Hash, PartialEq, CssDeclaration, CssValue)]
pub struct BackgroundClip(pub Multiple<BackgroundClipItem>);

/// <https://developer.mozilla.org/en-US/docs/Web/CSS/background-image>
#[derive(Clone, Debug, Eq, Hash, PartialEq, CssDeclaration, CssValue)]
pub struct BackgroundImage(#[field(write_fn = "Self::write_images")] pub Multiple<Option<Image>>);
impl BackgroundImage {
    fn write_image(f: &mut CssWriter, img: &Option<Image>) -> WriteResult {
        if let Some(img) = img {
            img.write_value(f)
        } else {
            f.write_str("none")
        }
    }

    fn write_images(f: &mut CssWriter, images: &[Option<Image>]) -> WriteResult {
        if let Some((first, others)) = images.split_first() {
            Self::write_image(f, first)?;
            for v in others {
                f.write_char(',')?;
                Self::write_image(f, v)?;
            }
        }
        Ok(())
    }
}

/// <https://developer.mozilla.org/en-US/docs/Web/CSS/background-origin>
#[derive(Clone, Debug, Eq, Hash, PartialEq, CssDeclaration, CssValue)]
pub struct BackgroundOrigin(pub Multiple<CssBox>);

/// <https://developer.mozilla.org/en-US/docs/Web/CSS/background-position>
#[derive(Clone, Debug, Eq, Hash, PartialEq, CssDeclaration, CssValue)]
pub struct BackgroundPosition(pub Multiple<Position>);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, CssValue)]
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

#[derive(Clone, Debug, Eq, Hash, PartialEq, CssValue)]
pub enum BackgroundRepeatItem {
    #[value]
    XY(BackgroundRepeatStyle, Option<BackgroundRepeatStyle>),
    #[keyword]
    RepeatX,
    #[keyword]
    RepeatY,
}

/// <https://developer.mozilla.org/en-US/docs/Web/CSS/background-repeat>
#[derive(Clone, Debug, Eq, Hash, PartialEq, CssDeclaration, CssValue)]
pub struct BackgroundRepeat(pub Multiple<BackgroundRepeatItem>);

#[derive(Clone, Debug, Eq, Hash, PartialEq, CssValue)]
pub enum BackgroundSizeItemValue {
    LengthPercentage(LengthPercentage),
    #[keyword]
    Auto,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, CssValue)]
pub enum BackgroundSizeItem {
    #[value]
    XY(BackgroundSizeItemValue, Option<BackgroundSizeItemValue>),
    #[keyword]
    Cover,
    #[keyword]
    Contain,
}

/// <https://developer.mozilla.org/en-US/docs/Web/CSS/background-size>
#[derive(Clone, Debug, Eq, Hash, PartialEq, CssDeclaration, CssValue)]
pub struct BackgroundSize(pub Multiple<BackgroundSizeItem>);

/// <https://drafts.csswg.org/css-backgrounds-3/#propdef-border-color>
#[derive(Clone, Debug, Eq, Hash, PartialEq, CssDeclaration, CssValue)]
pub struct BorderTopColor(pub Color);
/// <https://drafts.csswg.org/css-backgrounds-3/#propdef-border-color>
#[derive(Clone, Debug, Eq, Hash, PartialEq, CssDeclaration, CssValue)]
pub struct BorderRightColor(pub Color);
/// <https://drafts.csswg.org/css-backgrounds-3/#propdef-border-color>
#[derive(Clone, Debug, Eq, Hash, PartialEq, CssDeclaration, CssValue)]
pub struct BorderBottomColor(pub Color);
/// <https://drafts.csswg.org/css-backgrounds-3/#propdef-border-color>
#[derive(Clone, Debug, Eq, Hash, PartialEq, CssDeclaration, CssValue)]
pub struct BorderLeftColor(pub Color);
/// <https://drafts.csswg.org/css-backgrounds-3/#propdef-border-color>
#[derive(Clone, Debug, Eq, Hash, PartialEq, CssDeclaration, CssValue)]
pub struct BorderColor(pub OneToFour<Color>);

/// <https://drafts.csswg.org/css-backgrounds-3/#typedef-line-style>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, CssValue)]
pub enum LineStyle {
    #[keyword]
    None,
    #[keyword]
    Hidden,
    #[keyword]
    Dotted,
    #[keyword]
    Dashed,
    #[keyword]
    Solid,
    #[keyword]
    Double,
    #[keyword]
    Groove,
    #[keyword]
    Ridge,
    #[keyword]
    Inset,
    #[keyword]
    Outset,
}

/// <https://drafts.csswg.org/css-backgrounds-3/#propdef-border-style>
#[derive(Clone, Debug, Eq, Hash, PartialEq, CssDeclaration, CssValue)]
pub struct BorderTopStyle(pub Color);
/// <https://drafts.csswg.org/css-backgrounds-3/#propdef-border-style>
#[derive(Clone, Debug, Eq, Hash, PartialEq, CssDeclaration, CssValue)]
pub struct BorderRightStyle(pub Color);
/// <https://drafts.csswg.org/css-backgrounds-3/#propdef-border-style>
#[derive(Clone, Debug, Eq, Hash, PartialEq, CssDeclaration, CssValue)]
pub struct BorderBottomStyle(pub Color);
/// <https://drafts.csswg.org/css-backgrounds-3/#propdef-border-style>
#[derive(Clone, Debug, Eq, Hash, PartialEq, CssDeclaration, CssValue)]
pub struct BorderLeftStyle(pub Color);
/// <https://drafts.csswg.org/css-backgrounds-3/#propdef-border-style>
#[derive(Clone, Debug, Eq, Hash, PartialEq, CssValue)]
pub struct BorderStyle(pub OneToFour<LineStyle>);
