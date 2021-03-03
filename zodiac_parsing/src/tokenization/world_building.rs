use legion::*;
use zodiac_entities::world_building::*;
use crate::tokenization::abstract_syntax::*;

pub trait WorldBuilder {
    fn build_world(&mut self, world: &mut World) -> Result<(), AbstractSyntaxTokenError>;
}

impl<'a, I: Iterator<Item=AbstractSyntaxTokenResult<'a>>> WorldBuilder for I {
    fn build_world(&mut self, world: &mut World) -> Result<(), AbstractSyntaxTokenError> { 
        let mut entity_builder = WorldEntityBuilder::for_world(world);
        for token_result in self {
            match token_result {
                Ok(value) => {
                    match value {
                        AbstractSyntaxToken::Circle => entity_builder.create_circle_entity(),
                        AbstractSyntaxToken::Rectangle => entity_builder.create_rectangle_entity(),
                        AbstractSyntaxToken::Text => entity_builder.create_canvas_layout_content_entity(),
                        AbstractSyntaxToken::CanvasLayoutContent => entity_builder.create_canvas_layout_content_entity(),
                        AbstractSyntaxToken::HorizontalLayoutContent => entity_builder.create_horizontal_layout_content_entity(),
                        AbstractSyntaxToken::VerticalLayoutContent => entity_builder.create_vertical_layout_content_entity(),
                        AbstractSyntaxToken::Left(left) => entity_builder.add_left_component(left),
                        AbstractSyntaxToken::Top(top) => entity_builder.add_top_component(top),
                        AbstractSyntaxToken::Width(width) => entity_builder.add_width_component(width),
                        AbstractSyntaxToken::Height(height) => entity_builder.add_height_component(height),
                        AbstractSyntaxToken::Radius(radius) => entity_builder.add_radius_component(radius),
                        AbstractSyntaxToken::StrokeWidth(width) => entity_builder.add_stroke_width_component(width),
                        AbstractSyntaxToken::Content(content) => { 
                            let mut position = 0;
                            for character in content.chars() {
                                entity_builder.create_glyph_entity();
                                entity_builder.add_character_component(character, position);
                                entity_builder.complete_entity();
                                position = position + 1;
                            }
                        },
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