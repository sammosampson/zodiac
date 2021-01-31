use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum Error {
    FailedToGetExePath
}

pub fn from_relative_exe_path(rel_path: &str) -> Result<PathBuf, Error> {
    let exe_file_name = ::std::env::current_exe().map_err(|_| Error::FailedToGetExePath)?;
    let exe_path: &Path = exe_file_name.parent().ok_or(Error::FailedToGetExePath)?;
    Ok(exe_path.join(rel_path))
}