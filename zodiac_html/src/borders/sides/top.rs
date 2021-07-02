use serde::*;
use crate::layout::*;
use crate::borders::styles::*;
use crate::colour::*;
use crate::size::*;
use crate::BorderValues;

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

impl zodiac::PropertySet<Colour> for BorderTopColour {
    fn set(&mut self, to_set: Colour) {
        self.0 = to_set;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderTopStyle(BorderStyles);

impl Default for BorderTopStyle {
    fn default() -> Self {
        Self(BorderStyles::None)
    }
}

impl zodiac::PropertySet<BorderStyles> for BorderTopStyle {
    fn set(&mut self, to_set: BorderStyles) {
        self.0 = to_set;
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

impl zodiac::PropertySet<Size> for BorderTopWidth {
    fn set(&mut self, to_set: Size) {
        self.0 = to_set;
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

impl zodiac::PropertySet<BorderValues> for BorderTop {
    fn set(&mut self, to_set: BorderValues) {
        self.0 = to_set;
        self.1 = true;
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

impl Into<LayoutDistance> for BorderTop {
    fn into(self) -> LayoutDistance {
        self.0.into()
    }
}

impl From<(&BorderTopWidth, &BorderTopStyle, &BorderTopColour)> for BorderTop {
    fn from(props: (&BorderTopWidth, &BorderTopStyle, &BorderTopColour)) -> Self {
        Self((props.0.0, props.1.0, props.2.0).into(), true)
    }
}
