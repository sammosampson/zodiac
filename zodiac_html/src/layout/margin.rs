use serde::*;
use zodiac::*;
use crate::dimensions::*;
use crate::size::*;
use super::spatial::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub enum MarginSize {
    None,
    Inherit,
    Auto,
    Specific(Size)
}

impl Default for MarginSize {
    fn default() -> Self {
        Self::None
    }
}

impl Into<Size> for MarginSize {
    fn into(self) -> Size {
        match self {
            MarginSize::Specific(size) => size,
            _ => Size::default()
        }
    }
}


impl Into<LayoutDistance> for MarginSize {
    fn into(self) -> LayoutDistance {
        match self {
            MarginSize::Specific(size) => LayoutDistance::Fixed(size.into()),
            _ => LayoutDistance::default()
        }
    }
}

impl Into<MarginSize> for Size {
    fn into(self) -> MarginSize {
        MarginSize::Specific(self)
    }
}

#[derive(Default, Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct MarginSizes(MarginSize, MarginSize, MarginSize, MarginSize);

impl Into<Dimensions> for MarginSizes {
    fn into(self) -> Dimensions {
        WrappedDimensions::from((self.0.into(), self.1.into(), self.2.into(), self.3.into())).into()
    }
}

impl Into<LayoutOffsetRect> for MarginSizes {
    fn into(self) -> LayoutOffsetRect {
        LayoutOffsetRect::from((self.0.into(), self.1.into(), self.2.into(), self.3.into()))
    }
}

impl Into<MarginSizes> for (MarginSize, MarginSize, MarginSize, MarginSize) {
    fn into(self) -> MarginSizes {
        MarginSizes(self.0, self.1, self.2, self.3)
    }
}

impl Into<MarginSizes> for (MarginSize, MarginSize, MarginSize) {
    fn into(self) -> MarginSizes {
        MarginSizes(self.0, self.1, self.2, self.1)
    }
}

impl Into<MarginSizes> for (MarginSize, MarginSize) {
    fn into(self) -> MarginSizes {
        MarginSizes(self.0, self.1, self.0, self.1)
    }
}

impl Into<MarginSizes> for MarginSize {
    fn into(self) -> MarginSizes {
        MarginSizes(self, self, self, self)
    }
}

impl Into<MarginSizes> for Size {
    fn into(self) -> MarginSizes {
        MarginSizes(self.into(), self.into(), self.into(), self.into())
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Margin(MarginSizes, bool);

impl zodiac::PropertySetCheck for Margin {
    fn is_set(&self) -> bool {
        self.1
    }
}

impl From<MarginSizes> for Margin {
    fn from(values: MarginSizes) -> Self {
        Self(values, true)
    }
}

impl Into<Dimensions> for &Margin {
    fn into(self) -> Dimensions {
        self.0.into()
    }
}

impl Into<LayoutOffsetRect> for &Margin {
    fn into(self) -> LayoutOffsetRect {
        self.0.into()
    }
}