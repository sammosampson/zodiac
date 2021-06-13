use mox::mox;
use zodiac::*;
use crate::big_control::*;

#[topo::nested]
pub fn other_control() -> Node {
    mox!(
        <horizontal_stack>
            <big_control />
            /* <text
                left=100
                content="Hello World".to_string()
                colour=(0, 255, 255, 255)
                font_size=10
            />*/
        </horizontal_stack>
    )
}