use mox::mox;
use zodiac::*;
use crate::big_control::*;
use zodiac_html::*;

#[topo::nested]
pub fn other_control() -> Node {
    mox!(
        <div>
            { big_control(8) }
        </div>
    )
}