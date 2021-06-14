use serde::*;

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Style {
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Span {
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Size(u8);

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextSize {
    size: Size
}