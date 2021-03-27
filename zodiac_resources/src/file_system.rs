use std::{ fs, path::{ PathBuf } };

pub fn create_file_paths(relative_folder_path: &'static str) -> FilePaths {
    FilePaths::new(relative_folder_path)
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FilePaths {
    zod_folder_path: &'static str
}

impl FilePaths {
    pub fn new(zod_folder_path: &'static str) -> Self {
        FilePaths {
            zod_folder_path
        }
    }

    pub fn get_zod_folder(&self) -> PathBuf {
        PathBuf::from(self.zod_folder_path)
    }
}

impl Default for FilePaths {
    fn default() -> Self {
        Self {
            zod_folder_path: ""
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
    let zod_app_file_path = file_paths.get_zod_folder().join(format!("{}.zod", zod_file));
    Ok(fs::read_to_string(zod_app_file_path).map_err(|error|Error::FailedToReadZodFile(error)))?
}