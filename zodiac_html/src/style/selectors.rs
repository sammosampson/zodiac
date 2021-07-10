use serde::*;
use crate::layout::*;

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElementSelector(ElementType);

impl From<ElementType> for ElementSelector {
    fn from(element_type: ElementType) -> Self {
        Self(element_type)
    }
}

impl Into<ElementType> for ElementSelector {
    fn into(self) -> ElementType {
        self.0
    }
}

impl Into<ElementType> for &ElementSelector {
    fn into(self) -> ElementType {
        self.0
    }
}