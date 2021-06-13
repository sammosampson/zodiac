use app_state::*;
use mox::mox;
use zodiac::*;

use crate::small_control::*;
use crate::other_control::*;


#[topo::nested]
pub fn app_root() -> RootNode<TestState> {
    mox!(
        <root>
            <vertical_stack>
                <small_control />
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