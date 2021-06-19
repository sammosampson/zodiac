use serde::*;
use crate::borders::styles::*;
use crate::colour::*;
use crate::size::*;

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderLeftColour(Colour);

impl From<Colour> for BorderLeftColour {
    fn from(colour: Colour) -> Self {
        Self(colour)
    }
}

impl Into<Colour> for &BorderLeftColour {
    fn into(self) -> Colour {
        self.0
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

impl Into<BorderStyles> for &BorderLeftStyle {
    fn into(self) -> BorderStyles {
        self.0
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderLeftWidth(pub Size);

impl From<Size> for BorderLeftWidth {
    fn from(size: Size) -> Self {
        Self(size)
    }
}

impl Into<Size> for &BorderLeftWidth {
    fn into(self) -> Size {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderLeft {
    pub width: Size,
    pub style: BorderStyles,
    pub colour: Colour,
    is_set: bool
}


impl zodiac::PropertySetCheck for BorderLeft {
    fn is_set(&self) -> bool {
        self.is_set
    }
}

impl Default for BorderLeft {
    fn default() -> Self {
        Self {
            width: Size::default(),
            style:BorderStyles::None,
            colour: Colour::default(),
            is_set: false
        }
    }
}

impl From<(Size, BorderStyles, Colour)> for BorderLeft {
    fn from(props: (Size, BorderStyles, Colour)) -> Self {
        Self {
            width: props.0,
            style: props.1,
            colour: props.2,
            is_set: true
        }
    }
}

impl Into<(Size, BorderStyles, Colour)> for BorderLeft {
    fn into(self) -> (Size, BorderStyles, Colour) {
        (self.width, self.style, self.colour)
    }
}

impl From<(&BorderLeftWidth, &BorderLeftStyle, &BorderLeftColour)> for BorderLeft {
    fn from(props: (&BorderLeftWidth, &BorderLeftStyle, &BorderLeftColour)) -> Self {
        Self {
            width: props.0.0,
            style: props.1.0,
            colour: props.2.0,
            is_set: true
        }
    }
}
