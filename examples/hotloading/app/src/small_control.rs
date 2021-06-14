use mox::mox;
use zodiac::*;
use zodiac_html::*;

#[topo::nested]
pub fn small_control() -> Node {
    mox!(
        <style text_size=px(10) text_colour=rgb(255, 255, 255) />
    )
}