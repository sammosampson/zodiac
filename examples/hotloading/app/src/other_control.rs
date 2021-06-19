use app_state::TestState;
use illicit::from_env;
use mox::mox;
use moxie::*;
use zodiac::*;
use zodiac_html::*;

use crate::big_control::*;
use crate::state::*;

#[topo::nested]
#[from_env(state: &Key<TestState>)]
pub fn other_control() -> Node {
    increase_border_size();            
    mox!(
        <div>
            { big_control(state.border_size) }
        </div>
    )
}