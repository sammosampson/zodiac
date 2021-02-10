use legion::*;
use legion::storage::*;
use zodiac_entities::components::*;
use zodiac_parsing::tokenization::abstract_syntax::{
    AbstractSyntaxTokenError,
    AbstractSyntaxTokenResult,
    AbstractSyntaxToken
};

pub trait WorldBuilder {
    fn build_world(&mut self, world: &mut World) -> Result<(), AbstractSyntaxTokenError>;
}

fn create_entity(world: &mut World) -> Entity {
    world.push(())
}

fn add_component<T:Component>(world: &mut World, entity: Entity, component: T) {
    if let Some(mut entry) = world.entry(entity) {
        entry.add_component(component);
    }
}

impl<I: Iterator<Item=AbstractSyntaxTokenResult>> WorldBuilder for I {
    fn build_world(&mut self, world: &mut World) -> Result<(), AbstractSyntaxTokenError> {
        let mut current_entity = create_entity(world);
        let mut last_value = AbstractSyntaxToken::Circle;
        for token_result in self {
            match token_result {
                Ok(value) => {
                    if last_value == AbstractSyntaxToken::CompleteControl {
                        current_entity = create_entity(world);
                    }
                    match value {
                        AbstractSyntaxToken::Container => 
                            add_component(world, current_entity, Container {}),
                        AbstractSyntaxToken::Circle =>
                            add_component(world, current_entity, Circle {}),
                        AbstractSyntaxToken::Rectangle =>
                            add_component(world, current_entity, Rectangle {}),
                        AbstractSyntaxToken::Text =>
                            add_component(world, current_entity, Text {}),
                        AbstractSyntaxToken::Layout(layout_type) => 
                            add_component(world, current_entity, Layout { layout_type }),
                        AbstractSyntaxToken::Position((x, y)) =>
                            add_component(world, current_entity, Position { x, y }),
                        AbstractSyntaxToken::Dimensions((x, y)) =>
                            add_component(world, current_entity, Dimensions { x, y }),
                        AbstractSyntaxToken::Radius(radius) =>
                            add_component(world, current_entity, Radius { radius }),
                        AbstractSyntaxToken::StrokeWidth(width) =>
                            add_component(world, current_entity, StrokeWidth { width }),
                        AbstractSyntaxToken::GlyphIndex(index) =>
                            add_component(world, current_entity, GlyphIndex { index }),
                        AbstractSyntaxToken::Colour((r, g, b ,a)) =>
                            add_component(world, current_entity, Colour { r, g, b ,a }),
                        AbstractSyntaxToken::StrokeColour((r, g, b ,a)) =>
                            add_component(world, current_entity, StrokeColour { r, g, b ,a }),
                        AbstractSyntaxToken::CornerRadii((left_top, right_top, right_bottom, left_bottom)) =>
                            add_component(world, current_entity, CornerRadii {
                                left_top, right_top, right_bottom, left_bottom
                            }),
                        AbstractSyntaxToken::CompleteControl => add_component(world, current_entity, Dirty {}),
                    }
                    last_value = value;
                },
                Err(err) => return Err(err)
            }
        }
        Ok(())
    }
}