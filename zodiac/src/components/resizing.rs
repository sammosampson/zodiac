use serde::*;

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RootWindowResized {
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Resized {
}
