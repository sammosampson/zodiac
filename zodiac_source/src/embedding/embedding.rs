use zodiac_entities::*;
use crate::changes::*;
use super::nodes::*;
use crate::building::*;

element! {
    <vertical_stack>
    [LayoutContent::vertical()]
}

element! {
    <circle>
    [Renderable::circle()]
    attributes {
        radius(u16)
        stroke_width(u16)
        stroke_colour(StrokeColour)
        content(String)
    }
}