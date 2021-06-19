use serde::*;
use crate::size::*;

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderWidth(Size, bool);

impl BorderWidth {
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl From<Size> for BorderWidth {
    fn from(size: Size) -> Self {
        Self(size, true)
    }
}

impl zodiac::PropertySetCheck for &BorderWidth {
    fn is_set(&self) -> bool {
        self.1
    }
}

impl Into<f32> for BorderWidth {
    fn into(self) -> f32 {
        self.0.into()
    }
}

impl Into<Size> for &BorderWidth {
    fn into(self) -> Size {
        self.0
    }
}