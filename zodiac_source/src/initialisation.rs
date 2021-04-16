use shrev::*;
use std::marker::PhantomData;
use legion::*;
use legion::systems::*;
use zodiac_entities::*;
use crate::*;

#[derive(Debug)]
pub struct SourceBuildBundleBuilder<TSourceReader> where TSourceReader: SourceReader + Send +'static {
    source_reader: PhantomData<TSourceReader>
}

impl<TSourceReader> SourceBuildBundleBuilder<TSourceReader> where TSourceReader: SourceReader + Send +'static {
    pub fn new() -> Self {
        Self {
            source_reader: PhantomData::<TSourceReader>::default()
        }
    }
}

impl<TSourceReader> ApplicationBundleBuilder for SourceBuildBundleBuilder<TSourceReader> 
where TSourceReader: SourceReader + Send + 'static {
    fn description(&self) -> String {
        "standard source build".to_string()
    }

    fn setup_build_systems(&self, builder: &mut Builder) {
        builder
            .add_system(source_token_removal_system())
            .add_system(source_parse_system::<TSourceReader>())
            .flush()
            .add_system(apply_initially_read_root_source_to_world_system())
            .add_system(apply_created_source_to_world_system())
            .add_system(apply_removed_source_to_world_system())
            .add_system(apply_changed_source_to_world_system())
            .flush()
            .add_system(world_build_system::<TSourceReader>())
            .flush()
            .add_system(error_control_for_renderable_system())
            .add_system(error_control_for_non_renderable_system());
    }

    fn setup_layout_systems(&self, _: &mut Builder) {
    }

    fn setup_rendering_systems(&self, _: &mut Builder) {
    }

    fn setup_cleanup_systems(&self, _: &mut Builder) {
    }

    fn setup_resources(&self, resources: &mut Resources, _: &mut EventChannel<SystemEvent>) -> Result<(), ZodiacError>  {
        resources.insert(create_source_entity_lookup());
        resources.insert(create_source_tokens_lookup());
        resources.insert(create_source_location_lookup());
        Ok(())
    }
}