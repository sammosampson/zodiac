use legion::*;
use crate::components::*;

#[system(for_each)]
pub fn render_rectangles(
    _rectangle: &Rectangle, 
    position: &Position, 
    dimensions: &Dimensions, 
    colour: &Colour,
    stroke_colour: &StrokeColour,
    stroke_width: &StrokeWidth, 
    corner_radii: &CornerRadii) {
        println!("{:?}", position);
        println!("{:?}", dimensions);
        println!("{:?}", colour);
        println!("{:?}", stroke_colour);
        println!("{:?}", stroke_width);
        println!("{:?}", corner_radii);
}