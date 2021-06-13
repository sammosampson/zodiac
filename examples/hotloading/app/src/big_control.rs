use mox::mox;
use zodiac::*;
use zodiac_rendering_glium::*;

#[topo::nested]
pub fn big_control() -> Node {
    mox!(
        <rect
            colour=(255, 255, 0, 255)
            stroke_colour=(255, 255, 255, 255)
            stroke_width=2
            corner_radii=(1, 1, 0, 0)
        />
    )
}