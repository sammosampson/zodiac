use std::ops::Add;
use legion::*;
use serde::*;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum AbstractSyntaxNodeType {
    Root,
    Import,
    Circle,
    Rectangle,
    Text,
    CanvasLayoutContent,
    HorizontalLayoutContent,
    VerticalLayoutContent,
    Left,
    Top,
    Width,
    Height,
    Radius,
    StrokeWidth,
    Content,
    Path,
    Name,
    Colour,
    StrokeColour,
    CornerRadii,
    Unknown
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum BuildErrorReason {
    UnexpectedToken(AbstractSyntaxNodeType),
    MissingRequiredTokens(Vec<AbstractSyntaxNodeType>),
    SourceLocationDoesNotExist(String),
    ControlDoesNotExist(String),
    ControlSourceDoesNotExist(String)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BuildError {
    pub entity: Entity,
    reason: BuildErrorReason
}

impl BuildError {
    pub fn new(entity: Entity, reason: BuildErrorReason) -> Self {
        Self {
            entity,
            reason
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BuildErrorOccurrence {
    pub error: BuildError
}

impl From<BuildError> for BuildErrorOccurrence {
    fn from(error: BuildError) -> Self {
        Self {
            error
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SourceFile {
}


#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SourceFileRoot {
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceImplementation {
    pub source_file_entity: Entity
}

impl SourceImplementation {
    pub fn from_source_entity(source_file_entity: Entity) -> Self {
        Self {
            source_file_entity
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SourceFileParsed {
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SourceFileChange {
}
#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SourceFileCreation {
}


#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SourceFileInitialRead {
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SourceFileRemoval {
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Removed {
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
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
    
    pub fn without_children(&self) -> Self {
        Self {
            parent: self.parent,
            next_sibling: self.next_sibling,
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
pub struct RootWindowResized {
    pub width: u16,
    pub height: u16
}

impl From<(u16, u16)> for RootWindowResized {
    fn from(dimensions: (u16, u16)) -> Self {
        Self {
            width: dimensions.0,
            height: dimensions.1,
        }
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

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Import {
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

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum RenderType {
    Circle,
    Rectangle,
    Glyph
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Renderable { 
    pub render_type: RenderType 
}

impl Renderable {
    pub fn circle() -> Self {
        Self { render_type: RenderType::Circle }
    }

    pub fn rectangle() -> Self {
        Self { render_type: RenderType::Rectangle }
    }

    pub fn glyph() -> Self {
        Self { render_type: RenderType::Glyph }
    }
}


#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Name { 
    pub name: String
}

impl From<&str> for Name {
    fn from(name: &str) -> Self {
        Self {
            name: name.to_string()
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Path { 
    pub path: String
}

impl From<&str> for Path {
    fn from(path: &str) -> Self {
        Self {
            path: path.to_string()
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Character { 
    pub character: char,
    pub position: usize
}

impl Character {
    pub fn new(character: char, position: usize) -> Self {
        Self {
            character,
            position
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

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct GlyphIndex {
    pub index: u16
}

impl From<u16> for GlyphIndex {
    fn from(index: u16) -> Self {
        Self {
            index
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl From<(f32, f32, f32, f32)> for Colour {
    fn from(colour: (f32, f32, f32, f32)) -> Self {
        Self {
            r: colour.0,
            g: colour.1,
            b: colour.2,
            a: colour.3,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct StrokeColour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl From<(f32, f32, f32, f32)> for StrokeColour {
    fn from(colour: (f32, f32, f32, f32)) -> Self {
        Self {
            r: colour.0,
            g: colour.1,
            b: colour.2,
            a: colour.3,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
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