extern crate hotwatch;
extern crate zodiac_parsing;
extern crate zodiac_file_system;

use std::{
    fs,
    path::{ PathBuf }
};
use zodiac_file_system::monitoring::{RecursiveFolderFileMonitor, RecursiveFileLister };
use zodiac_parsing::lexing::Lexer;

fn parse(path: PathBuf) {
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

fn main() {
    let mut monitor = RecursiveFolderFileMonitor::new(RecursiveFileLister{});
    monitor.monitor("test_zods", "zod").expect("failed to monitor folder");
    for path in monitor {
        parse(path);
    }
}