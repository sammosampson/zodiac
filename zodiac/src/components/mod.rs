mod relationships;
mod errors;
mod resizing;
mod layout;
mod style;

pub use relationships::*;
pub use errors::*;
pub use resizing::*;
pub use layout::*;
pub use style::*;

use serde::*;


#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Root {
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Rebuild {
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ComponentId(u64);

impl From<u64> for ComponentId {
    fn from(id: u64) -> Self {
        Self(id)
    }
}

impl Into<u64> for &ComponentId {
    fn into(self) -> u64 { 
        self.0
     }
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