use legion::*;
use zodiac_entities::*;
use crate::building::*;
use super::root_control::*;

pub fn create_control_implementation_builder<T:SourceReader>(control_name: String) -> Box<dyn EntityBuilder<T>> {
    Box::new(ControlImplementationBuilder::new(control_name))
}

struct ControlImplementationBuilder {
    control_name: String
}

impl ControlImplementationBuilder {
    fn new(control_name: String) -> Self {
        Self {
            control_name
        }
    }
}

impl<T:SourceReader> EntityBuilder<T> for ControlImplementationBuilder {
    fn get_entity<'a>(&self, build_resources_mut: &mut MutableBuildResources<'a>) -> Entity {
        build_resources_mut.world_builder.get_current_entity()
    }
    
    fn process_token(&mut self, _: &AbstractSyntaxToken) -> Result<(), BuildError> {
        Ok(())
    }

    fn build<'a>(&self, build_resources: &BuildResources<T>, build_resources_mut: &mut MutableBuildResources<'a>) -> Result<(), BuildError> {
        if let Some(source_entity) = build_resources_mut.import_control_lookup.get(&self.control_name) {
            let (control_impl_entity, source_implementation)  = build_resources_mut
                .world_builder
                .create_control_implementation(*source_entity);

            let mut builder = create_root_control_builder::<T>(control_impl_entity, source_implementation);
            builder.build_control(build_resources, build_resources_mut);
            return Ok(());
        }
        Err(BuildError::ControlDoesNotExist(self.control_name.clone()))
    }
}