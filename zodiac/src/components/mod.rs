mod relationships;
mod errors;
mod resizing;
mod layout;

pub use relationships::*;
pub use errors::*;
pub use resizing::*;
pub use layout::*;

use serde::*;


#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Root {
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Rebuild {
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Mapped {
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Removed {
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Renderable {
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OffsetsMapped {
}