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
pub struct Display(DisplayTypes);

impl Display {
    pub fn inline() -> Self {
        Self(DisplayTypes::Inline)
    }

    pub fn block() -> Self {
        Self(DisplayTypes::Block)
    }
}

impl From<DisplayTypes> for Display {
    fn from(types: DisplayTypes) -> Self {
        Self(types)
    }
}

impl Into<DisplayTypes> for &Display {
    fn into(self) -> DisplayTypes {
        self.0
    }
}