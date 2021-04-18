use std::sync::mpsc::*;
use std::time::Duration;
use notify::{ RecommendedWatcher, DebouncedEvent, Watcher, RecursiveMode };
use zodiac_parsing::SourceLocation;
use crate::*    ; 

pub fn monitor_files(paths: FilePaths, watch_check: Duration) -> Result<FileSystemFileMonitor, FileMonitorError> {
    FileSystemFileMonitor::watch(paths, watch_check)
}

#[derive(Debug)]
pub enum FileMonitorError {
    WatchError(notify::Error),
    FilePathError(FilePathError)
}

#[derive(Debug)]
pub enum FileMonitorWatchError {
    NoLongerMonitoring,
    NoFileChanges
}

pub enum FileMonitorFileChange {
    Create(SourceLocation),
    Modify(SourceLocation),
    Delete(SourceLocation),
}

pub trait FileMonitor {
    fn try_get_file_changed(&self) -> Result<FileMonitorFileChange, FileMonitorWatchError>;
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

pub struct FileSystemFileMonitor {
    #[allow(dead_code)]
    watcher: RecommendedWatcher,
    rx: Receiver<DebouncedEvent>
}

impl FileSystemFileMonitor {
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
}

impl FileMonitor for FileSystemFileMonitor {
    fn try_get_file_changed(&self) -> Result<FileMonitorFileChange, FileMonitorWatchError> {
        match self.rx.try_recv() {
            Ok(event) => {
                match event {
                    DebouncedEvent::Create(path) => Ok(FileMonitorFileChange::Create(path.to_source_location())),
                    DebouncedEvent::Write(path) => Ok(FileMonitorFileChange::Modify(path.to_source_location())),
                    DebouncedEvent::NoticeRemove(path) => Ok(FileMonitorFileChange::Delete(path.to_source_location())),
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