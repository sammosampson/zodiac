

use std::error::Error;
use glob::glob;
use std::path::{ PathBuf };
pub struct  RecursiveFolderFileMonitor<L> 
    where L: FileLister {
        lister: L,
        files: Vec<PathBuf>
}

impl<'a, L> RecursiveFolderFileMonitor<L> where L: FileLister {
    fn get_pattern (folder: &'a str, file_type: &'a str) -> String {
        format!("{}/**/*.{}", folder, file_type)
    }

    pub fn new(lister: L) -> Self {
        Self {
            lister,
            files: vec![]
        }
    }
    
    pub fn monitor(&mut self, folder: &str, file_type: &str) -> Result<(), Box<dyn Error>> {
        let initial_files = &self.lister.get_files(RecursiveFolderFileMonitor::<L>::get_pattern(folder, file_type))?;
        for file_path in initial_files {
            self.files.push(file_path.to_path_buf());
        } 
        Ok(())
    }
}

impl<L> Iterator for RecursiveFolderFileMonitor<L> where L: FileLister {
    type Item = PathBuf;
    fn next(&mut self) -> Option<PathBuf> {
        self.files.pop()
    }
}

pub trait FileLister {
    fn get_files(&self, file_pattern: String) -> Result<Vec<PathBuf>, Box<dyn Error>>;
}

pub struct RecursiveFileLister {
}

impl FileLister for RecursiveFileLister {
    fn get_files(&self, pattern: String) -> Result<Vec<PathBuf>, Box<dyn Error>> {
        let mut initial_files: Vec<PathBuf> = Vec::new();
        for entry in glob(&pattern)? {
            initial_files.push(entry?)
        }
        Ok(initial_files)
    }
}

