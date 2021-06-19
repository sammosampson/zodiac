use std::{fmt::Debug, time::Instant};
use zodiac::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TestState {
    pub time: Instant,
    pub border_size: u64
}

impl Default for TestState {
    fn default() -> Self {
        Self {
            time: Instant::now(),
            border_size: 0
        }
    }
}

impl State for TestState {
}

pub fn root() -> RootBuilder<TestState> {
    RootBuilder::<TestState>::new()
}