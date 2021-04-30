use std::marker::PhantomData;

use shrev::*;
use legion::*;
use legion::systems::*;
use zodiac_entities::*;
use crate::*;

#[derive(Default, Debug)]
pub struct RendereringBuilder<TRenderer, TRenderQueue>
where 
    TRenderer: Renderer + 'static,
    TRenderQueue: RenderQueue + 'static {
    renderer: PhantomData<TRenderer>,
    render_queue: PhantomData<TRenderQueue>,
}

impl<TRenderer, TRenderQueue> RendereringBuilder<TRenderer, TRenderQueue>
where 
    TRenderer: Renderer + 'static,
    TRenderQueue: RenderQueue + 'static {
    pub fn new() -> Self {
        Self {
            renderer: PhantomData::<TRenderer>::default(),
            render_queue: PhantomData::<TRenderQueue>::default()
        }
    }
}

impl<TRenderer, TRenderQueue> ApplicationBundleBuilder 
for RendereringBuilder<TRenderer, TRenderQueue>
where 
    TRenderer: Renderer + 'static,
    TRenderQueue: RenderQueue + 'static {
    fn description(&self) -> String {
        "standard rendering".to_string()
    }
    fn setup_build_systems(&self, builder: &mut Builder) {
        builder
            .add_thread_local(initial_window_size_notification_system::<TRenderer>());
    }

    fn setup_layout_systems(&self, _: &mut Builder) {
    }

    fn setup_rendering_systems(&self, builder: &mut Builder) {
        builder
            .add_thread_local(queue_render_primitives_system::<TRenderQueue>());
    }

    fn setup_cleanup_systems(&self, _: &mut Builder) {
    }

    fn setup_final_functions(&self, _: &mut Builder) {
    }

    fn setup_resources(&self, _: &mut Resources, _: &mut EventChannel<SystemEvent>) -> Result<(), ZodiacError>  {
        Ok(())
    }
}