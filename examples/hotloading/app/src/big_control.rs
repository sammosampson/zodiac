use mox::mox;
use zodiac::*;
use zodiac_html::*;

#[topo::nested]
pub fn big_control() -> Node {
    mox!(
        <span style=big_control_style() />
    )
}

#[topo::nested]
pub fn big_control_style() -> Node {
    mox!(
        <style text_size=px(25) text_colour=rgb(255, 255, 255) />
    )
}