extern crate zodiac_parsing;
extern crate zodiac_resources;

use legion::*;

use zodiac_parsing::tokenization::source::SourceTokenizer;
use zodiac_parsing::tokenization::abstract_syntax::{AbstractSyntaxTokenizer, AbstractSyntaxTokenError};
use zodiac_resources::file_system;

use crate::abstract_syntax::world_building::WorldBuilder;
use crate::systems::*;
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


pub fn initialise(zod_relative_folder_path: &str) -> Result<(), ZodiacError>  {
    let mut world = World::default();
    let mut resources = Resources::default();

    setup_world(&mut world, &mut resources);
    parse_to_world(&mut world, load_app_zod_file_from_relative_path(zod_relative_folder_path)?.as_str())
}

fn setup_world(world: &mut World, resources: &mut Resources) {
    let mut schedule = Schedule::builder()
        .add_system(render_rectangles_system())
        .build();

    schedule.execute(world, resources);
}

fn parse_to_world(world: &mut World, text: &str) -> Result<(), ZodiacError> {
    let mut tokens = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string(text));
    tokens.build_world(world)?;
    world.to_pretty();
    Ok(())
}

fn load_app_zod_file_from_relative_path(zod_relative_folder_path: &str) -> Result<String, ZodiacError> {
    Ok(file_system::load_app_zod_file_from_relative_path(zod_relative_folder_path)?)
}