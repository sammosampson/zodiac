use std::marker::PhantomData;

use shrev::*;
use legion::*;
use legion::systems::*;
use zodiac_entities::*;
use crate::*;

#[derive(Default, Debug)]
pub struct RendereringBuilder<TRenderer>
where 
    TRenderer: Renderer + 'static {
    renderer: PhantomData<TRenderer>
}

impl<TRenderer> RendereringBuilder<TRenderer>
where 
    TRenderer: Renderer + 'static {
    pub fn new() -> Self {
        Self {
            renderer: PhantomData::<TRenderer>::default()
        }
    }
}

impl<TRenderer> ApplicationBundleBuilder 
for RendereringBuilder<TRenderer>
where 
    TRenderer: Renderer + 'static{
    fn description(&self) -> String {
        "standard rendering".to_string()
    }
    fn setup_build_systems(&self, builder: &mut Builder) {
        builder
            .add_thread_local(initial_window_size_notification_system::<TRenderer>());
    }

    fn setup_layout_systems(&self, _: &mut Builder) {
    }

    fn setup_rendering_systems(&self, _: &mut Builder) {
    }

    fn setup_cleanup_systems(&self, _: &mut Builder) {
    }

    fn setup_final_functions(&self, _: &mut Builder) {
    }

    fn setup_resources(&self, _: &mut Resources, _: &mut EventChannel<SystemEvent>) -> Result<(), ZodiacError>  {
        Ok(())
    }    
    
    fn register_components_for_world_serializiation(&self, _: &mut WorldSerializer) {
    }
}