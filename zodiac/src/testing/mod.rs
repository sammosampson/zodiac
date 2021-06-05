pub mod rendering;

pub use rendering::*;
use zodiac_entities::*;
use zodiac_layout::*;
use zodiac_source::*;
use zodiac_source::embedding::*;
use zodiac_source::application_state::*;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct TestState {
}

impl State for TestState {
}

pub fn root() -> RootBuilder<TestState> {
    RootBuilder::<TestState>::new()
}

pub fn test_builders<TRootFunc: FnMut() -> RootNode<TestState> + Copy + Clone + 'static>(
    root_func: TRootFunc,
    dimensions: Dimensions) -> Vec::<Box::<dyn ApplicationBundleBuilder>> {
    vec!(
        Box::new(standard_source_building(TestState::default(), root_func)),
        Box::new(standard_layout()),
        Box::new(standard_test_rendering()),
        Box::new(test_renderer(dimensions)))
}