extern crate zodiac_parsing;
extern crate zodiac_resources;

use legion::*;

use zodiac_parsing::tokenization::source::SourceTokenizer;
use zodiac_parsing::tokenization::abstract_syntax::{AbstractSyntaxTokenizer, AbstractSyntaxTokenError};
use zodiac_resources::file_system;
use crate::abstract_syntax::world_building::WorldBuilder;
use crate::formatting::Pretty;

#[derive(Debug)]
pub enum ZodiacError {
    FailedToLoadZodFile(file_system::Error),
    FailedParse(AbstractSyntaxTokenError)
}

impl From<AbstractSyntaxTokenError> for ZodiacError {
    fn from(error: AbstractSyntaxTokenError) -> Self {
        ZodiacError::FailedParse(error)
    }
}
impl From<file_system::Error> for ZodiacError {
    fn from(error: file_system::Error) -> Self {
        ZodiacError::FailedToLoadZodFile(error)
    }
}

fn parse_to_renderer(text: &str) -> Result<(), ZodiacError> {
    let mut tokenizer = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string(text));
    let mut world = World::default();
    tokenizer.build_world(&mut world)?;
    world.to_pretty();
    Ok(())
}

pub fn initialise(zod_relative_folder_path: &str) -> Result<(), ZodiacError>  {
    parse_to_renderer(load_app_zod_file_from_relative_path(zod_relative_folder_path)?.as_str())
}

fn load_app_zod_file_from_relative_path(zod_relative_folder_path: &str) -> Result<String, ZodiacError> {
    file_system::load_app_zod_file_from_relative_path(zod_relative_folder_path)
        .map_err(|error|ZodiacError::from(error))
}