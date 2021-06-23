use serde::*;
use crate::BorderValues;
use crate::borders::styles::*;
use crate::colour::*;
use crate::size::*;

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderRightColour(Colour);

impl From<Colour> for BorderRightColour {
    fn from(colour: Colour) -> Self {
        Self(colour)
    }
}

impl Into<Colour> for &BorderRightColour {
    fn into(self) -> Colour {
        self.0
    }
}

impl zodiac::PropertySet<Colour> for BorderRightColour {
    fn set(&mut self, to_set: Colour) {
        self.0 = to_set;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderRightStyle(BorderStyles);

impl Default for BorderRightStyle {
    fn default() -> Self {
        Self(BorderStyles::None)
    }
}

impl zodiac::PropertySet<BorderStyles> for BorderRightStyle {
    fn set(&mut self, to_set: BorderStyles) {
        self.0 = to_set;
    }
}

impl From<BorderStyles> for BorderRightStyle {
    fn from(styles: BorderStyles) -> Self {
        Self(styles)
    }
}

impl Into<BorderStyles> for &BorderRightStyle {
    fn into(self) -> BorderStyles {
        self.0
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderRightWidth(pub Size);

impl From<Size> for BorderRightWidth {
    fn from(size: Size) -> Self {
        Self(size)
    }
}

impl zodiac::PropertySet<Size> for BorderRightWidth {
    fn set(&mut self, to_set: Size) {
        self.0 = to_set;
    }
}

impl Into<Size> for &BorderRightWidth {
    fn into(self) -> Size {
        self.0
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderRight(BorderValues, bool);

impl zodiac::PropertySetCheck for BorderRight {
    fn is_set(&self) -> bool {
        self.1
    }
}

impl zodiac::PropertySet<BorderValues> for BorderRight {
    fn set(&mut self, to_set: BorderValues) {
        self.0 = to_set;
        self.1 = true;
    }
}

impl From<BorderValues> for BorderRight {
    fn from(side: BorderValues) -> Self {
        Self(side, true)
    }
}

impl Into<(Size, BorderStyles, Colour)> for BorderRight {
    fn into(self) -> (Size, BorderStyles, Colour) {
        self.0.into()
    }
}

impl From<(&BorderRightWidth, &BorderRightStyle, &BorderRightColour)> for BorderRight {
    fn from(props: (&BorderRightWidth, &BorderRightStyle, &BorderRightColour)) -> Self {
        Self((props.0.0, props.1.0, props.2.0).into(), true)
    }
}
