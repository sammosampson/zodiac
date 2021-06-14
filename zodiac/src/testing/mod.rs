mod rendering;
mod embedding;
mod components;
mod systems;

pub use rendering::*;
pub use embedding::*;
pub use components::*;

use crate::{ApplicationBundleBuilder, Dimensions, RootBuilder, State};

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct TestState {
}

impl State for TestState {
}

pub fn root() -> RootBuilder<TestState> {
    RootBuilder::<TestState>::new()
}

pub fn test_builders(dimensions: Dimensions) -> Vec::<Box::<dyn ApplicationBundleBuilder>> {
    vec!(Box::new(test_renderer(dimensions)))
}