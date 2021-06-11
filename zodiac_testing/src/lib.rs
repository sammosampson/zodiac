mod rendering;
mod embedding;
mod components;
mod systems;

pub use rendering::*;
pub use embedding::*;
pub use components::*;
use zodiac::*;
use zodiac_layout::*;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct TestState {
}

impl State for TestState {
}

pub fn root() -> RootBuilder<TestState> {
    RootBuilder::<TestState>::new()
}

pub fn test_builders(dimensions: Dimensions) -> Vec::<Box::<dyn ApplicationBundleBuilder>> {
    vec!(
        Box::new(standard_layout()),
        Box::new(test_renderer(dimensions)))
}