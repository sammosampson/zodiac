use app_state::*;
use mox::mox;
use zodiac::*;
use zodiac_html::*;

use crate::small_control::*;
use crate::other_control::*;


#[topo::nested]
pub fn app_root() -> RootNode<TestState> {
    mox!(
        <root>
            <div>
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
            </div>
        </root>
    )
}