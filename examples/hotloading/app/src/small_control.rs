use mox::mox;
use zodiac::*;
use zodiac_rendering_glium::*;

#[topo::nested]
pub fn small_control() -> Node {
    mox!(
        <circle
            radius=100
            colour=(0, 255, 255, 255)
            stroke_colour=(255, 255, 255, 255)
            stroke_width=3
        />
    )
}