use serde::*;
use crate::Size;

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderRadius {
    pub top_left: Size,
    pub top_right: Size,
    pub bottom_left: Size,
    pub bottom_right: Size,
    is_set: bool
}

impl From<Size> for BorderRadius {
    fn from(size: Size) -> Self {
        Self {
            top_left: size,
            top_right: size,
            bottom_left: size,
            bottom_right: size,
            is_set: true
        }
    }
}

impl zodiac::PropertySetCheck for BorderRadius {
    fn is_set(&self) -> bool {
        self.is_set
    }
}
