use std::collections::*;
use std::path::PathBuf;
use std::sync::mpsc::*;
use std::time::Duration;
use notify::{ RecommendedWatcher, DebouncedEvent, Watcher, RecursiveMode };
use legion::*;
use legion::systems::*;

use zodiac_entities::components::*;
use crate::file_system::*; 

pub fn monitor_files(paths: FilePaths, watch_check: Duration) -> Result<FileMonitor, FileMonitorError> {
    FileMonitor::watch(paths, watch_check)
}

pub enum FileMonitorWatchError {
    NoLongerMonitoring,
    NoFileChanges
}

#[derive(Debug)]
pub enum FileMonitorError {
    WatchError(notify::Error),
    FilePathError(FilePathError)
}

pub enum FileMonitorFileChange {
    Create(PathBuf),
    Modify(PathBuf),
    Delete(PathBuf),
}

pub struct FileMonitor {
    #[allow(dead_code)]
    watcher: RecommendedWatcher,
    rx: Receiver<DebouncedEvent>
}

impl From<notify::Error> for FileMonitorError {
    fn from(error: notify::Error) -> FileMonitorError {
        FileMonitorError::WatchError(error)
    }
}

impl From<FilePathError> for FileMonitorError {
    fn from(error: FilePathError) -> FileMonitorError {
        FileMonitorError::FilePathError(error)
    }
}

impl FileMonitor {
    pub fn watch(paths: FilePaths, watch_check: Duration) -> Result<Self, FileMonitorError> {
        let (tx, rx) = channel();
        let mut watcher: RecommendedWatcher = Watcher::new(tx, watch_check)?;
        let path = paths.get_absolute_folder_path()?;        
        watcher.watch(path, RecursiveMode::Recursive)?;

        let monitor = Self {
            watcher,
            rx  
        };

        Ok(monitor)
    }
    pub fn try_get_file_changed(&self) -> Result<FileMonitorFileChange, FileMonitorWatchError> {
        match self.rx.try_recv() {
            Ok(event) => {
                match event {
                    DebouncedEvent::Create(path) => Ok(FileMonitorFileChange::Create(path)),
                    DebouncedEvent::Write(path) => Ok(FileMonitorFileChange::Modify(path)),
                    DebouncedEvent::Remove(path) => Ok(FileMonitorFileChange::Delete(path)),
                    _ => Err(FileMonitorWatchError::NoFileChanges)
                }
            },
            Err(err) => match err {
                TryRecvError::Empty => Err(FileMonitorWatchError::NoFileChanges),
                TryRecvError::Disconnected => Err(FileMonitorWatchError::NoLongerMonitoring),
            }
        }
    }
}

pub fn create_source_file_id_lookup() -> SourceFileIdLookup {
    SourceFileIdLookup {
        inner: HashMap::<PathBuf, u64>::default(),
        current_id: 0
    }   
}

pub fn create_source_file_path_lookup() -> SourceFilePathLookup {
    SourceFilePathLookup {
        inner: HashMap::<u64, PathBuf>::default()
    }   
}

pub struct SourceFileIdLookup {
    inner: std::collections::HashMap<PathBuf, u64>,
    current_id: u64
}

impl SourceFileIdLookup {
    fn stash_file_path(&mut self, path: &PathBuf) -> u64 {
        if let Some(id) = self.inner.get(path) {
            return *id;
        }
        let key = PathBuf::from(path);
        self.current_id += 1;
        self.inner.insert(key, self.current_id);
        self.current_id
    }
    
    fn remove_file_path(&mut self, path: &PathBuf) -> Option<u64> {
        self.inner.remove(path)
    }
}

pub struct SourceFilePathLookup {
    inner: HashMap<u64, PathBuf>
}

impl SourceFilePathLookup {
    fn stash_file_path(&mut self, id: u64, path: PathBuf) {
        self.inner.insert(id, path);
    }
    
    fn remove_file_path(&mut self, id: u64) {
        self.inner.remove(&id);
    }
}

fn queue_source_file_change(command_buffer: &mut CommandBuffer, file_id: u64) {
    command_buffer.push((SourceFileChange { file_id },));
}

fn queue_source_file_removal(command_buffer: &mut CommandBuffer,file_id: u64) {
   command_buffer.push((SourceFileRemoval { file_id },));
}

fn queue_fatal_error (command_buffer: &mut CommandBuffer, error: FatalErrorReason) {
    command_buffer.push((FatalError { error },));
}

#[system(simple)]
#[write_component(SourceFileChange)]
pub fn source_file_monitoring(
    command_buffer: &mut CommandBuffer,
    #[resource] monitor: &FileMonitor,
    #[resource] file_id_lookup: &mut SourceFileIdLookup,
    #[resource] file_path_lookup: &mut SourceFilePathLookup) {
    match monitor.try_get_file_changed() {
        Ok(event) => match event {
            FileMonitorFileChange::Create(_) => {},
            FileMonitorFileChange::Delete(path) => {
                if let Some(file_id) = file_id_lookup.remove_file_path(&path) {
                    file_path_lookup.remove_file_path(file_id);
                    queue_source_file_removal(command_buffer, file_id);
                }
            },
            FileMonitorFileChange::Modify(path) => {
                let file_id = file_id_lookup.stash_file_path(&path);
                file_path_lookup.stash_file_path(file_id, path);
                queue_source_file_change(command_buffer, file_id);
            },
        },
        Err(err) => match err {
            FileMonitorWatchError::NoLongerMonitoring => queue_fatal_error(command_buffer, FatalErrorReason::FileMonitoringFailed),
            _ => {}
        }
    }
}

#[system(for_each)]
pub fn remove_source_file_changed(change: &SourceFileChange, command_buffer: &mut CommandBuffer, entity: &Entity) {
    println!("change {:?}", change);
    command_buffer.remove(*entity);
}

#[system(for_each)]
pub fn remove_source_file_deleted(removal: &SourceFileRemoval, command_buffer: &mut CommandBuffer, entity: &Entity) {
    println!("removed {:?}", removal);
    command_buffer.remove(*entity);
}

#[system(for_each)]
pub fn process_fatal_error(error: &FatalError, command_buffer: &mut CommandBuffer, entity: &Entity) {
    println!("error {:?}", error.error);
    command_buffer.remove(*entity);
}
