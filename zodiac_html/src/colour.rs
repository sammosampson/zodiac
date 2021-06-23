use serde::*;

pub fn rgb(r: u8, g: u8, b: u8) -> Colour {
    Colour::from((r, g, b, 255))
}

pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Colour {
    Colour::from((r, g, b, a))
}

#[derive(Default, Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<(u8, u8, u8, u8)> for Colour {
    fn from(colour: (u8, u8, u8, u8)) -> Self {
        Self {
            r: colour.0,
            g: colour.1,
            b: colour.2,
            a: colour.3,
        }
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackgroundColour(pub Colour);

impl From<Colour> for BackgroundColour {
    fn from(colour: Colour) -> Self {
        Self(colour)
    }
}

impl Into<Colour> for BackgroundColour {
    fn into(self) -> Colour {
        self.0
    }
}