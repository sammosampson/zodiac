use app_state::TestState;
use mox::mox;
use moxie::*;
use zodiac::*;
use zodiac_html::*;
use illicit::from_env;
use crate::state::increase_border_size;

#[topo::nested]
#[from_env(state: &Key<TestState>)]
pub fn big_control() -> Node {
    increase_border_size();            
    mox!(
        <div style=big_control_style(state.border_size) />
    )
}

#[topo::nested]
pub fn big_control_style(size: u64) -> Node {
    mox!(
        <style
            padding=(px(23).into(), px(23).into(), px(23).into(), px(23).into()).into()
            border=(px(size as u16), BorderStyles::Double, rgb(100, 100, 100)).into()
            background_colour=rgb(100, 255, 255) />
    )
}