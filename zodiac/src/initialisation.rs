extern crate zodiac_parsing;
extern crate zodiac_resources;

use zodiac_parsing::lexing::Lexer;
use zodiac_parsing::formatting::Pretty;
use zodiac_resources::file_system;

#[derive(Debug)]
pub enum Error {
    FailedToLoadZodFile(file_system::Error),
}

fn parse_to_renderer(text: &str) -> Result<(), Error> {
    let mut lexer = Lexer::parse(text);
    lexer.to_pretty();
    Ok(())
}

pub fn initialise(zod_relative_folder_path: &str) -> Result<(), Error>  {
    parse_to_renderer(load_app_zod_file_from_relative_path(zod_relative_folder_path)?.as_str())
}

fn load_app_zod_file_from_relative_path(zod_relative_folder_path: &str) -> Result<String, Error> {
    file_system::load_app_zod_file_from_relative_path(zod_relative_folder_path).map_err(|error|Error::FailedToLoadZodFile(error))
}