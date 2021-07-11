use app_state::*;
use mox::mox;
use zodiac::*;
use zodiac_html::*;
use crate::small_control::*;
use crate::big_control::*;


#[topo::nested]
pub fn app_root() -> RootNode<TestState> {
    mox!(
        <root>
            <window width=px(1000) height=px(1000) title="Demo".into()>
                <small_control />
                <big_control />
            </window>
        </root>
    )
}