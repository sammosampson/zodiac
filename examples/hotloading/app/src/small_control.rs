use app_state::TestState;
use illicit::from_env;
use mox::mox;
use moxie::*;
use zodiac::*;
use zodiac_html::*;

#[topo::nested]
#[from_env(state: &Key<TestState>)]
pub fn small_control() -> Node {
    mox!(
        <span style=small_control_style(state.border_size) />
    )
}

#[topo::nested]
pub fn small_control_style(size: u64) -> Node {
    mox!(
        <style
            padding=(px(23).into(), px(23).into(), px(23).into(), px(23).into()).into()
            margin=(px(23).into(), MarginSize::Auto.into(), MarginSize::Inherit.into()).into()
            border_top=(px(size as u16), BorderStyles::Dashed, rgb(200, 200, 200)).into()
            border_radius=px(6)
            background_colour=rgb(100, 100, 200)
        />
    )
}