use std::ops::*;
use legion::*;
use serde::*;

#[derive(Debug)]
pub enum ZodiacError {
    FailedToRender(RendererError)
}

impl From<RendererError> for ZodiacError {
    fn from(error: RendererError) -> Self {
        ZodiacError::FailedToRender(error)
    }
}

#[derive(Debug)]
pub enum RendererError {
    FailedToDisplayWindow,
    FailedToCreateShaders(String),
    FailedToLoadFont,
    BufferSwapError,
    BufferCreationError,
    DrawError
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RootWindowResized {
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Removed {
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Relationship {
    pub parent: Option<Entity>,
    pub next_sibling: Option<Entity>,
    pub previous_sibling: Option<Entity>,
    pub first_child: Option<Entity>,
    pub last_child: Option<Entity>
}

impl Relationship {
    pub fn for_parent_only(parent: Entity) -> Self {
        Self {
            parent: Some(parent),
            next_sibling: None,
            previous_sibling: None,
            first_child: None,
            last_child: None 
        }
    }
    
    pub fn without_children(&self) -> Self {
        Self {
            parent: self.parent,
            next_sibling: self.next_sibling,
            previous_sibling: self.previous_sibling,
            first_child: None,
            last_child: None 
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Root {
}


#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Control {
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Rebuild {
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Dimensions {
    pub width: u16,
    pub height: u16
}

impl Dimensions {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
        }
    }
}

impl From<(u16, u16)> for Dimensions {
    fn from(dimensions: (u16, u16)) -> Self {
        Self::new(dimensions.0, dimensions.1)
    }
}

impl From<(u32, u32)> for Dimensions {
    fn from(dimensions: (u32, u32)) -> Self {
        Self::new(dimensions.0 as u16, dimensions.1 as u16)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurrentLayoutConstraints {
    pub left: u16,
    pub top: u16,
    pub width: u16,
    pub height: u16
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LayoutRequest {
    pub left: u16,
    pub top: u16,
    pub width: u16,
    pub height: u16
}

impl From<&Dimensions> for LayoutRequest {
    fn from(dimensions: &Dimensions) -> Self {
        LayoutRequest {
            left: 0, 
            top: 0, 
            width: 
            dimensions.width, 
            height: dimensions.height
        }
    }
}

impl From<&CurrentLayoutConstraints> for LayoutRequest {
    fn from(current_layout_constraints: &CurrentLayoutConstraints) -> Self {
        LayoutRequest {
            left: current_layout_constraints.left, 
            top: current_layout_constraints.top, 
            width: current_layout_constraints.width,
            height: current_layout_constraints.height,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Resized {
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Mapped {
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum LayoutType {
    Horizontal,
    Vertical,
    Canvas,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Renderable {
}



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
pub struct FontSize { 
    pub size: u8
}

impl From<u8> for FontSize {
    fn from(size: u8) -> Self {
        Self {
            size
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Left {
    pub left: u16
}

impl From<u16> for Left {
    fn from(left: u16) -> Self {
        Self {
            left
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Top {
    pub top: u16
}

impl From<u16> for Top {
    fn from(top: u16) -> Self {
        Self {
            top
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OffsetsMapped {
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
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

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Width {
    pub width: u16
}

impl From<u16> for Width {
    fn from(width: u16) -> Self {
        Self {
            width
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

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct MinimumWidth {
    pub width: u16
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Height {
    pub height: u16
}


impl From<u16> for Height {
    fn from(height: u16) -> Self {
        Self {
            height
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

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct MinimumHeight {
    pub height: u16
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

impl Into<[f32;4]> for Colour {
    fn into(self) -> ([f32;4]) {
        [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0
        ]
    }
}

impl Into<[u8;4]> for Colour {
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

impl Into<[f32;4]> for StrokeColour {
    fn into(self) -> ([f32;4]) {
        [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0
        ]
    }
}

impl Into<[u8;4]> for StrokeColour {
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