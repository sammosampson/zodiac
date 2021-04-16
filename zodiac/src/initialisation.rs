use log::info;
use shrev::*;
use legion::*;
use legion::systems::*;
use zodiac_entities::*;

use crate::systems::error_reporting::*;

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

    pub fn with_builder<T>(mut self, builder: T) -> Self
        where T: ApplicationBundleBuilder + 'static {
        self.builders.push(Box::new(builder));
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