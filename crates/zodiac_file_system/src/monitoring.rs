use crate::{
    listing::{ FileLister, FileListerError },
    watching::{WriteWatcher, WriteWatcherError}
};

pub struct RecursiveFolderFileMonitor<'a, W, L> {
    watcher: W,
    lister:  L,
    path: &'a str
}

impl <'a, W, L> RecursiveFolderFileMonitor<'a, W, L> where W: WriteWatcher, L: FileLister<'a>, {
    pub fn monitor(watcher: W, lister: L, path: &'a str) -> Self {
        Self {
            watcher,
            lister,
            path
        }
    }

    pub fn get_files(&mut self) -> Result<&Vec<&str>, FileListerError> {
        self.lister.list_files(self.path)
    }
}