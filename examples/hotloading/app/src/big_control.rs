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
            border=(px(size as u8), BorderStyles::Double, rgb(100, 100, 100)).into()
            background_colour=rgb(100, 255, 255) />
    )
}