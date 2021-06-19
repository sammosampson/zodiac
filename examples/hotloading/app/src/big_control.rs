use mox::mox;
use zodiac::*;
use zodiac_html::*;

#[topo::nested]
pub fn big_control(size: u64) -> Node {
    mox!(
        <span style=big_control_style(size) />
    )
}

#[topo::nested]
pub fn big_control_style(size: u64) -> Node {
    mox!(
        <style
            border_style=BorderStyles::Double
            border_width=px(size as u8)
            border_colour=rgb(100, 100, 100)
            background_colour=rgb(100, 255, 255) />
    )
}