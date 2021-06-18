use serde::*;
use zodiac::*;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Content { 
    pub text: String
}

impl From<&str> for Content {
    fn from(text: &str) -> Self {
        Self {
            text: text.to_string()
        }
    }
}

impl From<String> for Content {
    fn from(text: String) -> Self {
        Self {
            text
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Size { 
    pub size: u8
}

impl From<u8> for Size {
    fn from(size: u8) -> Self {
        Self {
            size
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Radius {
    pub radius: u16
}

impl From<u16> for Radius {
    fn from(radius: u16) -> Self {
        Self {
            radius
        }
    }
}

impl From<&Radius> for Height {
    fn from(radius: &Radius) -> Self {
        Height {
            height: radius.radius
        }
    }
}

impl From<&Radius> for Width {
    fn from(radius: &Radius) -> Self {
        Width {
            width: radius.radius
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Colour {
    pub fn red() -> Self {
        Self {
            r: 255, 
            g: 0, 
            b: 0, 
            a: 255
        }
    }
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

impl Into<[f32;4]> for &Colour {
    fn into(self) -> ([f32;4]) {
        [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0
        ]
    }
}

impl Into<[u8;4]> for &Colour {
    fn into(self) -> ([u8;4]) {
        [
            self.r,
            self.g,
            self.b,
            self.a
        ]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StrokeWidth {
    pub width: u16
}

impl From<u16> for StrokeWidth {
    fn from(width: u16) -> Self {
        Self {
            width
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Default)]
pub struct StrokeColour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<(u8, u8, u8, u8)> for StrokeColour {
    fn from(colour: (u8, u8, u8, u8)) -> Self {
        Self {
            r: colour.0,
            g: colour.1,
            b: colour.2,
            a: colour.3,
        }
    }
}

impl Into<[f32;4]> for &StrokeColour {
    fn into(self) -> ([f32;4]) {
        [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0
        ]
    }
}

impl Into<[u8;4]> for &StrokeColour {
    fn into(self) -> ([u8;4]) {
        [
            self.r,
            self.g,
            self.b,
            self.a
        ]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CornerRadii {
    pub left_top: u16,
    pub right_top: u16,
    pub right_bottom: u16,
    pub left_bottom: u16,
}


impl From<(u16, u16, u16, u16)> for CornerRadii {
    fn from(radii: (u16, u16, u16, u16)) -> Self {
        Self {
            left_top: radii.0,
            right_top: radii.1,
            right_bottom: radii.2,
            left_bottom: radii.3,
        }
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Circle {
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rectangle {
}