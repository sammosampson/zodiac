extern crate hotwatch;
extern crate zodiac_parsing;
extern crate zodiac_file_system;

use std::{
    fs,
    thread,
    time,
    path::{ Path }
};
use hotwatch::Hotwatch;
use zodiac_file_system::watching::WriteWatcher;
use zodiac_parsing::lexing::Lexer;

fn parse(path: &Path) {
    let file = fs::read_to_string(path);

    match file {
        Ok(text) => {
            let lexer = Lexer::parse(text.as_str());
            println!("{:?}", text);
            for token in lexer {
                match token {
                    Ok(value) => println!("{:?}", value),
                    Err(error) => println!("{:?}", error) 
                }
            }
        },
        Err(error) => panic!(error)
    }   
}

fn create_watcher() -> Hotwatch {
    Hotwatch::new().expect("watcher bust")
}

fn watch(watcher: &mut impl WriteWatcher) {
    watcher.watch_for_writes_to("test_zods", parse).expect("watcher bust");
}

fn main() {
    let mut watcher = create_watcher();
    watch(&mut watcher);
    thread::sleep(time::Duration::from_secs(30));
}