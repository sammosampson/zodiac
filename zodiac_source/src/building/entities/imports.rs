use log::{debug};
use legion::*;
use zodiac_entities::*;
use crate::building::*;
use super::*;

pub fn create_import_builder<T:SourceReader>(root_entity: Entity, source_implementation: SourceImplementation) -> Box<dyn EntityBuilder<T>> {
    Box::new(ImportBuilder::new(root_entity, source_implementation))
}

struct ImportBuilder {
    root_entity: Entity,
    source_implementation: SourceImplementation,
    tokens: AbstractSyntaxPropertyTokenRequirements
}

impl ImportBuilder {
    fn new(root_entity: Entity, source_implementation: SourceImplementation) -> Self {
        let mut tokens = AbstractSyntaxPropertyTokenRequirements::new();

        tokens
            .with_required_token(AbstractSyntaxNodeType::Name)
            .with_required_token(AbstractSyntaxNodeType::Path);
            
        Self {
            root_entity,
            source_implementation,
            tokens
        }
    }
}

impl<T:SourceReader> EntityBuilder<T> for ImportBuilder {
    fn get_entity<'a>(&self, _: &mut MutableBuildResources<'a>) -> Entity {
        self.root_entity
    }

    fn process_token(&mut self, token: &AbstractSyntaxToken)-> Result<(), BuildError> {
        self.tokens.process_token(token)?;
        Ok(())
    }

    fn build(&self, build_resources: &BuildResources<T>, build_resources_mut: &mut MutableBuildResources) -> Result<(), BuildError> {
        debug!("building import {:?}", self.root_entity);
        
        let mut import_relative_path = String::default();
        let mut import_control_name = String::default();

        for token in self.tokens.get_processed_tokens()? {
            match token {
                AbstractSyntaxToken::Path(path) => import_relative_path.push_str(&path),
                AbstractSyntaxToken::Name(name) => import_control_name.push_str(&name),
                value => return Err(BuildError::UnexpectedToken(AbstractSyntaxNodeType::from(&value)))
            }
        }

        let root_location = build_resources.source_location_lookup
            .get(&self.source_implementation.source_file_entity)
            .unwrap();
        
        if let Ok(full_import_control_location) = build_resources.source_reader.get_relative_source_location(root_location,  &import_relative_path) {
            if let Some(import_entity)  = build_resources.source_entity_lookup.get(&full_import_control_location) {
                build_resources_mut.import_control_lookup.insert(import_control_name, *import_entity);
            }
            else {
                return Err(BuildError::ControlSourceDoesNotExist(String::from(full_import_control_location)));
            }
        } else {
            return Err(BuildError::SourceLocationDoesNotExist(import_relative_path));
        }

        Ok(())
    }
}