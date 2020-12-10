extern crate hotwatch;
extern crate zodiac_parsing;

use hotwatch::{Hotwatch, Event};
use std::fs;
use std::path::Path;
use zodiac_parsing::lexing::Lexer;
use std::{thread, time};

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



fn main() {
    let mut hotwatch = Hotwatch::new()
        .expect("hotwatch failed to initialize!");

    hotwatch
        .watch("test_zods", |event: Event| {
            if let Event::Write(path) = event {
                parse(&path);
            }
    }).expect("failed to watch file!");

    
    thread::sleep(time::Duration::from_millis(100000));
}