use serde::*;

use crate::Colour;
use crate::FontSize;

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Style {
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Span {
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextSize {
    size: FontSize
}

impl From<FontSize> for TextSize {
    fn from(size: FontSize) -> Self {
        Self {
            size
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextColour {
    colour: Colour
}

impl From<Colour> for TextColour {
    fn from(colour: Colour) -> Self {
        Self {
            colour
        }
    }
}