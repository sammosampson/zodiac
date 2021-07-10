mod tree;
mod selectors;

pub use tree::*;
pub use selectors::*;

use serde::*;

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Style {
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefaultStyle {
}
