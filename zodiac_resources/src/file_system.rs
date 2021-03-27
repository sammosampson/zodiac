use std::{
    env,
    fs,
    path::{Path, PathBuf}
};

pub fn create_file_paths(relative_folder_path: &'static str) -> FilePaths {
    FilePaths::new(relative_folder_path)
}

pub struct FilePaths {
    relative_folder_path: &'static str
}

impl FilePaths {
    pub fn new(relative_folder_path: &'static str) -> Self {
        FilePaths {
            relative_folder_path
        }
    }

    pub fn get_absolute_path(&self) -> PathBuf {
        let path = PathBuf::from("C:\\work\\other\\zodiac\\zodiac_examples");//env::current_dir().unwrap();
        path.join(self.relative_folder_path)
    }
}

#[derive(Debug)]
pub enum Error {
    FailedToGetExePath,
    FailedToReadZodFile(std::io::Error),
}

pub fn load_app_zod_file_from_relative_path(relative_path: &str) -> Result<String, Error> {
    load_zod_file_from_relative_path("app", relative_path)
}

pub fn load_zod_file_from_relative_path(zod_file: &str, relative_path: &str) -> Result<String, Error>{
    let zod_folder_path = get_full_path(relative_path)?;
    let zod_app_file_path = zod_folder_path.join(format!("{}.zod", zod_file));
    Ok(fs::read_to_string(zod_app_file_path).map_err(|error|Error::FailedToReadZodFile(error)))?
}

fn get_full_path(relative_path: &str) -> Result<PathBuf, Error> {
    let exe_file_name = ::std::env::current_exe().map_err(|_| Error::FailedToGetExePath)?;
    let exe_path: &Path = exe_file_name.parent().ok_or(Error::FailedToGetExePath)?;
    Ok(exe_path.join(relative_path))
}