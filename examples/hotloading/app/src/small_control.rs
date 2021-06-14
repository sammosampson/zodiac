use mox::mox;
use zodiac::*;
use zodiac_html::*;

#[topo::nested]
pub fn small_control() -> Node {
    mox!(
        <span style=small_control_style() />
    )
}

#[topo::nested]
pub fn small_control_style() -> Node {
    mox!(
        <style text_size=px(10) text_colour=rgb(255, 255, 255) />
    )
}