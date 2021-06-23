use serde::*;
use zodiac::Dimensions;

use crate::{Size, layout::Margin};

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct MinimumDimensionsWithoutChildren(Dimensions);

impl From<&Margin> for MinimumDimensionsWithoutChildren {
    fn from(margin: &Margin) -> Self {
        Self(margin.into())
    }
}

pub struct WrappedDimensions(Dimensions);

impl Into<Dimensions> for WrappedDimensions {
    fn into(self) -> Dimensions {
        self.0
    }
}

impl From<(Size, Size)> for WrappedDimensions {
    fn from(sizes: (Size, Size)) -> Self {
        let width: u16 = sizes.0.into();
        let height: u16 =  sizes.1.into();
        Self(Dimensions::from((width, height)))
    }
}

impl From<(Size, Size, Size, Size)> for WrappedDimensions {
    fn from(sizes: (Size, Size, Size, Size)) -> Self {
        let height = sizes.0 + sizes.2;
        let width = sizes.1 + sizes.3;
        Self::from((width, height))
    }
}
