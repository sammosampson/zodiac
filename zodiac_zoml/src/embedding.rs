use zodiac::*;
use crate::components::*;

element! {
    <canvas>
    [LayoutContent::canvas()]
    attributes {
        left(u16)
        top(u16)
        width(u16)
        height(u16)
    }
}

element! {
    <vertical_stack>
    [LayoutContent::vertical()]
    attributes {
        left(u16)
        top(u16)
        width(u16)
        height(u16)
    }
}

element! {
    <horizontal_stack>
    [LayoutContent::horizontal()]
    attributes {
        left(u16)
        top(u16)
        width(u16)
        height(u16)
    }
}

element! {
    <circle>
    [Circle::default()]
    extra_components {
        [Renderable::default()]
    }
    attributes {
        left(u16)
        top(u16)
        radius(u16)
        colour((u8, u8, u8, u8))
        stroke_colour((u8, u8, u8, u8))
        stroke_width(u16)
    }
}

element! {
    <rect>    
    [Rectangle::default()]
    extra_components {
        [Renderable::default()]
    }
    attributes {
        left(u16)
        top(u16)
        width(u16)
        height(u16)
        colour((u8, u8, u8, u8))
        stroke_colour((u8, u8, u8, u8))
        stroke_width(u16)
        corner_radii((u16, u16, u16, u16))
    }
}