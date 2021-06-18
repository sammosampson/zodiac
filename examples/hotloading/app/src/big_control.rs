use mox::mox;
use zodiac::*;
use zodiac_html::*;

#[topo::nested]
pub fn big_control(size: u8) -> Node {
    mox!(
        <span style=big_control_style(size) />
    )
}

#[topo::nested]
pub fn big_control_style(size: u8) -> Node {
    mox!(
        <style border_width=px(size) background_colour=rgb(100, 255, 255) />
    )
}