mod rendering;
mod systems;
mod logging;

pub use rendering::*;
pub use logging::*;

use legion::*;

use zodiac::{
    Application, ApplicationBundleBuilder, ApplicationRunner, Dimensions, RootBuilder, RootNode, State, world_logging
};

use crate::html_builder;

pub fn test_app<TRootFunc: FnMut() -> RootNode<TestState> + Copy + 'static>(root_func: TRootFunc) -> TestApplication {
    TestApplication::new(root_func)
}

pub struct TestApplication(Application<TestState>, Dimensions);

impl TestApplication {
    fn new<TRootFunc: FnMut() -> RootNode<TestState> + Copy + 'static>(root_func: TRootFunc) -> Self {
        Self(
            Application::new(TestState::default(), root_func),
            Dimensions::new(1024, 768)
        )
    }
    
    pub fn with_screen_dimensions(self, screen_width: u16, screen_height: u16) -> Self {
        Self(self.0, Dimensions::new(screen_width, screen_height))
    }

    pub fn build(self) -> TestApplicationRunner {
        let runner = self.0
            .with_builders(&mut test_builders(self.1))
            .with_builder(world_logging())
            .build()
            .unwrap();
        
        TestApplicationRunner::new(runner)
    }
}

pub struct TestApplicationRunner(ApplicationRunner<TestState>);

impl TestApplicationRunner {
    fn new(runner: ApplicationRunner<TestState>) -> Self {
        Self(runner)
    }

    pub fn run_once(mut self) -> Self{
        self.0.run_once();
        self
    }

    pub fn get_changes(&mut self) -> Vec::<RenderPrimitive> {
        <&RenderPrimitive>::query()
            .iter(self.0.world_mut())
            .map(|change| change.clone())
            .collect()
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct TestState {
}

impl State for TestState {
}

pub fn root() -> RootBuilder<TestState> {
    RootBuilder::<TestState>::new()
}

pub fn test_builders(dimensions: Dimensions) -> Vec::<Box::<dyn ApplicationBundleBuilder>> {
    vec!(Box::new(test_renderer(dimensions)), Box::new(html_builder()))
}
