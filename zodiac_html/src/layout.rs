use serde::*;
use zodiac::Dimensions;
use crate::{dimensions::WrappedDimensions, size::*};

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Span {
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Div {
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub enum DisplayTypes {
    None,
    Inline,
    Block
}

impl Default for DisplayTypes {
    fn default() -> Self {
        DisplayTypes::None
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Display(DisplayTypes);

impl Display {
    pub fn inline() -> Self {
        Self(DisplayTypes::Inline)
    }

    pub fn block() -> Self {
        Self(DisplayTypes::Block)
    }
}

impl From<DisplayTypes> for Display {
    fn from(types: DisplayTypes) -> Self {
        Self(types)
    }
}

impl Into<DisplayTypes> for &Display {
    fn into(self) -> DisplayTypes {
        self.0
    }
}

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

impl Into<PaddingSize> for Size {
    fn into(self) -> PaddingSize {
        PaddingSize::Specific(self)
    }
}

#[derive(Default, Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct PaddingSizes(PaddingSize, PaddingSize, PaddingSize, PaddingSize);

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