mod tree;
mod selectors;
mod embedding;

pub use tree::*;
pub use selectors::*;
pub use embedding::*;

use serde::*;

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Style {
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefaultStyle {
}
