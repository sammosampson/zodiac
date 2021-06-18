use serde::*;

pub fn px(points: u8) -> Size {
    Size::points(points)
}

#[derive(Default, Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct Size(u8);

impl Size {
    fn points(points:u8) -> Self {
        Self(points)
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