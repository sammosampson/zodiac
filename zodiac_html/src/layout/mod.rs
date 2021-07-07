mod boxing;
mod padding;
mod margin;
mod display;
mod tree;
mod direction;
mod spatial;
pub use boxing::*;
pub use padding::*;
pub use margin::*;
pub use display::*;
pub use tree::*;
pub use direction::*;
pub use spatial::*;

use serde::*;



#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub enum ElementType {
    None,
    Div,
    Span
}

impl Default for ElementType {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Element(ElementType);

impl Element {
    pub fn matches_selector(&self, selector: Option<ElementType>) -> bool {
        if let Some(selector) = selector {
            return selector == self.0;
        }
        selector == None
    }
}

impl From<ElementType> for Element {
    fn from(element_type: ElementType) -> Self {
        Self(element_type)
    }
}

impl Into<ElementType> for Element {
    fn into(self) -> ElementType {
        self.0
    }
}

impl Into<ElementType> for &Element {
    fn into(self) -> ElementType {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LayoutRequest {
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LayoutChange {
}
