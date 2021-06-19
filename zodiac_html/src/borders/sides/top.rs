use serde::*;
use crate::BorderValues;
use crate::borders::styles::*;
use crate::colour::*;
use crate::size::*;

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderTopColour(pub Colour);

impl From<Colour> for BorderTopColour {
    fn from(colour: Colour) -> Self {
        Self(colour)
    }
}

impl Into<Colour> for &BorderTopColour {
    fn into(self) -> Colour {
        self.0
    }
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

impl Into<BorderStyles> for &BorderTopStyle {
    fn into(self) -> BorderStyles {
        self.0
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderTopWidth(Size);

impl From<Size> for BorderTopWidth {
    fn from(size: Size) -> Self {
        Self(size)
    }
}

impl Into<Size> for &BorderTopWidth {
    fn into(self) -> Size {
        self.0
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderTop(BorderValues, bool);

impl zodiac::PropertySetCheck for BorderTop {
    fn is_set(&self) -> bool {
        self.1
    }
}

impl From<BorderValues> for BorderTop {
    fn from(side: BorderValues) -> Self {
        Self(side, true)
    }
}

impl Into<(Size, BorderStyles, Colour)> for BorderTop {
    fn into(self) -> (Size, BorderStyles, Colour) {
        self.0.into()
    }
}

impl From<(&BorderTopWidth, &BorderTopStyle, &BorderTopColour)> for BorderTop {
    fn from(props: (&BorderTopWidth, &BorderTopStyle, &BorderTopColour)) -> Self {
        Self((props.0.0, props.1.0, props.2.0).into(), true)
    }
}
