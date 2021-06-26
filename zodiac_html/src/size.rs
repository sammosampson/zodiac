use std::ops::Add;

use serde::*;

pub fn px(pixels: u16) -> Size {
    Size::pixels(pixels)
}

#[derive(Default, Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct Size(u16);

impl Size {
    fn pixels(pixels:u16) -> Self {
        Self(pixels)
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl Into<f32> for Size {
    fn into(self) -> f32 {
        self.0.into()
    }
}

impl Into<u16> for Size {
    fn into(self) -> u16 {
        self.0
    }
}

impl Add<Size> for Size {
    type Output = Self;

    fn add(self, rhs: Size) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

