use serde::*;
use crate::BorderValues;
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

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderLeft(BorderValues, bool);

impl zodiac::PropertySetCheck for BorderLeft {
    fn is_set(&self) -> bool {
        self.1
    }
}

impl From<BorderValues> for BorderLeft {
    fn from(side: BorderValues) -> Self {
        Self(side, true)
    }
}

impl Into<(Size, BorderStyles, Colour)> for BorderLeft {
    fn into(self) -> (Size, BorderStyles, Colour) {
        self.0.into()
    }
}

impl From<(&BorderLeftWidth, &BorderLeftStyle, &BorderLeftColour)> for BorderLeft {
    fn from(props: (&BorderLeftWidth, &BorderLeftStyle, &BorderLeftColour)) -> Self {
        Self((props.0.0, props.1.0, props.2.0).into(), true)
    }
}
