use serde::*;
use crate::borders::styles::*;
use crate::colour::*;
use crate::size::*;

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderBottomColour(Colour);

impl From<Colour> for BorderBottomColour {
    fn from(colour: Colour) -> Self {
        Self(colour)
    }
}

impl Into<Colour> for &BorderBottomColour {
    fn into(self) -> Colour {
        self.0
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

impl Into<BorderStyles> for &BorderBottomStyle {
    fn into(self) -> BorderStyles {
        self.0
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderBottomWidth(Size);

impl From<Size> for BorderBottomWidth {
    fn from(size: Size) -> Self {
        Self(size)
    }
}

impl Into<Size> for &BorderBottomWidth {
    fn into(self) -> Size {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderBottom {
    pub width: Size,
    pub style: BorderStyles,
    pub colour: Colour,
    is_set: bool
}

impl Default for BorderBottom {
    fn default() -> Self {
        Self {
            width: Size::default(),
            style:BorderStyles::None,
            colour: Colour::default(),
            is_set: false
        }
    }
}


impl zodiac::PropertySetCheck for BorderBottom {
    fn is_set(&self) -> bool {
        self.is_set
    }
}

impl From<(Size, BorderStyles, Colour)> for BorderBottom {
    fn from(props: (Size, BorderStyles, Colour)) -> Self {
        Self {
            width: props.0,
            style: props.1,
            colour: props.2,
            is_set: true
        }
    }
}

impl Into<(Size, BorderStyles, Colour)> for BorderBottom {
    fn into(self) -> (Size, BorderStyles, Colour) {
        (self.width, self.style, self.colour)
    }
}

impl From<(&BorderBottomWidth, &BorderBottomStyle, &BorderBottomColour)> for BorderBottom {
    fn from(props: (&BorderBottomWidth, &BorderBottomStyle, &BorderBottomColour)) -> Self {
        Self {
            width: props.0.0,
            style: props.1.0,
            colour: props.2.0,
            is_set: true
        }
    }
}
