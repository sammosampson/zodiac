use std::collections::HashMap;

use legion::*;
use legion::world::*;
use legion::systems::*;
use zodiac_entities::*;
use zodiac_parsing::*;
use zodiac_resources::*;
use zodiac_layout::*;
use zodiac_rendering::*;
use zodiac_rendering_glium::*;

pub fn build_zodiac_systems_schedule() -> Schedule {
    Schedule::builder()
        .add_thread_local(test_source_location_build_system())
        .flush()
        .add_system(source_token_removal_system())
        .add_system(source_parse_system::<TestSourceReader>())
        .flush()
        .add_system(apply_initially_read_root_source_to_world_system())
        .add_system(apply_created_source_to_world_system())
        .add_system(apply_removed_source_to_world_system())
        .add_system(apply_changed_source_to_world_system())
        .flush()
        .add_system(world_build_system::<TestSourceReader>())
        .flush()
        .add_system(resize_screen_system())
        .add_system(resize_after_rebuild_system())
        .flush()
        .add_system(remove_from_relationship_map_system())
        .add_system(build_relationship_map_system())
        .add_system(remove_from_text_colour_map_system())
        .add_system(build_text_colour_map_system())
        .flush()
        .add_system(format_glyphs_system())
        .flush()
        .add_system(remove_from_left_offset_map_system())
        .add_system(build_left_offset_map_system())
        .add_system(remove_from_top_offset_map_system())
        .add_system(build_top_offset_map_system())
        .add_system(remove_from_minimum_width_map_system())
        .add_system(remove_from_width_map_system())
        .add_system(build_width_map_system())
        .add_system(remove_from_minimum_height_map_system())
        .add_system(remove_from_height_map_system())
        .add_system(build_height_map_system())
        .add_system(build_width_and_height_maps_from_radius_system())
        .add_system(remove_from_layout_type_map_system())
        .add_system(build_layout_type_map_system())
        .flush()
        .add_system(remove_entity_system())
        .flush()
        .add_system(mark_as_mapped_system())
        .add_system(measure_fixed_width_constraints_system())
        .add_system(measure_fixed_height_constraints_system())
        .flush()
        .add_system(resize_system())
        .flush()
        .add_thread_local(queue_render_primitives_system::<GliumRenderQueue>())
        .flush()
        .add_thread_local(remove_layout_change_system())
        .add_thread_local(remove_resized_system())
        .add_thread_local(remove_source_file_initial_read_system())    
        .add_thread_local(remove_source_file_change_system())
        .add_thread_local(remove_source_file_creation_system())
        .add_thread_local(remove_source_file_removal_system())
        .add_thread_local(remove_rebuild_system())
        .flush()
        .build()
}

pub fn build_zodiac_resources(source_code_lookup: SourceCodeLookup) -> Resources {
    let mut resources=  Resources::default();

    resources.insert(create_test_source_reader(source_code_lookup.to_owned()));    
    resources.insert(source_code_lookup);    
    resources.insert(create_source_entity_lookup());
    resources.insert(create_source_tokens_lookup());
    resources.insert(create_source_location_lookup());
    resources.insert(create_text_colour_map());
    resources.insert(create_relationship_map());
    resources.insert(create_layout_type_map());
    resources.insert(create_left_offset_map());
    resources.insert(create_top_offset_map());
    resources.insert(create_width_map());
    resources.insert(create_height_map());
    resources.insert(create_minimum_width_map());
    resources.insert(create_minimum_height_map());
    resources.insert(create_glium_render_queue());

    resources
}

pub fn notify_resize_root_window(world: &mut World, dimensions: (u16, u16)) {
    world.push((RootWindowResized::from(dimensions), ));
}

pub type SourceCodeLookup = HashMap<SourceLocation, String>;

#[system(simple)]
#[write_component(SourceFile)]
pub fn test_source_location_build(
    world: &mut SubWorld,
    command_buffer: &mut CommandBuffer,
    #[resource] source_entity_lookup: &mut SourceEntityLookup,
    #[resource] source_location_lookup: &mut SourceLocationLookup,
    #[resource] source_code_lookup: &mut SourceCodeLookup) {
    
    let source_files: Vec::<&SourceFile> = <&SourceFile>::query().iter(world).collect();

    if source_files.len() > 0 {
        return;
    }
    
    for location in source_code_lookup.keys() {
        let entity = command_buffer.push((SourceFile::default(), SourceFileInitialRead::default()));
        source_location_lookup.insert(entity, location.to_owned());
        source_entity_lookup.insert(location.to_owned(), entity);
    }
}

pub fn create_test_source_reader(source_code_lookup: SourceCodeLookup) -> TestSourceReader {
    TestSourceReader {
        source_code_lookup
    }
}

pub struct TestSourceReader {
    source_code_lookup: SourceCodeLookup
}

impl SourceReader for TestSourceReader {
    fn read_source_at_location(&self, location: &SourceLocation) -> Result<String, SourceReaderError> {
        if let Some(source) = self.source_code_lookup.get(location) {
            return Ok(source.to_owned())
        }
        Err(SourceReaderError::SourceNotFound)
    }

    fn get_relative_source_location(&self, _: &SourceLocation, relative_location: &str) -> Result<SourceLocation, SourceLocationError> {
        Ok(SourceLocation::from(relative_location))
    }
}