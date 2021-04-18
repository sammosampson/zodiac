use legion::*;
use zodiac_entities::*;
use crate::tokenization::abstract_syntax::*;
use crate::source::*;
use crate::building::*;
use entities::{imports::*, standard::*, control_implementation::*};

pub fn create_root_control_builder<T:SourceReader>(root_source_implementation_entity: Entity, source_implementation: SourceImplementation) -> RootControlBuilder<T> {
    RootControlBuilder::<T>::new(root_source_implementation_entity, source_implementation)
}
pub struct RootControlBuilder<T:SourceReader> {
    root_source_implementation_entity: Entity,
    source_implementation: SourceImplementation,
    current_entity_builders: Vec::<Box<dyn EntityBuilder<T>>>
}

impl<T:SourceReader> RootControlBuilder<T> {
    fn new(root_source_implementation_entity: Entity, source_implementation: SourceImplementation) -> Self {
        Self {
            root_source_implementation_entity,
            source_implementation,
            current_entity_builders: vec!()
        }
    }

    pub fn get_entity<'a>(&mut self, build_resources_mut: &mut MutableBuildResources<'a>) -> Entity {
        if let Some(current_entity_builder) = self.current_entity_builders.pop() {
            let entity = current_entity_builder.get_entity(build_resources_mut);
            self.current_entity_builders.push(current_entity_builder);
            return entity;
        } 
        self.root_source_implementation_entity
    }

    fn handle_error<'a, TResult>(&mut self, build_resources_mut: &mut MutableBuildResources<'a>, result: Result<TResult, BuildError>) -> Option<TResult> {
        match result {
            Ok(return_value) => Some(return_value),
            Err(build_error) => {
                self.set_error(build_resources_mut, build_error);
                return None;
            }
        }
    }

    fn set_error<'a>(&mut self, build_resources_mut: &mut MutableBuildResources<'a>, error: BuildError) {
        let error_entity = self.get_entity(build_resources_mut);
        build_resources_mut.world_builder.add_error_component_to_entity(error_entity, error);
    }

    pub fn build_control<'a>(&mut self, build_resources: &BuildResources<T>, build_resources_mut: &mut MutableBuildResources<'a>) {
        let tokens_result = build_resources
            .source_tokens_lookup
            .get(&self.source_implementation.source_file_entity)
            .ok_or(BuildError::ControlSourceFileDoesNotExist);
            
        if let Some(tokens) = self.handle_error(build_resources_mut, tokens_result) {
            for token_result in tokens {
                match token_result {
                    Ok(token) => {
                        match token {
                            AbstractSyntaxToken::Root => self.current_entity_builders.push(create_root_builder(build_resources_mut)),
                            AbstractSyntaxToken::Import => self.current_entity_builders.push(create_import_builder(self.root_source_implementation_entity, self.source_implementation)),
                            AbstractSyntaxToken::Circle => self.current_entity_builders.push(create_circle_builder(build_resources_mut)),
                            AbstractSyntaxToken::Rectangle => self.current_entity_builders.push(create_rectangle_builder(build_resources_mut)),
                            AbstractSyntaxToken::Text => self.current_entity_builders.push(create_text_builder(build_resources_mut)),
                            AbstractSyntaxToken::Control => self.current_entity_builders.push(create_control_builder(build_resources_mut)),
                            AbstractSyntaxToken::ControlImplementation(control_name) => self.current_entity_builders.push(create_control_implementation_builder(control_name.to_owned())),
                            AbstractSyntaxToken::CanvasLayoutContent => self.current_entity_builders.push(create_canvas_builder(build_resources_mut)),
                            AbstractSyntaxToken::HorizontalLayoutContent => self.current_entity_builders.push(create_horizontal_layout_builder(build_resources_mut)),
                            AbstractSyntaxToken::VerticalLayoutContent => self.current_entity_builders.push(create_vertical_layout_builder(build_resources_mut)),
                            AbstractSyntaxToken::CompleteControl => {
                                if let Some(current_entity_builder) = self.current_entity_builders.pop() {
                                    let build_result = current_entity_builder.build(build_resources, build_resources_mut);
                                    self.handle_error(build_resources_mut, build_result);
                                }
                            },
                            property_token => {
                                if let Some(mut current_entity_builder) = self.current_entity_builders.pop() {
                                    self.handle_error(build_resources_mut, current_entity_builder.process_token(&property_token));
                                    self.current_entity_builders.push(current_entity_builder);
                                }
                            }
                        }
                    },
                    Err(error) => self.set_error(build_resources_mut, BuildError::from(*error))
                }
            }
        }
    }
}