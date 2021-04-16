use shrev::*;
use std::time::*;
use legion::*;
use legion::systems::*;
use zodiac_entities::*;
use zodiac_source::*;
use crate::*;

pub fn standard_source_file_building(relative_zod_folder_path: &'static str) -> SourceFileBundleBuilder {
    SourceFileBundleBuilder::new(relative_zod_folder_path)
}

pub fn standard_source_building() -> SourceBuildBundleBuilder<FileSourceReader> {
    SourceBuildBundleBuilder::<FileSourceReader>::new()
}

#[derive(Debug)]
pub struct SourceFileBundleBuilder {
    relative_zod_folder_path: &'static str,
    file_monitor_poll: Duration
}

impl SourceFileBundleBuilder {
    pub fn new(relative_zod_folder_path: &'static str) -> Self  {
        Self {
            relative_zod_folder_path,
            file_monitor_poll: Duration::from_millis(50)    
        }
    }    
}

impl ApplicationBundleBuilder for SourceFileBundleBuilder {
    fn description(&self) -> String {
        "standard file system source build".to_string()
    }

    fn setup_build_systems(&self, builder: &mut Builder) {
        builder
            .add_thread_local(recurisve_source_location_build_system::<FileSystemSourceLocationWalker, FileSystemSourceLocationIterator>())
            .flush()
            .add_thread_local(source_file_monitoring_system::<FileSystemFileMonitor>());
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
        let file_paths = FilePaths::new(self.relative_zod_folder_path);
        resources.insert(file_paths);
        resources.insert(create_file_system_source_location_walker());
        resources.insert(monitor_files(file_paths, self.file_monitor_poll)?);
        resources.insert(create_source_file_reader());
        Ok(())
    }
}