use legion::*;
use zodiac_parsing::tokenization::abstract_syntax::*;
use crate::world_building::entities::*;

pub trait WorldBuilder {
    fn build_world(&mut self, world: &mut World) -> Result<(), AbstractSyntaxTokenError>;
}

impl<I: Iterator<Item=AbstractSyntaxTokenResult>> WorldBuilder for I {
    fn build_world(&mut self, world: &mut World) -> Result<(), AbstractSyntaxTokenError> { 
        let mut entity_builder = WorldEntityBuilder::for_world(world);
        for token_result in self {
            match token_result {
                Ok(value) => {
                    match value {
                        AbstractSyntaxToken::Circle => entity_builder.create_circle_entity(),
                        AbstractSyntaxToken::Rectangle => entity_builder.create_rectangle_entity(),
                        AbstractSyntaxToken::Text => entity_builder.create_text_entity(),
                        AbstractSyntaxToken::CanvasLayoutContent => entity_builder.create_canvas_layout_content_entity(),
                        AbstractSyntaxToken::HorizontalLayoutContent => entity_builder.create_horizontal_layout_content_entity(),
                        AbstractSyntaxToken::Left(left) => entity_builder.add_left_component(left),
                        AbstractSyntaxToken::Top(top) => entity_builder.add_top_component(top),
                        AbstractSyntaxToken::Width(width) => entity_builder.add_width_component(width),
                        AbstractSyntaxToken::Height(height) => entity_builder.add_height_component(height),
                        AbstractSyntaxToken::Radius(radius) => entity_builder.add_radius_component(radius),
                        AbstractSyntaxToken::StrokeWidth(width) => entity_builder.add_stroke_width_component(width),
                        AbstractSyntaxToken::GlyphIndex(index) => entity_builder.add_glyph_index_component(index),
                        AbstractSyntaxToken::Colour((r, g, b ,a)) => entity_builder.add_colour_component(r, g, b ,a),
                        AbstractSyntaxToken::StrokeColour((r, g, b ,a)) => entity_builder.add_stroke_colour_component(r, g, b ,a),
                        AbstractSyntaxToken::CornerRadii((left_top, right_top, right_bottom, left_bottom)) => entity_builder.add_corner_radii_component(
                            left_top,
                            right_top,
                            right_bottom,
                            left_bottom),
                        AbstractSyntaxToken::CompleteControl => entity_builder.complete_entity(),
                    }
                },
                Err(err) => return Err(err)
            }
        }
        Ok(())
    }
}