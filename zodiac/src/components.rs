#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Circle {
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Text {
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dimensions {
    pub x: u16,
    pub y: u16,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Radius {
    pub radius: u16
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlyphIndex {
    pub index: u16
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StrokeWidth {
    pub width: u16
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StrokeColour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CornerRadii {
    pub left_top: f32,
    pub right_top: f32,
    pub right_bottom: f32,
    pub left_bottom: f32,
}