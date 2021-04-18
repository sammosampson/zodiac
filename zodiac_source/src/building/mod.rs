mod world;
mod entities;

pub use world::*;
pub use entities::*;

use std::{collections::HashMap};
use legion::*;
use zodiac_entities::*;
use crate::tokenization::abstract_syntax::*;
use crate::source::*;

pub fn create_import_control_lookup() -> ImportControlEntityLookup {
    ImportControlEntityLookup::new()
}

pub type ImportControlEntityLookup = HashMap<String, Entity>;

pub struct MutableBuildResources<'a> {
    pub world_builder: &'a mut WorldBuilder<'a>,
    pub import_control_lookup: ImportControlEntityLookup
}

impl <'a> MutableBuildResources<'a> {
    pub fn new(world_builder: &'a mut WorldBuilder<'a>) -> Self {
        Self {
            world_builder,
            import_control_lookup: create_import_control_lookup()
        }
    }
}

pub struct BuildResources<'a, T:SourceReader> {
    pub source_entity_lookup: &'a SourceEntityLookup,
    pub source_location_lookup: &'a SourceLocationLookup,
    pub source_tokens_lookup: &'a SourceTokensLookup,
    pub source_reader: &'a T
}

struct AbstractSyntaxPropertyTokenRequirements { 
    required_tokens: Vec<AbstractSyntaxNodeType>,
    optional_tokens: Vec<AbstractSyntaxNodeType>,
    processed_tokens: Vec<AbstractSyntaxToken>,
}

impl AbstractSyntaxPropertyTokenRequirements {
    fn new() -> Self {
        Self {
            required_tokens: vec!(),
            optional_tokens: vec!(),
            processed_tokens: vec!(),
        }
    }
    fn with_required_token(&mut self, property: AbstractSyntaxNodeType) -> &mut Self {
        self.required_tokens.push(property);
        self
    }

    fn with_optional_token(&mut self, property: AbstractSyntaxNodeType) -> &mut Self {
        self.optional_tokens.push(property);
        self
    }

    fn process_token(&mut self, token: &AbstractSyntaxToken) -> Result<(), BuildError> {
        let node_type = AbstractSyntaxNodeType::from(token);

        if self.required_tokens.contains(&node_type) {
            self.required_tokens.retain(|property| property != &node_type)
        } else if self.optional_tokens.contains(&node_type){
            self.optional_tokens.retain(|property| property != &node_type)
        } else {
            return Err(BuildError::UnexpectedToken(node_type));
        }

        self.processed_tokens.push(token.to_owned());

        Ok(())
    }

    fn get_processed_tokens(&self) -> Result<Vec<AbstractSyntaxToken>, BuildError> {
        if self.required_tokens.len() > 0 {
            return Err(BuildError::MissingRequiredTokens(self.required_tokens.to_owned()));
        }
        Ok(self.processed_tokens.to_owned())
    }
}
