use std::marker::PhantomData;
use log::info;
use shrev::*;
use legion::*;
use legion::systems::*;
use crate::*;
use crate::formatting::WorldSerializer;
use crate::source::*;

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

pub fn zodiac_source<TState: State, TRootFunc: FnMut() -> RootNode<TState> +'static>(state: TState, root_func: TRootFunc) -> ZodiacSourceBuilder<TState, TRootFunc>  {
    ZodiacSourceBuilder::new(state, root_func)
}

#[derive(Debug, Default)]
pub struct ZodiacSourceBuilder<TState: State, TRootFunc: FnMut() -> RootNode<TState> +'static> {
    root_func: TRootFunc,
    state: TState
}

impl<TState: State, TRootFunc: FnMut() -> RootNode<TState> +'static> ZodiacSourceBuilder<TState, TRootFunc> {
    pub fn new(state: TState, root_func: TRootFunc) -> Self  {
        Self {
            root_func,
            state
        }
    }    
}


impl<TState: State, TRootFunc: FnMut() -> RootNode<TState> + Copy + Clone + 'static> ApplicationBundleBuilder for ZodiacSourceBuilder<TState, TRootFunc> {
    fn description(&self) -> String {
        "zodiac build".to_string()
    }

    fn setup_build_systems(&self, builder: &mut Builder) {
        builder.add_thread_local(run_moxie_system::<TState>());
    }

    fn setup_layout_systems(&self, _: &mut Builder) {
    }

    fn setup_rendering_systems(&self, _: &mut Builder) {
    }

    fn setup_cleanup_systems(&self, builder: &mut Builder) {            
        builder.add_thread_local(remove_rebuild_system());
    }

    fn setup_final_functions(&self, _: &mut Builder) {
    }

    fn setup_resources(&self, resources: &mut Resources, _: &mut EventChannel<SystemEvent>) -> Result<(), ZodiacError>  {
        resources.insert(create_relationship_map());
        resources.insert(create_system_event_producer());     
        resources.insert(create_moxie_runner::<TState, TRootFunc>(self.root_func, self.state));
        resources.insert(create_state_repository::<TState>());
        resources.insert(create_entity_map()); 
        Ok(())
    }

    fn register_components_for_world_serializiation(&self, world_serializer: &mut WorldSerializer) {
        world_serializer.register_component::<ComponentId>(stringify!(ComponentId));
        world_serializer.register_component::<Dimensions>(stringify!(Dimensions));
        world_serializer.register_component::<RootWindowResized>(stringify!(RootWindowResized));
        world_serializer.register_component::<Removed>(stringify!(Removed));
        world_serializer.register_component::<Relationship>(stringify!(Relationship));
        world_serializer.register_component::<Root>(stringify!(Root));
        world_serializer.register_component::<Rebuild>(stringify!(Rebuild));
        world_serializer.register_component::<CurrentLayoutConstraints>(stringify!(CurrentLayoutConstraints));
        world_serializer.register_component::<Resized>(stringify!(Resized));
        world_serializer.register_component::<Mapped>(stringify!(Mapped));
        world_serializer.register_component::<LayoutType>(stringify!(LayoutType));
        world_serializer.register_component::<LayoutContent>(stringify!(LayoutContent));
        world_serializer.register_component::<LayoutRequest>(stringify!(LayoutRequest));
        world_serializer.register_component::<LayoutChange>(stringify!(LayoutChange));
        world_serializer.register_component::<Renderable>(stringify!(Renderable));
        world_serializer.register_component::<Left>(stringify!(Left));
        world_serializer.register_component::<Top>(stringify!(Top));
        world_serializer.register_component::<OffsetsMapped>(stringify!(OffsetsMapped));
        world_serializer.register_component::<Width>(stringify!(Width));
        world_serializer.register_component::<MinimumWidth>(stringify!(MinimumWidth));
        world_serializer.register_component::<Height>(stringify!(Height));
        world_serializer.register_component::<MinimumHeight>(stringify!(MinimumHeight));
    }
}

fn standard_layout() -> LayoutBundleBuilder {
    LayoutBundleBuilder::default()
}

#[derive(Default, Debug, Copy, Clone)]
struct LayoutBundleBuilder {
}

impl ApplicationBundleBuilder for LayoutBundleBuilder {
    fn description(&self) -> String {
        "standard layout".to_string()
    }
    
    fn setup_build_systems(&self, _: &mut Builder) {
    }

    fn setup_layout_systems(&self, builder: &mut Builder) {
        builder
            .add_system(resize_screen_system())
            .add_system(resize_after_rebuild_system())
            .flush()
            .add_system(remove_from_left_offset_map_system())
            .add_system(build_left_offset_map_system())
            .add_system(remove_from_top_offset_map_system())
            .add_system(build_top_offset_map_system())
            .add_system(remove_from_minimum_width_map_system())
            .add_system(remove_from_width_map_system())
            .add_system(build_width_map_system())
            .add_system(remove_from_minimum_height_map_system())
            .add_system(remove_from_height_map_system())
            .add_system(build_height_map_system())
            .add_system(remove_from_layout_type_map_system())
            .add_system(build_layout_type_map_system())
            .flush()
            .add_system(measure_fixed_width_constraints_system())
            .add_system(measure_fixed_height_constraints_system())
            .flush()
            .add_system(resize_system());
    }

    fn setup_rendering_systems(&self, _: &mut Builder) {
    }

    fn setup_cleanup_systems(&self, builder: &mut Builder) { 
        builder
            .add_thread_local(remove_layout_change_system())
            .add_thread_local(remove_resized_system());

    }

    fn setup_final_functions(&self, _: &mut Builder) {
    }
    
    fn setup_resources(&self, resources: &mut Resources, event_channel: &mut EventChannel<SystemEvent>) -> Result<(), ZodiacError>  {
        resources.insert(create_layout_event_reader_registry(event_channel));
        resources.insert(create_layout_type_map());
        resources.insert(create_left_offset_map());
        resources.insert(create_top_offset_map());
        resources.insert(create_width_map());
        resources.insert(create_height_map());
        resources.insert(create_minimum_width_map());
        resources.insert(create_minimum_height_map());
        
        Ok(())
    }    
    
    fn register_components_for_world_serializiation(&self, _: &mut WorldSerializer) {
    }
}

pub struct Application<TState: State> {
    resources: Resources,
    schedule_builder: Builder,
    builders: Vec::<Box::<dyn ApplicationBundleBuilder>>,
    _marker: PhantomData<TState>
}

impl<TState: State> Application<TState> {
    pub fn new<TRootFunc: FnMut() -> RootNode<TState> + Copy + 'static>(state: TState, root_func: TRootFunc) -> Self {
        let resources = Resources::default();
        let schedule_builder = Schedule::builder();
            
        Self {
            resources,
            schedule_builder,
            builders: vec!(Box::new(zodiac_source(state, root_func)), Box::new(standard_layout())),
            _marker: PhantomData::<TState>::default()
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

    pub fn build(mut self) -> Result<ApplicationRunner<TState>, ZodiacError> {        
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

        Ok(ApplicationRunner::<TState>::new(self.schedule_builder.build(), self.resources))
    }
}

pub struct ApplicationRunner<TState: State> {
    world: World, 
    resources: Resources,
    schedule: Schedule,
    main_reader_id: ReaderId<SystemEvent>,
    _marker: PhantomData<TState>
}

impl<TState: State> ApplicationRunner<TState> {
    fn new(schedule: Schedule, resources: Resources) -> Self {
        let main_reader_id = resources
            .get_mut::<EventChannel<SystemEvent>>()
            .unwrap()
            .register_reader();

        Self {
            world: World::default(),
            schedule,
            resources,
            main_reader_id,
            _marker: PhantomData::<TState>::default()
        }
    }

    pub fn run_until_closed(&mut self) {           
        loop {
            &mut self.execute();
            
            let event_channel = self.resources.get::<EventChannel<SystemEvent>>().unwrap();
            for event in event_channel.read(&mut self.main_reader_id) {
                match event {
                    SystemEvent::Window(SystemWindowEventType::CloseRequested) => return,
                    _ => {}
                }
            }
        }
    }

    pub fn run_once(&mut self) -> TState {
        &mut self.execute();
        self.get_state()
    }

    pub fn get_state(&mut self) -> TState {
        let repository = self.resources_mut().get::<StateRepository<TState>>().unwrap();
        repository.get()
    }

    pub fn resources_mut(&mut self) -> &mut Resources {
        &mut self.resources
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    fn execute(&mut self) {
        &mut self.schedule.execute(&mut self.world, &mut self.resources);
    }
}