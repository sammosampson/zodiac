use std::fmt::Debug;
use zodiac::*;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct TestState {
    pub test_control: i32,
    pub control_count: usize
}

impl State for TestState {
}

pub fn root() -> RootBuilder<TestState> {
    RootBuilder::<TestState>::new()
}