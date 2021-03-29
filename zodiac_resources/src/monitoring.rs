use std::path::PathBuf;
use std::sync::mpsc::*;
use std::time::Duration;
use notify::{ RecommendedWatcher, DebouncedEvent, Watcher, RecursiveMode };
use crate::file_system::*; 

pub fn monitor_files(paths: FilePaths, watch_check: Duration) -> Result<FileMonitor, FileMonitorError> {
    FileMonitor::watch(paths, watch_check)
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