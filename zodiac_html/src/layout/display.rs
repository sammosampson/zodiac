use serde::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub enum DisplayTypes {
    None,
    Inline,
    Block
}

impl Default for DisplayTypes {
    fn default() -> Self {
        DisplayTypes::None
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Display(DisplayTypes, bool);

impl Display {
    pub fn inline() -> Self {
        Self(DisplayTypes::Inline, true)
    }

    pub fn block() -> Self {
        Self(DisplayTypes::Block, true)
    }
}

impl From<DisplayTypes> for Display {
    fn from(types: DisplayTypes) -> Self {
        Self(types, true)
    }
}

impl Into<DisplayTypes> for &Display {
    fn into(self) -> DisplayTypes {
        self.0
    }
}

impl zodiac::PropertySetCheck for Display {
    fn is_set(&self) -> bool {
        self.1
    }
}