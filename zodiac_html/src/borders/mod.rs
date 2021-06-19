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
use crate::size::*;
use crate::colour::*;


#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Border {
    colour: Colour,
    width: Size,
    style: BorderStyles,
    is_set: bool
}

impl Default for Border {
    fn default() -> Self {
        Self {
            colour: Colour::default(),
            width: Size::default(),
            style: BorderStyles::None,
            is_set: false
        }
    }
}

impl zodiac::PropertySetCheck for Border {
    fn is_set(&self) -> bool {
        self.is_set
    }
}

impl From<(Colour, Size, BorderStyles)> for Border {
    fn from(props: (Colour, Size, BorderStyles)) -> Self {
        Self {
            colour: props.0,
            width: props.1,
            style: props.2,
            is_set: true
        }
    }
}

impl Into<(Colour, Size, BorderStyles)> for &Border {
    fn into(self) -> (Colour, Size, BorderStyles) {
        (self.colour, self.width, self.style)
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
