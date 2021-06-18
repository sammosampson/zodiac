use serde::*;
use crate::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub enum BorderStyles {
    None,
    Dotted,
    Dashed,
    Solid,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
    Hidden
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderTopStyle(BorderStyles);

impl Default for BorderTopStyle {
    fn default() -> Self {
        Self(BorderStyles::None)
    }
}

impl From<BorderStyles> for BorderTopStyle {
    fn from(styles: BorderStyles) -> Self {
        Self(styles)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderBottomStyle(BorderStyles);

impl Default for BorderBottomStyle {
    fn default() -> Self {
        Self(BorderStyles::None)
    }
}

impl From<BorderStyles> for BorderBottomStyle {
    fn from(styles: BorderStyles) -> Self {
        Self(styles)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderLeftStyle(BorderStyles);

impl Default for BorderLeftStyle {
    fn default() -> Self {
        Self(BorderStyles::None)
    }
}

impl From<BorderStyles> for BorderLeftStyle {
    fn from(styles: BorderStyles) -> Self {
        Self(styles)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderRightStyle(BorderStyles);

impl Default for BorderRightStyle {
    fn default() -> Self {
        Self(BorderStyles::None)
    }
}

impl From<BorderStyles> for BorderRightStyle {
    fn from(styles: BorderStyles) -> Self {
        Self(styles)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderStyle {
    pub top: BorderStyles,
    pub left: BorderStyles,
    pub bottom: BorderStyles,
    pub right: BorderStyles,
    is_set: bool
}

impl Default for BorderStyle {
    fn default() -> Self {
        Self {
            top: BorderStyles::None,
            left: BorderStyles::None,
            bottom: BorderStyles::None,
            right: BorderStyles::None,
            is_set: false
        }
    }
}

impl From<BorderStyles> for BorderStyle {
    fn from(styles: BorderStyles) -> Self {
        Self {
            top: styles,
            left: styles,
            bottom: styles,
            right: styles,
            is_set: true
        }
    }
}

impl zodiac::PropertySetCheck for BorderStyle {
    fn is_set(&self) -> bool {
        self.is_set
    }
}

impl From<(&BorderTopStyle, &BorderLeftStyle, &BorderBottomStyle, &BorderRightStyle)> for BorderStyle {
    fn from(props: (&BorderTopStyle, &BorderLeftStyle, &BorderBottomStyle, &BorderRightStyle)) -> Self {
        Self {
            top: props.0.0,
            left: props.1.0,
            bottom: props.2.0,
            right: props.3.0,
            is_set: true
        }
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderRadius {
    pub top_left: Size,
    pub top_right: Size,
    pub bottom_left: Size,
    pub bottom_right: Size,
    is_set: bool
}

impl From<Size> for BorderRadius {
    fn from(size: Size) -> Self {
        Self {
            top_left: size,
            top_right: size,
            bottom_left: size,
            bottom_right: size,
            is_set: true
        }
    }
}

impl zodiac::PropertySetCheck for BorderRadius {
    fn is_set(&self) -> bool {
        self.is_set
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderWidth(Size);

impl BorderWidth {
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl From<Size> for BorderWidth {
    fn from(size: Size) -> Self {
        Self(size)
    }
}

impl Into<f32> for BorderWidth {
    fn into(self) -> f32 {
        self.0.into()
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderColour(pub Colour);

impl From<Colour> for BorderColour {
    fn from(colour: Colour) -> Self {
        Self(colour)
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Border {
    pub colour: Colour,
    pub width: Size,
    pub style: BorderStyle,
    pub radius: BorderRadius,
    is_set: bool
}

impl zodiac::PropertySetCheck for Border {
    fn is_set(&self) -> bool {
        self.is_set
    }
}

impl From<(Colour, Size, BorderStyles, Size)> for Border {
    fn from(props: (Colour, Size, BorderStyles, Size)) -> Self {
        Self {
            colour: props.0,
            width: props.1,
            style: BorderStyle::from(props.2),
            radius: BorderRadius::from(props.3),
            is_set: true
        }
    }
}

impl From<(&BorderColour, &BorderWidth, &BorderStyle, &BorderRadius)> for Border {
    fn from(props: (&BorderColour, &BorderWidth, &BorderStyle, &BorderRadius)) -> Self {
        Self {
            colour: props.0.0,
            width: props.1.0,
            style: *props.2,
            radius: *props.3,
            is_set: true
        }
    }
}