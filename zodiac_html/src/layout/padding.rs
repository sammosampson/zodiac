use serde::*;
use crate::size::*;
use super::spatial::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub enum PaddingSize {
    None,
    Inherit,
    Specific(Size)
}

impl Default for PaddingSize {
    fn default() -> Self {
        Self::None
    }
}

impl Into<LayoutDistance> for PaddingSize {
    fn into(self) -> LayoutDistance {
        match self {
            PaddingSize::Specific(size) => LayoutDistance::Fixed(size.into()),
            _ => LayoutDistance::default()
        }
    }
}

impl Into<Size> for PaddingSize {
    fn into(self) -> Size {
        match self {
            PaddingSize::Specific(size) => size,
            _ => Size::default()
        }
    }
}

impl Into<PaddingSize> for Size {
    fn into(self) -> PaddingSize {
        PaddingSize::Specific(self)
    }
}

#[derive(Default, Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct PaddingSizes(PaddingSize, PaddingSize, PaddingSize, PaddingSize);

impl Into<LayoutOffsetRect> for PaddingSizes {
    fn into(self) -> LayoutOffsetRect {
        LayoutOffsetRect::from((self.0.into(), self.1.into(), self.2.into(), self.3.into()))
    }
}

impl Into<PaddingSizes> for (PaddingSize, PaddingSize, PaddingSize, PaddingSize) {
    fn into(self) -> PaddingSizes {
        PaddingSizes(self.0, self.1, self.2, self.3)
    }
}

impl Into<PaddingSizes> for (PaddingSize, PaddingSize, PaddingSize) {
    fn into(self) -> PaddingSizes {
        PaddingSizes(self.0, self.1, self.2, self.1)
    }
}

impl Into<PaddingSizes> for (PaddingSize, PaddingSize) {
    fn into(self) -> PaddingSizes {
        PaddingSizes(self.0, self.1, self.0, self.1)
    }
}

impl Into<PaddingSizes> for PaddingSize {
    fn into(self) -> PaddingSizes {
        PaddingSizes(self, self, self, self)
    }
}

impl Into<PaddingSizes> for Size {
    fn into(self) -> PaddingSizes {
        PaddingSizes(self.into(), self.into(), self.into(), self.into())
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Padding(PaddingSizes, bool);

impl zodiac::PropertySetCheck for Padding {
    fn is_set(&self) -> bool {
        self.1
    }
}

impl From<PaddingSizes> for Padding {
    fn from(values: PaddingSizes) -> Self {
        Self(values, true)
    }
}

impl Into<LayoutOffsetRect> for &Padding {
    fn into(self) -> LayoutOffsetRect {
        self.0.into()
    }
}