use std::time::*;
use legion::*;
use glium::*;
use glium::glutin::event_loop::*;
use glutin::event::*;
use zodiac_resources::file_system;
use zodiac_entities::components::*;
use zodiac_parsing::tokenization::source::*;
use zodiac_parsing::tokenization::abstract_syntax::*;
use zodiac_parsing::tokenization::world_building::*;
use zodiac_layout::relationships::*;
use zodiac_layout::positioning::*;
use zodiac_layout::measurement::*;
use zodiac_layout::resizing::*;
use zodiac_layout::cleanup::*;
use zodiac_rendering::rendering::*;
use zodiac_rendering_glium::rendering::*;


#[derive(Debug)]
pub enum ZodiacError {
    FailedToLoadZodFile(file_system::Error),
    FailedParse(AbstractSyntaxTokenError),
    FailedToRender(RendererError)
}

impl From<AbstractSyntaxTokenError> for ZodiacError {
    fn from(error: AbstractSyntaxTokenError) -> Self {
        ZodiacError::FailedParse(error)
    }
}

impl From<file_system::Error> for ZodiacError {
    fn from(error: file_system::Error) -> Self {
        ZodiacError::FailedToLoadZodFile(error)
    }
}

impl From<RendererError> for ZodiacError {
    fn from(error: RendererError) -> Self {
        ZodiacError::FailedToRender(error)
    }
}

pub struct Application {
    pub world: World, 
    pub resources: Resources,
    schedule: Schedule
}

impl Application {
    pub fn build() -> Self {
        let world = World::default();
        let resources = Resources::default();
        let schedule = Schedule::builder()
            .add_system(build_relationship_map_system())
            .add_system(build_left_offset_map_system())
            .add_system(build_top_offset_map_system())
            .add_system(build_width_map_system())
            .add_system(build_height_map_system())
            .add_system(build_width_and_height_maps_from_radius_system())
            .add_system(build_layout_type_map_system())
            .add_system(resize_screen_system())
            .flush()
            .add_system(mark_as_mapped_system())
            .add_system(measure_fixed_width_constraints_system())
            .flush()
            .add_system(resize_system())
            .flush()
            .add_thread_local(render_primitives_system::<GliumRenderer>())
            .flush()
            .add_thread_local(remove_layout_change_system())
            .add_thread_local(remove_resized_system())
            .flush()
            .build();
            
        Self {
            world,
            resources,
            schedule
        }
    }

    pub fn initialise(mut self, zod_relative_folder_path: &str) -> Result<Application, ZodiacError>  {
        self.parse_to_world(self.load_app_zod_file_from_relative_path(zod_relative_folder_path)?.as_str())?;
        Ok(self)
    }
    
    fn parse_to_world(&mut self, text: &str) -> Result<(), ZodiacError> {
        let mut tokens = AbstractSyntaxTokenizer::from_source(SourceTokenizer::from_string(text));
        tokens.build_world(&mut self.world)?;
        Ok(())
    }
    
    fn load_app_zod_file_from_relative_path(&self, zod_relative_folder_path: &str) -> Result<String, ZodiacError> {
        Ok(file_system::load_app_zod_file_from_relative_path(zod_relative_folder_path)?)
    }
    
    pub fn run(mut self) -> Result<(), ZodiacError> {
        let event_loop: EventLoop<()> = EventLoop::new();

        &mut self.resources.insert(GliumRenderer::new(&event_loop)?);
        &mut self.resources.insert(create_relationship_map());
        &mut self.resources.insert(create_layout_type_map());
        &mut self.resources.insert(create_left_offset_map());
        &mut self.resources.insert(create_top_offset_map());
        &mut self.resources.insert(create_width_map());
        &mut self.resources.insert(create_height_map());
        &mut self.resources.insert(create_minimum_width_map());
        &mut self.resources.insert(create_minimum_height_map());

        event_loop.run(move |ev, _, control_flow| {
            &mut self.schedule.execute(&mut self.world, &mut self.resources);
            
            const SIXTY_FPS:u64 = 16_666_667;
            let next_frame_time = Instant::now() + Duration::from_nanos(SIXTY_FPS);
            *control_flow = ControlFlow::WaitUntil(next_frame_time);
            
            match ev {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                        return;
                    },
                    WindowEvent::Moved(position) => {
                        println!("root window positioned {:?}", position);

                    },
                    WindowEvent::Focused(is_focused) => {
                        println!("root window focused {:?}", is_focused);

                    },
                    WindowEvent::Resized(size) =>  
                    {
                        println!("root window resize {:?}", size);
                        self.world.push((RootWindowResized { width: size.width as u16, height: size.height as u16},));
                    },
                    _ => return,
                },
                _ => (),
            }
        });
    }
}