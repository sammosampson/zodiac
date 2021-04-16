use std::{fmt, ops::*};
use std::error::*;
use legion::*;
use serde::*;

#[derive(Debug)]
pub enum ZodiacError {
    FailedToRender(RendererError),
    FailedToFileMonitorFiles(FileMonitorError)
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

impl From<FileMonitorError> for ZodiacError {
    fn from(error: FileMonitorError) -> Self {
        ZodiacError::FailedToFileMonitorFiles(error)
    }
}

#[derive(Debug)]
pub enum FileMonitorError {
    WatchError,
    FilePathError(FilePathError)
}

#[derive(Debug)]
pub enum FileMonitorWatchError {
    NoLongerMonitoring,
    NoFileChanges
}

#[derive(Debug)]
pub enum FilePathError {
    ManifestDirectoryEnvironmentVariableNotSet
}

impl From<FilePathError> for FileMonitorError {
    fn from(error: FilePathError) -> FileMonitorError {
        FileMonitorError::FilePathError(error)
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum SourceTokenError {
    CouldNotFindStartTag(usize),
    CouldNotParseNumberValue(usize),
    CouldNotFindControlName(usize),
    CouldNotFindPropertyStartSymbol(usize),
    CouldNotFindControlToClose(usize),
    CouldNotFindControlCloseSymbol(usize),
    ClosingWrongTag(usize)
}

impl Error for SourceTokenError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            _ => None
        }
    }
}

impl fmt::Display for SourceTokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SourceTokenError::CouldNotFindStartTag(position) => write!(f, "could not find start tag ({:?})", position),
            SourceTokenError::CouldNotParseNumberValue(position) => write!(f, "could not parse number value ({:?})", position),
            SourceTokenError::CouldNotFindControlName(position) => write!(f, "could not find control name ({:?})", position),
            SourceTokenError::CouldNotFindPropertyStartSymbol(position) => write!(f, "could not find property start symbol ({:?})", position),
            SourceTokenError::CouldNotFindControlToClose(position) => write!(f, "could not find control to close ({:?})", position),
            SourceTokenError::CouldNotFindControlCloseSymbol(position) => write!(f, "could not find control close symbol ({:?})", position),
            SourceTokenError::ClosingWrongTag(position) => write!(f, "closing wrong tag ({:?})", position),
        }
    }    
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum AbstractSyntaxTokenError {
    SourceTokenError(SourceTokenError),
    UnusedPropertyType,
    UnknownProperty,
    BadColourValue,
    BadStrokeColourValue,
    BadCornerRadiiValue
}

impl Error for AbstractSyntaxTokenError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AbstractSyntaxTokenError::SourceTokenError(source) => Some(source),
            AbstractSyntaxTokenError::UnusedPropertyType => None,
            AbstractSyntaxTokenError::UnknownProperty => None,
            AbstractSyntaxTokenError::BadColourValue => None,
            AbstractSyntaxTokenError::BadStrokeColourValue => None,
            AbstractSyntaxTokenError::BadCornerRadiiValue => None,
        }
    }
}

impl fmt::Display for AbstractSyntaxTokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AbstractSyntaxTokenError::SourceTokenError(source) => write!(f, "Error in source ({:?})", source),
            AbstractSyntaxTokenError::UnusedPropertyType => write!(f, "Unused property type"),
            AbstractSyntaxTokenError::UnknownProperty => write!(f, "Unknown property"),
            AbstractSyntaxTokenError::BadColourValue => write!(f, "Bad colour value"),
            AbstractSyntaxTokenError::BadStrokeColourValue => write!(f, "Bad stroke colour value"),
            AbstractSyntaxTokenError::BadCornerRadiiValue => write!(f, "Bad corner radii value"),
        }
    }    
}

impl<'a> From<SourceTokenError> for AbstractSyntaxTokenError {
    fn from(error: SourceTokenError) -> Self {
        AbstractSyntaxTokenError::SourceTokenError(error)
    }
}

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
pub enum BuildError {
    TokenError(AbstractSyntaxTokenError),
    UnexpectedToken(AbstractSyntaxNodeType),
    MissingRequiredTokens(Vec<AbstractSyntaxNodeType>),
    SourceLocationDoesNotExist(String),
    ControlDoesNotExist(String),
    ControlSourceDoesNotExist(String),
    ControlSourceFileDoesNotExist
}

impl Error for BuildError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            BuildError::TokenError(source) => Some(source),
            BuildError::UnexpectedToken(_) => None,
            BuildError::MissingRequiredTokens(_) => None,
            BuildError::SourceLocationDoesNotExist(_) => None,
            BuildError::ControlDoesNotExist(_) => None,
            BuildError::ControlSourceDoesNotExist(_) => None,
            BuildError::ControlSourceFileDoesNotExist => None
        }
    }
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildError::TokenError(source) => source.fmt(f),
            BuildError::UnexpectedToken(token) => write!(f, "Unexpected token {:?}", token),
            BuildError::MissingRequiredTokens(tokens) => write!(f, "Missing required tokens ({:?})", tokens),
            BuildError::SourceLocationDoesNotExist(location) => write!(f, "Source does not exist for {:?}", location),
            BuildError::ControlDoesNotExist(control) => write!(f, "Control does not exist {:?}", control),
            BuildError::ControlSourceDoesNotExist(control) => write!(f, "Control source does not exist {:?}", control),
            BuildError::ControlSourceFileDoesNotExist => write!(f, "Control source file does not exist"),
        }
    }    
}

impl<'a> From<AbstractSyntaxTokenError> for BuildError {
    fn from(error: AbstractSyntaxTokenError) -> Self {
        BuildError::TokenError(error)
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

impl Colour {
    pub fn red() -> Self {
        Self {
            r: 1.0, 
            g: 0.0, 
            b: 0.0, 
            a: 1.0
        }
    }
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

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Default)]
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