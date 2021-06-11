use mox::mox;
use zodiac::*;
use zodiac_layout::*;
use zodiac_rendering_pathfinder::*;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct WorldViewerState {
}

impl State for WorldViewerState {
}

pub fn root() -> RootBuilder<WorldViewerState> {
    RootBuilder::<WorldViewerState>::new()
}

#[topo::nested]
fn world_viewer() -> Node {
    mox!(
        <text
            content="Hello World!".to_string()
            font_size=19
            colour=(255, 255, 255, 255)
            left=50
            top=50
        />
    )
}

#[topo::nested]
fn app_root() -> RootNode<WorldViewerState> {
    mox!(
        <root>
            <world_viewer />
        </root>
    )
}

fn main() {
    std::env::set_var("RUST_LOG", "info");
    Application::new(WorldViewerState::default(), app_root)
        .use_logging()
        .with_builder(standard_layout())        
        .with_builder(standard_pathfinder_rendering())
        .with_builder(pathfinder_renderer())
        //.with_builder(world_logging())
        .build()
        .unwrap()
        .run_until_closed();
}