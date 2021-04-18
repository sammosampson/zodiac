use std::collections::HashMap;
use legion::*;
use zodiac_entities::*;
use zodiac_parsing::*;
use zodiac_resources::*;
use zodiac_layout::*;
use zodiac_rendering::*;
use zodiac_rendering_glium::*;

pub fn build_zodiac_systems_schedule() -> Schedule {
    Schedule::builder()
        .add_thread_local(recurisve_source_location_build_system::<TestSourceLocationWalker, TestSourceLocationIterator>())
        .flush()
        .add_thread_local(source_file_monitoring_system::<TestFileMonitor>())
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

pub fn build_zodiac_resources() -> Resources {
    let mut resources=  Resources::default();

    let file_paths = FilePaths::new("");
    resources.insert(file_paths);
    resources.insert(create_test_source_location_walker()); 
    resources.insert(create_test_monitor());   
    resources.insert(create_test_source_reader());    
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

pub enum SourceChangeType {
    Change,
    Creation,
    Remove
}

pub type SourceCodeLookup = HashMap<SourceLocation, String>;

fn create_test_source_location_walker() -> TestSourceLocationWalker {
    TestSourceLocationWalker::new()
}
    
pub fn apply_initial_source(resources: &mut Resources, location: &str, source: &str) {
    resources.get_mut::<TestSourceLocationWalker>().unwrap().add_location_to_walk(SourceLocation::from(location));
    resources.get_mut::<TestSourceReader>().unwrap().add_source(SourceLocation::from(location), String::from(source));
}
    
pub fn apply_changed_source(resources: &mut Resources, location: &str, source: &str) {
    resources.get_mut::<TestFileMonitor>().unwrap().changed_source(SourceLocation::from(location), SourceChangeType::Change);
    resources.get_mut::<TestSourceReader>().unwrap().add_source(SourceLocation::from(location), String::from(source));
}

pub fn apply_created_source(resources: &mut Resources, location: &str, source: &str) {
    resources.get_mut::<TestFileMonitor>().unwrap().changed_source(SourceLocation::from(location), SourceChangeType::Creation);
    resources.get_mut::<TestSourceReader>().unwrap().add_source(SourceLocation::from(location), String::from(source));
}

pub fn delete_source(resources: &mut Resources, location: &str) {
    resources.get_mut::<TestFileMonitor>().unwrap().changed_source(SourceLocation::from(location), SourceChangeType::Remove);
    resources.get_mut::<TestSourceReader>().unwrap().remove_source(SourceLocation::from(location));
}

pub struct TestSourceLocationWalker {
    locations: Vec::<SourceLocation>
}

impl TestSourceLocationWalker {
    fn new() -> Self {
        Self {
            locations: vec!()
        }
    }

    pub fn add_location_to_walk(&mut self, to_add: SourceLocation) {
        self.locations.push(to_add);
    }
}


impl SourceLocationWalker<TestSourceLocationIterator> for TestSourceLocationWalker {
    fn walk(&self, _: &FilePaths) -> Result<TestSourceLocationIterator, SourceLocationWalkerError> {
        Ok(TestSourceLocationIterator::new(self.locations.clone()))     
    }
}

pub struct TestSourceLocationIterator {
    locations: std::vec::IntoIter::<SourceLocation>
}

impl TestSourceLocationIterator {
    fn new(locations: Vec::<SourceLocation>) -> Self {
        Self {
            locations: locations.into_iter()
        }
    }
}

impl Iterator for TestSourceLocationIterator {
    type Item = SourceLocation;

    fn next(&mut self) -> Option<Self::Item> {
        self.locations.next()           
    }
}

fn create_test_monitor() -> TestFileMonitor {
    TestFileMonitor::default()
}

pub type SourceCodeChangeLookup = HashMap<SourceLocation, SourceChangeType>;

#[derive(Default)]
pub struct TestFileMonitor {
    change: Option<(SourceLocation, SourceChangeType)>
}

impl TestFileMonitor {
    fn changed_source(&mut self, location: SourceLocation, change: SourceChangeType) {
        self.change = Some((location, change));
    }
}

impl FileMonitor for TestFileMonitor {
    fn try_get_file_changed(&self) -> Result<FileMonitorFileChange, FileMonitorWatchError> {
        if let Some((location, change)) = &self.change {
            match change {
                SourceChangeType::Change => Ok(FileMonitorFileChange::Modify(location.clone())),
                SourceChangeType::Creation => Ok(FileMonitorFileChange::Create(location.clone())),
                SourceChangeType::Remove => Ok(FileMonitorFileChange::Delete(location.clone())),
            }
        } else {
            Err(FileMonitorWatchError::NoFileChanges)            
        }
    }
}

fn create_test_source_reader() -> TestSourceReader {
    TestSourceReader::new()
}

pub struct TestSourceReader {
    source_code_lookup: SourceCodeLookup
}

impl TestSourceReader {
    fn new() -> Self {
        Self {
            source_code_lookup: SourceCodeLookup::default()
        }
    }

    pub fn add_source(&mut self, location: SourceLocation, source: String) {
        self.source_code_lookup.insert(location, source);
    }

    pub fn remove_source(&mut self, location: SourceLocation) {
        self.source_code_lookup.remove(&location);
    }
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