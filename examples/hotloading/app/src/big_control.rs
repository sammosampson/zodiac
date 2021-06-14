use mox::mox;
use zodiac::*;
use zodiac_html::*;

#[topo::nested]
pub fn big_control() -> Node {
    mox!(
        <style text_size=px(25) text_colour=rgb(255, 255, 255) />
    )
}