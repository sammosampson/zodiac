use mox::mox;
use zodiac::initialisation::*;
use zodiac_source::*;
use zodiac_entities::*;
use zodiac_rendering_pathfinder::*;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct FirstState {
}

impl State for FirstState {
}

pub fn root() -> RootBuilder<FirstState> {
    RootBuilder::<FirstState>::new()
}

#[topo::nested]
fn small_control() -> Node {
    mox!(
        <circle
            colour=(0, 0, 255, 255)
            stroke_colour=(255, 255, 255, 255)
            stroke_width=20
        />
    )
}

#[topo::nested]
fn big_control() -> Node {
    mox!(
        <rect
            colour=(255, 255, 255, 255)
            stroke_colour=(0, 200, 255, 255)
            stroke_width=5
        />
    )
}

#[topo::nested]
fn other_control() -> Node {
    mox!(
        <horizontal_stack>
            <big_control />
            <text
                left=100
                content="Hello World".to_string()
                colour=(0, 255, 255, 255)
                font_size=10
            />
        </horizontal_stack>
    )
}

#[topo::nested]
fn app_root() -> RootNode<FirstState> {
    mox!(
        <root>
            <vertical_stack>
                <small_control/>
                <other_control/>
                <other_control/>
                <other_control/>
                <other_control/>
                <other_control/>
                <other_control/>
                <other_control/>
                <other_control/>
                <other_control/>
                <other_control/>
            </vertical_stack>
        </root>
    )
}
fn main() {
    std::env::set_var("RUST_LOG", "info");
    Application::<FirstState>::new()
        .use_logging()
        .with_builders(&mut standard_builders(FirstState::default(), app_root))
        .with_builder(standard_pathfinder_rendering())
        .with_builder(pathfinder_renderer())
        //.with_builder(world_logging())
        .build()
        .unwrap()
        .run_until_closed();
}