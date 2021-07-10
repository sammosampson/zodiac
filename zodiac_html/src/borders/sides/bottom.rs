use serde::*;
use crate::BorderValues;
use crate::borders::styles::*;
use crate::colour::*;
use crate::size::*;
use crate::layout::*;

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderBottomColour(Colour, bool);

impl From<Colour> for BorderBottomColour {
    fn from(colour: Colour) -> Self {
        Self(colour, true)
    }
}

impl Into<Colour> for &BorderBottomColour {
    fn into(self) -> Colour {
        self.0
    }
}

impl zodiac::PropertySet<Colour> for BorderBottomColour {
    fn set(&mut self, to_set: Colour) {
        self.0 = to_set;
        self.1 = true;
    }
}

impl zodiac::PropertySetCheck for BorderBottomColour {
    fn is_set(&self) -> bool {
        self.1
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderBottomStyle(BorderStyles, bool);

impl zodiac::PropertySet<BorderStyles> for BorderBottomStyle {
    fn set(&mut self, to_set: BorderStyles) {
        self.0 = to_set;
        self.1 = true;
    }
}

impl zodiac::PropertySetCheck for BorderBottomStyle {
    fn is_set(&self) -> bool {
        self.1
    }
}

impl From<BorderStyles> for BorderBottomStyle {
    fn from(styles: BorderStyles) -> Self {
        Self(styles, true)
    }
}

impl Into<BorderStyles> for &BorderBottomStyle {
    fn into(self) -> BorderStyles {
        self.0
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderBottomWidth(Size, bool);

impl From<Size> for BorderBottomWidth {
    fn from(size: Size) -> Self {
        Self(size, true)
    }
}

impl zodiac::PropertySet<Size> for BorderBottomWidth {
    fn set(&mut self, to_set: Size) {
        self.0 = to_set;
        self.1 = true;
    }
}

impl zodiac::PropertySetCheck for BorderBottomWidth {
    fn is_set(&self) -> bool {
        self.1
    }
}

impl Into<Size> for &BorderBottomWidth {
    fn into(self) -> Size {
        self.0
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderBottom(BorderValues, bool);

impl zodiac::PropertySetCheck for BorderBottom {
    fn is_set(&self) -> bool {
        self.1
    }
}

impl zodiac::PropertySet<BorderValues> for BorderBottom {
    fn set(&mut self, to_set: BorderValues) {
        self.0 = to_set;
        self.1 = true;
    }
}

impl From<BorderValues> for BorderBottom {
    fn from(side: BorderValues) -> Self {
        Self(side, true)
    }
}

impl Into<(Size, BorderStyles, Colour)> for BorderBottom {
    fn into(self) -> (Size, BorderStyles, Colour) {
        self.0.into()
    }
}

impl Into<LayoutDistance> for BorderBottom {
    fn into(self) -> LayoutDistance {
        self.0.into()
    }
}

impl From<(&BorderBottomWidth, &BorderBottomStyle, &BorderBottomColour)> for BorderBottom {
    fn from(props: (&BorderBottomWidth, &BorderBottomStyle, &BorderBottomColour)) -> Self {
        Self((props.0.0, props.1.0, props.2.0).into(), true)
    }
}
