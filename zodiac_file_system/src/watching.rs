use std::error::Error;
use hotwatch::{Hotwatch, Event};
use std::path::{ Path };

pub trait WriteWatcher {
    fn watch_for_writes_to(&mut self, path: &str, output_to: fn(&Path)) -> Result<(), Box<dyn Error>>;
}

impl WriteWatcher for Hotwatch {
    fn watch_for_writes_to(&mut self, path: &str, on_file_write_callback: fn(&Path)) -> Result<(), Box<dyn Error>> {
        self.watch(path, move |event: Event| {
            if let Event::Write(path) = event {
                on_file_write_callback(&path);
            }
        })?;
        Ok(())
    }
}