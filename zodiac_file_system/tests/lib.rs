#[cfg(test)]
mod tests 
{
    extern crate zodiac_file_system;

    use std::path::{ Path };
    use std::error;
    use std::fmt;

    use zodiac_file_system:: {
        watching::{ WriteWatcher, WriteWatcherError },
        listing::{ FileLister, FileListerError },
        monitoring::RecursiveFolderFileMonitor
    };
    
    struct TestFileSystem<'a> {
        files: Vec<&'a str>,
        path: &'a str
    }

    impl <'a> TestFileSystem <'a> {
        fn setup(path: &'a str) -> Self {
            Self {
                files: Default::default()
                path
            }
        }

        fn add_file(&mut self, file: &'a str) {
            self.files.push(file);
        }

        fn get_files(&mut self) -> Vec<&'a str>{
            self.files
        }
    }

    impl <'a> FileLister<'a> for TestFileSystem<'a> {
        fn list_files(&mut self, path: &'a str) -> Result<&Vec<&str>, FileListerError> {
            if self.path != path {
                return Err(FileListerError::Io);
            }
            Ok(self.files)
        }
    }

    impl <'a> WriteWatcher for TestFileSystem<'a> {
        fn watch_for_writes_to(&mut self, path: &str, output_to: fn(&Path)) -> Result<(), WriteWatcherError>{
            if self.path != path {
                return Err(WriteWatcherError::Io());
            }
            Ok(())
        }
    }

    #[test]
    fn file_system_lists_files_initially() {
        let path_name = "test";
        
        let lister = TestFileSystem::setup(path_name);
        lister.add_file("file1");
        lister.add_file("file2");

        let watcher = TestFileSystem::setup(path_name);
        let mut monitor = RecursiveFolderFileMonitor::monitor(watcher, lister, path_name);
        
        assert_eq!(lister.get_files(), *monitor.get_files().unwrap());
    }

    #[test]
    fn file_system_lists_only_changed_files_second_time() {
        let path_name = "test";

        let lister = TestFileSystem::setup(path_name);
        
        let watcher = TestFileSystem::setup(path_name);
        watcher.add_file("file1");
        watcher.add_file("file2");

        let mut monitor = RecursiveFolderFileMonitor::monitor(watcher, lister, path_name);
        
        *monitor.get_files().unwrap();
        assert_eq!(watcher.get_files(), *monitor.get_files().unwrap());
    }
}

    