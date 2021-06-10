use log::info;
use shrev::*;
use legion::*;
use legion::systems::*;
use crate::*;
use crate::formatting::WorldSerializer;

pub trait ApplicationBundleBuilder {
    fn description(&self) -> String;
    fn setup_build_systems(&self, builder: &mut Builder);
    fn setup_layout_systems(&self, builder: &mut Builder);
    fn setup_rendering_systems(&self, builder: &mut Builder);
    fn setup_cleanup_systems(&self, builder: &mut Builder);
    fn setup_final_functions(&self, builder: &mut Builder);
    fn setup_resources(&self, resources: &mut Resources, event_channel: &mut EventChannel<SystemEvent>) -> Result<(), ZodiacError>;
    fn register_components_for_world_serializiation(&self, serializer: &mut WorldSerializer);
}


pub fn world_logging() -> WorldLoggingBuilder {
    WorldLoggingBuilder::default()
}

#[derive(Debug, Default)]
pub struct WorldLoggingBuilder {
}

impl ApplicationBundleBuilder for WorldLoggingBuilder {
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

    fn register_components_for_world_serializiation(&self, _: &mut WorldSerializer) {
    }
}

pub fn entities() -> EntitiesBuilder {
    EntitiesBuilder::default()
}

#[derive(Debug, Default)]
pub struct EntitiesBuilder {
}


impl ApplicationBundleBuilder for EntitiesBuilder {
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

    fn setup_resources(&self, resources: &mut Resources, _: &mut EventChannel<SystemEvent>) -> Result<(), ZodiacError>  {
        resources.insert(create_relationship_map());
        resources.insert(create_system_event_producer());        
        resources.insert(create_system_event_producer());     

        Ok(())
    }

    fn register_components_for_world_serializiation(&self, world_serializer: &mut WorldSerializer) {
        world_serializer.register_component::<Dimensions>(stringify!(Dimensions));
        world_serializer.register_component::<RootWindowResized>(stringify!(RootWindowResized));
        world_serializer.register_component::<Removed>(stringify!(Removed));
        world_serializer.register_component::<Relationship>(stringify!(Relationship));
        world_serializer.register_component::<Root>(stringify!(Root));
        world_serializer.register_component::<Control>(stringify!(Control));
        world_serializer.register_component::<Rebuild>(stringify!(Rebuild));
        world_serializer.register_component::<CurrentLayoutConstraints>(stringify!(CurrentLayoutConstraints));
        world_serializer.register_component::<Resized>(stringify!(Resized));
        world_serializer.register_component::<Mapped>(stringify!(Mapped));
        world_serializer.register_component::<LayoutType>(stringify!(LayoutType));
        world_serializer.register_component::<LayoutContent>(stringify!(LayoutContent));
        world_serializer.register_component::<LayoutRequest>(stringify!(LayoutRequest));
        world_serializer.register_component::<LayoutChange>(stringify!(LayoutChange));
        world_serializer.register_component::<Renderable>(stringify!(Renderable));
        world_serializer.register_component::<Content>(stringify!(Content));
        world_serializer.register_component::<Left>(stringify!(Left));
        world_serializer.register_component::<Top>(stringify!(Top));
        world_serializer.register_component::<OffsetsMapped>(stringify!(OffsetsMapped));
        world_serializer.register_component::<Width>(stringify!(Width));
        world_serializer.register_component::<MinimumWidth>(stringify!(MinimumWidth));
        world_serializer.register_component::<Height>(stringify!(Height));
        world_serializer.register_component::<MinimumHeight>(stringify!(MinimumHeight));
        world_serializer.register_component::<Radius>(stringify!(Radius));
        world_serializer.register_component::<Colour>(stringify!(Colour));
        world_serializer.register_component::<StrokeWidth>(stringify!(StrokeWidth));
        world_serializer.register_component::<StrokeColour>(stringify!(StrokeColour));
        world_serializer.register_component::<CornerRadii>(stringify!(CornerRadii));  
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
            builders: vec!(Box::new(entities()))
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
        
        let mut event_channel = create_system_event_channel();

        for builder in &self.builders {
            info!("setup_resources: {:?}", builder.description());
            builder.setup_resources(&mut self.resources, &mut event_channel)?;
        }
             
        let mut world_serializer = create_world_serializer();

        for builder in &self.builders {
            info!("register_components_for_world_serializiation: {:?}", builder.description());
            builder.register_components_for_world_serializiation(&mut world_serializer)
        }
         
        &mut self.resources.insert(event_channel);
        &mut self.resources.insert(world_serializer);

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