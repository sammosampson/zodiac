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
        <style
            border_width=px(5)
            border_colour=rgb(200, 200, 200)
            border_radius=px(6)
            background_colour=rgb(100, 100, 200)
        />
    )
}