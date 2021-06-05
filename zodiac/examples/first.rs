use mox::mox;
use zodiac::initialisation::*;
use zodiac_source::application_state::*;
use zodiac_source::embedding::*;

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
            corner_radii=(0, 0, 0, 0)
        />
    )
}

#[topo::nested]
fn other_control() -> Node {
    mox!(
        <horizontal_stack>
            <rect
                colour=(255, 255, 255, 255)
                stroke_colour=(0, 200, 255, 255)
                stroke_width=5
                corner_radii=(0, 0, 0, 0)
            />
            <text
                content="Hello World".to_string()
                colour=(0, 255, 255, 255)
            />
        </horizontal_stack>
    )
}

#[topo::nested]
fn app_root() -> RootNode<FirstState> {
    mox!(
        <root>
            <horizontal_stack>
                <other_control />
                <big_control />
                <big_control />
                <big_control />
                <big_control />
                <big_control />
                <big_control />
                <big_control />
                <vertical_stack>
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
                    <other_control/>
                    <horizontal_stack>
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
                        <other_control/>
                    </horizontal_stack>
                </vertical_stack>
            </horizontal_stack>
        </root>
    )
}
fn main() {
    Application::new()
        .use_logging()
        .with_builders(&mut standard_builders(FirstState::default(), app_root))
        .build()
        .unwrap()
        .run_until_closed();
}