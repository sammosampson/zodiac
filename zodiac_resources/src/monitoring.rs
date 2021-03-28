use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
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

impl FileMonitor {
    pub fn watch(paths: FilePaths, watch_check: Duration) -> Result<Self, FileMonitorError> {
        let (tx, rx) = channel();
        let mut watcher: RecommendedWatcher = Watcher::new(tx, watch_check)?;
        let path = paths.get_absolute_folder_path();        
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

pub fn queue_source_file_change(command_buffer: &mut CommandBuffer, path: PathBuf) {
    command_buffer.push((SourceFileChange { file_hash: calculate_hash(&path) },));
}

pub fn queue_source_file_removal(command_buffer: &mut CommandBuffer, path: PathBuf) {
    command_buffer.push((SourceFileRemoval { file_hash: calculate_hash(&path) },));
}

pub fn queue_fatal_error (command_buffer: &mut CommandBuffer, error: FatalErrorReason) {
    command_buffer.push((FatalError { error },));
}

#[system(simple)]
#[write_component(SourceFileChange)]
pub fn source_file_monitoring(command_buffer: &mut CommandBuffer, #[resource] monitor: &FileMonitor) {
    match monitor.try_get_file_changed() {
        Ok(event) => match event {
            FileMonitorFileChange::Create(_) => {},
            FileMonitorFileChange::Delete(path) => queue_source_file_removal(command_buffer, path),
            FileMonitorFileChange::Modify(path) => queue_source_file_change(command_buffer, path),
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

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
