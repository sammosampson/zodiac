use log::{info};
use legion::*;
use legion::systems::*;
use zodiac_entities::*;
use crate::source::*;
use crate::building::*;

#[system(for_each)]
#[filter(component::<Rebuild>())]
pub fn world_build<T:SourceReader + 'static> (
    entity: &Entity,
    source_implementation: &SourceImplementation,
    command_buffer: &mut CommandBuffer,
    #[resource] source_entity_lookup: &mut SourceEntityLookup,
    #[resource] source_location_lookup: &mut SourceLocationLookup,
    #[resource] source_tokens_lookup: &mut SourceTokensLookup,
    #[resource] source_reader: &mut T
) {

    let mut world_builder = create_world_builder(command_buffer, *entity);
    
    let build_resources_mut = &mut MutableBuildResources::new(&mut world_builder);

    let build_resources = &mut BuildResources {
        source_entity_lookup,
        source_location_lookup,
        source_tokens_lookup,
        source_reader
    };

    let mut builder = create_root_control_builder::<T>(
        *entity,
        *source_implementation);
    
    builder.build_control(build_resources, build_resources_mut);
}

#[system(for_each)]
#[filter(component::<BuildErrorOccurrence>())]
pub fn error_control_for_renderable (
    entity: &Entity,
    renderable: &Renderable,
    command_buffer: &mut CommandBuffer
) {
    let red = Colour::red();
    let square = CornerRadii::default();
    let no_stroke_width = StrokeWidth::default();
    let no_stroke_colour = StrokeColour::default();
    let entity = *entity;
    match renderable.render_type {
        RenderType::Circle => {
            info!("Adding circle error to {:?}", entity);
            command_buffer.add_component(entity, red);
            command_buffer.add_component(entity, no_stroke_colour);
            command_buffer.add_component(entity, no_stroke_width);
        }
        RenderType::Rectangle => {
            info!("Adding rect error to {:?}", entity);
            command_buffer.add_component(entity, red);
            command_buffer.add_component(entity, no_stroke_colour);
            command_buffer.add_component(entity, no_stroke_width);
            command_buffer.add_component(entity, square);
        }
        RenderType::Glyph => {}
    }
}

#[system(for_each)]
#[filter(component::<BuildErrorOccurrence>())]
#[filter(!component::<Renderable>())]
pub fn error_control_for_non_renderable (
    entity: &Entity,
    command_buffer: &mut CommandBuffer
) {
    let mut world_builder = create_world_builder(command_buffer, *entity);
    world_builder.set_root_used();
    
    let error_entity = world_builder.create_rectangle_entity();

    info!("Adding rect error to {:?}", error_entity);
    world_builder.add_component_to_current_entity(Colour::red());
    world_builder.add_component_to_current_entity(CornerRadii::default());
    world_builder.add_component_to_current_entity( StrokeWidth::default());
    world_builder.add_component_to_current_entity(StrokeColour::default());

    world_builder.complete_entity();
}