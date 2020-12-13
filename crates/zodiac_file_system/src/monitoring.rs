

use std::error::Error;
use glob::glob;
use std::path::{ PathBuf };
pub struct RecursiveFolderFileMonitor {
    files: Vec<PathBuf>
}

impl<'a> RecursiveFolderFileMonitor {
    fn get_pattern (folder: &'a str, file_type: &'a str) -> String {
        format!("{}/**/*.{}", folder, file_type)
    }

    fn get_files_initially(pattern: String) -> Result<Vec<PathBuf>, Box<dyn Error>> {
        let mut initial_files: Vec<PathBuf> = Vec::new();
        for entry in glob(&pattern)? {
            initial_files.push(entry?)
        }
        Ok(initial_files)
    }
    
    pub fn monitor(folder: &str, file_type: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            files: RecursiveFolderFileMonitor::get_files_initially(
                RecursiveFolderFileMonitor::get_pattern(folder, file_type))?
        })
    }
}

impl Iterator for RecursiveFolderFileMonitor {
    type Item = PathBuf;
    fn next(&mut self) -> Option<PathBuf> {
        self.files.pop()
    }
}