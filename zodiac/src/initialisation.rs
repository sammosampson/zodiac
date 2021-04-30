use log::info;
use shrev::*;
use legion::*;
use legion::systems::*;
use zodiac_entities::*;
use zodiac_layout::*;
use zodiac_source_filesystem::*;
use zodiac_rendering::*;

use crate::systems::error_reporting::*;
use crate::systems::world_vision::*;

pub fn standard_builders(relative_zod_folder_path: &'static str) -> Vec::<Box::<dyn ApplicationBundleBuilder>> {
    vec!(
        Box::new(standard_source_file_building(relative_zod_folder_path)),
        Box::new(standard_source_building()),
        Box::new(standard_layout()),
        Box::new(standard_rendering()),
        Box::new(renderer()),
    )
}

#[cfg(feature = "glium_rendering")]
use zodiac_rendering_glium::*;


#[cfg(feature = "glium_rendering")]
pub fn standard_rendering() -> RendereringBuilder<GliumRenderer, GliumRenderQueue> {
    standard_glium_rendering()
}

#[cfg(feature = "glium_rendering")]
pub fn renderer() -> GliumRendererBuilder {
    glium_renderer()
}

#[cfg(feature = "pathfinder_rendering")]
use zodiac_rendering_pathfinder::*;

#[cfg(feature = "pathfinder_rendering")]
pub fn standard_rendering() -> RendereringBuilder<PathFinderRenderer, PathFinderRenderQueue> {
    standard_pathfinder_rendering()
}

#[cfg(feature = "pathfinder_rendering")]
pub fn renderer() -> PathFinderRendererBuilder {
    pathfinder_renderer()
}

pub fn world_vision() -> WorldVisionBundleBuilder {
    WorldVisionBundleBuilder::default()
}

#[derive(Debug, Default)]
pub struct WorldVisionBundleBuilder {
}

impl ApplicationBundleBuilder for WorldVisionBundleBuilder {
    fn description(&self) -> String {
        "world visualising build".to_string()
    }

    fn setup_build_systems(&self, _: &mut Builder) {
    }

    fn setup_layout_systems(&self, _: &mut Builder) {
    }

    fn setup_rendering_systems(&self, _: &mut Builder) {
    }

    fn setup_cleanup_systems(&self, _: &mut Builder) {            
    }

    fn setup_final_functions(&self, builder: &mut Builder) {
        builder.add_thread_local_fn(log_world_view);
    }

    fn setup_resources(&self, _: &mut Resources, _: &mut EventChannel<SystemEvent>) -> Result<(), ZodiacError>  {
        Ok(())
    }
}

pub struct Application {
    resources: Resources,
    schedule_builder: Builder,
    builders: Vec::<Box::<dyn ApplicationBundleBuilder>>
}

impl Application {
    pub fn new() -> Self {
        let resources = Resources::default();
        let schedule_builder = Schedule::builder();
            
        Self {
            resources,
            schedule_builder,
            builders: vec!()
        }
    }

    pub fn use_logging(self) -> Self {
        pretty_env_logger::init();
        self
    }

    pub fn with_builder<T>(mut self, builder: T) -> Self
        where T: ApplicationBundleBuilder + 'static {
        self.builders.push(Box::new(builder));
        self
    }

    pub fn with_builders(mut self, builders: &mut Vec::<Box::<dyn ApplicationBundleBuilder>>) -> Self
    {
        self.builders.append(builders);
        self
    }

    pub fn build(mut self) -> Result<ApplicationRunner, ZodiacError> {
        let mut event_channel = create_system_event_channel();
        
        for builder in &self.builders {
            info!("setup_build_systems: {:?}", builder.description());
            builder.setup_build_systems(&mut self.schedule_builder);
            self.schedule_builder.flush();    
        }

        self.schedule_builder.add_system(remove_from_relationship_map_system());
        self.schedule_builder.add_system(build_relationship_map_system());
        self.schedule_builder.flush();  

        for builder in &self.builders {
            info!("setup_layout_systems: {:?}", builder.description());
            builder.setup_layout_systems(&mut self.schedule_builder);
            self.schedule_builder.flush();    
        }

        self.schedule_builder.add_system(remove_entity_system());
        self.schedule_builder.flush();
        self.schedule_builder.add_system(mark_as_mapped_system());
        self.schedule_builder.flush();  

        for builder in &self.builders {
            info!("setup_rendering_systems: {:?}", builder.description());
            builder.setup_rendering_systems(&mut self.schedule_builder);
            self.schedule_builder.flush();    
        }

        self.schedule_builder.add_thread_local(report_build_error_system());
        self.schedule_builder.flush(); 

        for builder in &self.builders {
            info!("setup_cleanup_systems: {:?}", builder.description());
            builder.setup_cleanup_systems(&mut self.schedule_builder);
            self.schedule_builder.flush();    
        }
        
        for builder in &self.builders {
            info!("setup_final_functions: {:?}", builder.description());
            builder.setup_final_functions(&mut self.schedule_builder);
        }

        for builder in &self.builders {
            info!("setup_resources: {:?}", builder.description());
            builder.setup_resources(&mut self.resources, &mut event_channel)?;
        }

        &mut self.resources.insert(event_channel);
        &mut self.resources.insert(create_relationship_map());
        &mut self.resources.insert(create_system_event_producer());        

        Ok(ApplicationRunner::new(self.schedule_builder.build(), self.resources))
    }
}

pub struct ApplicationRunner {
    world: World, 
    resources: Resources,
    schedule: Schedule
}

impl ApplicationRunner {
    fn new(schedule: Schedule, resources: Resources) -> Self {
        Self {
            world: World::default(),
            schedule,
            resources,
        }
    }

    pub fn run_until_closed(&mut self) {
        let mut main_reader_id = &mut self.resources
            .get_mut::<EventChannel<SystemEvent>>()
            .unwrap()
            .register_reader();
            
        loop {
            &mut self.schedule.execute(&mut self.world, &mut self.resources);
            let event_channel = self.resources.get::<EventChannel<SystemEvent>>().unwrap();
            for event in event_channel.read(&mut main_reader_id) {
                match event {
                    SystemEvent::Window(SystemWindowEventType::CloseRequested) => return,
                    _ => {}
                }
            }
        }
    }

    pub fn resources_mut(&mut self) -> &mut Resources {
        &mut self.resources
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn run_once(&mut self) {
        &mut self.schedule.execute(&mut self.world, &mut self.resources);
    }
}