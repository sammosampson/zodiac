use std::ops::Add;
use legion::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FatalErrorReason {
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FatalError {
    pub error: FatalErrorReason
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct SourceFile {
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct SourceFileParsed {
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct SourceFileRemoval {
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Relationship {
    pub parent: Option<Entity>,
    pub next_sibling: Option<Entity>,
    pub first_child: Option<Entity>,
    pub last_child: Option<Entity>
}

impl Relationship {
    pub fn for_parent_only(parent: Entity) -> Self {
        Self {
            parent: Some(parent),
            next_sibling: None,
            first_child: None,
            last_child: None 
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Root {
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct RootWindowResized {
    pub width: u16,
    pub height: u16
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct LayoutRequest {
    pub left: u16,
    pub top: u16,
    pub width: u16,
    pub height: u16
}

impl From<&RootWindowResized> for LayoutRequest {
    fn from(window_resized: &RootWindowResized) -> Self {
        LayoutRequest {
            left: 0, 
            top: 0, 
            width: 
            window_resized.width, 
            height: window_resized.height
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Resized {
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Mapped {
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LayoutType {
    Horizontal,
    Vertical,
    Canvas,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LayoutContent {
    pub layout_type: LayoutType
}

impl LayoutContent {
    pub fn canvas() -> Self {
        Self { layout_type: LayoutType::Canvas }
    }

    pub fn horizontal() -> Self {
        Self { layout_type: LayoutType::Horizontal }
    }

    pub fn vertical() -> Self {
        Self { layout_type: LayoutType::Vertical }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RenderType {
    Circle,
    Rectangle,
    Glyph
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Renderable { 
    pub render_type: RenderType 
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Character { 
    pub character: char,
    pub position: usize
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Left {
    pub left: u16
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Top {
    pub top: u16
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct OffsetsMapped {
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct LayoutChange {
    pub left: u16,
    pub top: u16,
    pub width: u16,
    pub height: u16
}

impl Add<Left> for LayoutChange {
    type Output = Self;

    fn add(self, other: Left) -> Self {
        Self { left: self.left + other.left, top: self.top, width: self.width, height: self.height }
    }
}

impl Add<Top> for LayoutChange {
    type Output = Self;

    fn add(self, other: Top) -> Self {
        Self { left: self.left, top: self.top + other.top, width: self.width, height: self.height }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Width {
    pub width: u16
}

impl From<&Radius> for Width {
    fn from(radius: &Radius) -> Self {
        Width {
            width: radius.radius
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MinimumWidth {
    pub width: u16
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Height {
    pub height: u16
}

impl From<&Radius> for Height {
    fn from(radius: &Radius) -> Self {
        Height {
            height: radius.radius
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MinimumHeight {
    pub height: u16
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
    pub left_top: u16,
    pub right_top: u16,
    pub right_bottom: u16,
    pub left_bottom: u16,
}