use std::{ fs, path::{ PathBuf } };

pub fn create_file_paths(relative_folder_path: &'static str) -> FilePaths {
    FilePaths::new(relative_folder_path)
}

#[derive(Debug)]
pub enum FilePathError {
    ManifestDirectoryEnvironmentVariableNotSet
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

    pub fn get_absolute_folder_path(&self) -> Result<PathBuf, FilePathError> {
        let path = std::env::var("CARGO_MANIFEST_DIR").map_err(|_|FilePathError::ManifestDirectoryEnvironmentVariableNotSet)?;
        Ok(PathBuf::from(path).join(self.relative_folder_path))
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
    FilePathError(FilePathError)
}

impl From<FilePathError> for Error {
    fn from(error: FilePathError) -> Error {
        Error::FilePathError(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        Error::FailedToReadZodFile(error)
    }
}

pub fn load_zod_file(zod_file: &str, file_paths: FilePaths) -> Result<String, Error>{
    let zod_app_file_path = file_paths.get_absolute_folder_path()?.join(format!("{}.zod", zod_file));
    Ok(fs::read_to_string(zod_app_file_path)?)
}