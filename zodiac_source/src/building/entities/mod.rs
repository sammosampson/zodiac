mod root_control;
mod standard;
mod imports;
mod control_implementation;

pub use root_control::*;

use legion::*;
use zodiac_entities::*;
use crate::tokenization::abstract_syntax::*;
use crate::source::*;
use crate::building::*;

pub trait EntityBuilder<T:SourceReader> {
    fn get_entity<'a>(&self, build_resources_mut: &mut MutableBuildResources<'a>) -> Entity;
    fn process_token(&mut self, token: &AbstractSyntaxToken) -> Result<(), BuildError>; 
    fn build<'a>(&self, build_resources: &BuildResources<T>, build_resources_mut: &mut MutableBuildResources<'a>) -> Result<(), BuildError>;  
}