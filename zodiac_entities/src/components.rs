use std::ops::Add;
use legion::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Relationship {
    pub parent: Option<Entity>,
    pub next_sibling: Option<Entity>,
    pub first_child: Option<Entity>,
    pub last_child: Option<Entity>
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResizeRequest {
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RelationshipMapped {
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LeftOffsetMapped {
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TopOffsetMapped {
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WidthMapped {
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeightMapped {
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HorizontalLayoutContent {
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CanvasLayoutContent {
}

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
pub struct Left {
    pub left: u16
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Top {
    pub top: u16
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OffsetsMapped {
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Default for Position {
    fn default() -> Self { 
        Self { x: 0, y: 0 }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Width {
    pub width: u16
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Height {
    pub height: u16
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Radius {
    pub radius: u16
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dimensions {
    pub x: u16,
    pub y: u16,
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rendered {
}