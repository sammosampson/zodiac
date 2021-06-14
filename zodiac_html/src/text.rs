use serde::*;

pub fn px(points: u8) -> FontSize {
    FontSize::points(points)
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct FontSize {
    pub points: u8
}

impl FontSize {
    fn points(points:u8) -> Self {
        Self {
            points
        }
    }
}