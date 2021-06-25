use serde::*;

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Dimensions {
    pub width: u16,
    pub height: u16
}

impl Dimensions {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
        }
    }
}

impl From<(u16, u16)> for Dimensions {
    fn from(dimensions: (u16, u16)) -> Self {
        Self::new(dimensions.0, dimensions.1)
    }
}

impl From<(u32, u32)> for Dimensions {
    fn from(dimensions: (u32, u32)) -> Self {
        Self::new(dimensions.0 as u16, dimensions.1 as u16)
    }
}

