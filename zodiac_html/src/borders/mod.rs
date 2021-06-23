mod colour;
mod radius;
mod sides;
mod styles;
mod width;
pub use colour::*;
pub use radius::*;
pub use sides::*;
pub use styles::*;
pub use width::*;

use serde::*;
use zodiac::PropertySetCheck;
use crate::{Colour, Size};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct BorderValues(Size, BorderStyles, Colour);

impl Default for BorderValues {
    fn default() -> Self {
        Self(Size::default(), BorderStyles::None, Colour::default())
    }
}

impl From<(Size, BorderStyles, Colour)> for BorderValues {
    fn from(props: (Size, BorderStyles, Colour)) -> Self {
        Self(props.0, props.1, props.2)
    }
}

impl Into<(Size, BorderStyles, Colour)> for BorderValues {
    fn into(self) -> (Size, BorderStyles, Colour) {
        (self.0, self.1, self.2)
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Border(BorderValues, bool);

impl zodiac::PropertySetCheck for Border {
    fn is_set(&self) -> bool {
        self.1
    }
}

impl From<BorderValues> for Border {
    fn from(values: BorderValues) -> Self {
        Self(values, true)
    }
}

impl Into<(Size, BorderStyles, Colour)> for &Border {
    fn into(self) -> (Size, BorderStyles, Colour) {
        self.0.into()
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct FullBorder {
    top: BorderTop,
    left: BorderLeft,
    bottom: BorderBottom,
    right: BorderRight,
    radius: BorderRadius
}

impl FullBorder {
    pub fn is_visible(&self) -> bool {
        self.top.is_set()
        || self.left.is_set()
        || self.bottom.is_set()
        || self.right.is_set()
    }
}


impl zodiac::PropertySet<(&BorderTop, &BorderLeft, &BorderBottom, &BorderRight, &BorderRadius)> for FullBorder {
    fn set(&mut self, to_set: (&BorderTop, &BorderLeft, &BorderBottom, &BorderRight, &BorderRadius)) {
        self.top = *to_set.0;
        self.left = *to_set.1;
        self.bottom = *to_set.2;
        self.right = *to_set.3;
        self.radius = *to_set.4;
    }
}

impl From<(&BorderTop, &BorderLeft, &BorderBottom, &BorderRight, &BorderRadius)> for FullBorder {
    fn from(props: (&BorderTop, &BorderLeft, &BorderBottom, &BorderRight, &BorderRadius)) -> Self {
        Self {
            top: *props.0,
            left: *props.1,
            bottom: *props.2,
            right: *props.3,
            radius: *props.4,
        }
    }
}

impl Into<(BorderTop, BorderLeft, BorderBottom, BorderRight, BorderRadius)> for FullBorder {
    fn into(self) -> (BorderTop, BorderLeft, BorderBottom, BorderRight, BorderRadius) {
        (self.top, self.left, self.bottom, self.right, self.radius)
    }
}