extern crate hotwatch;
use std::vec::Vec;
use std::fmt;

#[derive(Debug)]
pub enum FileListerError {
    Io
}

impl fmt::Display for FileListerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileListerError::Io =>
                write!(f, "Io error occured")
        }
    }
}

pub trait FileLister<'a> {
    fn list_files(&mut self, path: &'a str) -> Result<&Vec<&str>, FileListerError>;
}
