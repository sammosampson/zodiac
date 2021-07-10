use serde::*;
use crate::BorderValues;
use crate::layout::*;
use crate::borders::styles::*;
use crate::colour::*;
use crate::size::*;

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderLeftColour(Colour, bool);

impl From<Colour> for BorderLeftColour {
    fn from(colour: Colour) -> Self {
        Self(colour, true)
    }
}

impl Into<Colour> for &BorderLeftColour {
    fn into(self) -> Colour {
        self.0
    }
}

impl zodiac::PropertySet<Colour> for BorderLeftColour {
    fn set(&mut self, to_set: Colour) {
        self.0 = to_set;
        self.1 = true;
    }
}

impl zodiac::PropertySetCheck for BorderLeftColour {
    fn is_set(&self) -> bool {
        self.1
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderLeftStyle(BorderStyles, bool);

impl zodiac::PropertySet<BorderStyles> for BorderLeftStyle {
    fn set(&mut self, to_set: BorderStyles) {
        self.0 = to_set;
        self.1 = true;
    }
}

impl zodiac::PropertySetCheck for BorderLeftStyle {
    fn is_set(&self) -> bool {
        self.1
    }
}

impl From<BorderStyles> for BorderLeftStyle {
    fn from(styles: BorderStyles) -> Self {
        Self(styles, true)
    }
}

impl Into<BorderStyles> for &BorderLeftStyle {
    fn into(self) -> BorderStyles {
        self.0
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderLeftWidth(Size, bool);

impl From<Size> for BorderLeftWidth {
    fn from(size: Size) -> Self {
        Self(size, true)
    }
}

impl zodiac::PropertySet<Size> for BorderLeftWidth {
    fn set(&mut self, to_set: Size) {
        self.0 = to_set;
        self.1 = true;
    }
}

impl zodiac::PropertySetCheck for BorderLeftWidth {
    fn is_set(&self) -> bool {
        self.1
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

impl zodiac::PropertySet<BorderValues> for BorderLeft {
    fn set(&mut self, to_set: BorderValues) {
        self.0 = to_set;
        self.1 = true;
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

impl Into<LayoutDistance> for BorderLeft {
    fn into(self) -> LayoutDistance {
        self.0.into()
    }
}

impl From<(&BorderLeftWidth, &BorderLeftStyle, &BorderLeftColour)> for BorderLeft {
    fn from(props: (&BorderLeftWidth, &BorderLeftStyle, &BorderLeftColour)) -> Self {
        Self((props.0.0, props.1.0, props.2.0).into(), true)
    }
}
