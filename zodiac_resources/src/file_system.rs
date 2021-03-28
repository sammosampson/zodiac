use std::{ fs, path::{ PathBuf } };

pub fn create_file_paths(relative_folder_path: &'static str) -> FilePaths {
    FilePaths::new(relative_folder_path)
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FilePaths {
    relative_folder_path: &'static str
}

impl FilePaths {
    pub fn new(relative_folder_path: &'static str) -> Self {
        FilePaths {
            relative_folder_path
        }
    }

    pub fn get_absolute_folder_path(&self) -> PathBuf {
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join(self.relative_folder_path)
    }
}

impl Default for FilePaths {
    fn default() -> Self {
        Self {
            relative_folder_path: ""
        }
    }
 }

#[derive(Debug)]
pub enum Error {
    FailedToGetExePath,
    FailedToReadZodFile(std::io::Error),
}

pub fn load_app_zod_file(file_paths: FilePaths) -> Result<String, Error> {
    load_zod_file("app", file_paths)
}

pub fn load_zod_file(zod_file: &str, file_paths: FilePaths) -> Result<String, Error>{
    let zod_app_file_path = file_paths.get_absolute_folder_path().join(format!("{}.zod", zod_file));
    Ok(fs::read_to_string(zod_app_file_path).map_err(|error|Error::FailedToReadZodFile(error)))?
}