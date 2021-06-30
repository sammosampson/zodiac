use serde::*;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum LayoutDirection {
    None,
    Horizontal,
    Vertical
}

impl Default for LayoutDirection {
    fn default() -> Self {
        Self::None
    }
}