extern crate zodiac_parsing;
extern crate zodiac_resources;

use std::{
    fs,
    path::{ PathBuf }
};
use zodiac_parsing::lexing::Lexer;

#[derive(Debug)]
pub enum Error {
    FailedToLoadFiles,
    FaliedToMonitorFiles
}

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

pub fn initialise(zod_relative_folder_path: &str) -> Result<(), Error>  {
    let zod_folder_path = get_full_path(zod_relative_folder_path)?;
    let zod_app_file_path = zod_folder_path.join("app.zod");
    parse(zod_app_file_path);
    Ok(())
}

fn get_full_path(relative_path: &str) -> Result<PathBuf, Error> {
    zodiac_resources::file_system::from_relative_exe_path(relative_path)
        .map_err(|_|Error::FailedToLoadFiles)
}