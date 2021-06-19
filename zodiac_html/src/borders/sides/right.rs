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
