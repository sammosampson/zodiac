mod entity;
mod world;

pub use entity::*;
pub use world::*;

use legion::*;
use zodiac_entities::*;
use crate::tokenization::abstract_syntax::*;

struct AbstractSyntaxPropertyTokens { 
    required_properties: Vec<AbstractSyntaxNodeType>,
    optional_properties: Vec<AbstractSyntaxNodeType>,
    processed_tokens: Vec<AbstractSyntaxToken>,
}

impl AbstractSyntaxPropertyTokens {
    fn new() -> Self {
        Self {
            required_properties: vec!(),
            optional_properties: vec!(),
            processed_tokens: vec!(),
        }
    }
    fn with_required_property(&mut self, property: AbstractSyntaxNodeType) -> &mut Self {
        self.required_properties.push(property);
        self
    }

    fn with_optional_property(&mut self, property: AbstractSyntaxNodeType) -> &mut Self {
        self.optional_properties.push(property);
        self
    }

    fn process_property_token(&mut self, entity: Entity, token: &AbstractSyntaxToken) -> Result<(), BuildError> {
        let node_type = AbstractSyntaxNodeType::from(token);

        if self.required_properties.contains(&node_type) {
            self.required_properties.retain(|property| property != &node_type)
        } else if self.optional_properties.contains(&node_type){
            self.optional_properties.retain(|property| property != &node_type)
        } else {
            return Err(BuildError::new(entity, BuildErrorReason::UnexpectedToken(node_type)));
        }

        self.processed_tokens.push(token.to_owned());

        Ok(())
    }

    fn get_processed_tokens(&self, entity: Entity) -> Result<Vec<AbstractSyntaxToken>, BuildError> {
        if self.required_properties.len() > 0 {
            return Err(BuildError::new(entity, BuildErrorReason::MissingRequiredTokens(self.required_properties.to_owned())));
        }
        Ok(self.processed_tokens.to_owned())
    }
}
