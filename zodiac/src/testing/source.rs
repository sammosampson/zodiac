use std::collections::HashMap;
use legion::*;
use legion::systems::*;
use shrev::EventChannel;
use zodiac_entities::*;
use zodiac_source::*;
use zodiac_source_filesystem::*;

pub fn test_source_file_building() -> TestSourceFileBundleBuilder {
    TestSourceFileBundleBuilder::default()
}

pub fn test_source_building() -> SourceBuildBundleBuilder<TestSourceReader> {
    SourceBuildBundleBuilder::<TestSourceReader>::new()
}

#[derive(Default, Debug)]
pub struct TestSourceFileBundleBuilder;

impl ApplicationBundleBuilder for TestSourceFileBundleBuilder {
    fn description(&self) -> String {
        "test file system source build".to_string()
    }

    fn setup_build_systems(&self, builder: &mut Builder) {
        builder
            .add_thread_local(recurisve_source_location_build_system::<TestSourceLocationWalker, TestSourceLocationIterator>())
            .flush()
            .add_thread_local(source_file_monitoring_system::<TestFileMonitor>());
    }

    fn setup_layout_systems(&self, _: &mut Builder) {
    }

    fn setup_rendering_systems(&self, _: &mut Builder) {
    }

    fn setup_cleanup_systems(&self, builder: &mut Builder) {            
        builder
            .add_thread_local(remove_source_file_initial_read_system())
            .add_thread_local(remove_source_file_change_system())
            .add_thread_local(remove_source_file_creation_system())
            .add_thread_local(remove_source_file_removal_system())
            .add_thread_local(remove_rebuild_system())
            .add_thread_local(remove_build_error_system());
    }

    fn setup_resources(&self, resources: &mut Resources, _: &mut EventChannel<SystemEvent>) -> Result<(), ZodiacError>  {
        resources.insert(FilePaths::new(""));
        resources.insert(create_test_source_location_walker()); 
        resources.insert(create_test_monitor());   
        resources.insert(create_test_source_reader());    

        Ok(())
    }
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