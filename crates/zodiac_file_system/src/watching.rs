extern crate hotwatch;

use std::error;
use std::fmt;
use hotwatch::{Hotwatch, Event};
use std::path::{ Path };

#[derive(Debug)]
pub enum WriteWatcherError {
    Io(std::io::Error),
    Other(hotwatch::notify::Error)
}

impl fmt::Display for WriteWatcherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WriteWatcherError::Io(..) =>
                write!(f, "Io error occured"),
            WriteWatcherError::Other(..) =>
                write!(f, "Other error"),
        }
    }
}

impl error::Error for WriteWatcherError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            WriteWatcherError::Io(ref e) => Some(e),
            WriteWatcherError::Other(ref e) => Some(e)
        }
    }
}

impl From<hotwatch::Error> for WriteWatcherError {
    fn from(err: hotwatch::Error) -> Self {
        match err {
            hotwatch::Error::Io(error) => Self::Io(error),
            hotwatch::Error::Notify(error) => Self::Other(error)
        }
    }
}

pub trait WriteWatcher {
    fn watch_for_writes_to(&mut self, path: &str, output_to: fn(&Path)) -> Result<(), WriteWatcherError>;
}

impl WriteWatcher for Hotwatch {
    fn watch_for_writes_to(&mut self, path: &str, on_file_write_callback: fn(&Path)) -> Result<(), WriteWatcherError> {
        self.watch(path, move |event: Event| {
            if let Event::Write(path) = event {
                on_file_write_callback(&path);
            }
        })?;
        Ok(())
    }
}