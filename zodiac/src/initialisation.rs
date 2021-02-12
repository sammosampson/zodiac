use std::time::*;
use legion::*;
use glium::*;
use glium::glutin::event_loop::*;
use glutin::event::*;
use zodiac_parsing::tokenization::source::SourceTokenizer;
use zodiac_parsing::tokenization::abstract_syntax::{AbstractSyntaxTokenizer, AbstractSyntaxTokenError};
use zodiac_resources::file_system;
use zodiac_rendering::rendering::*;
use zodiac_rendering_glium::rendering::*;
use crate::systems::rendering::*;
use crate::systems::relationships::*;
use crate::systems::layout::*;
use crate::world_building::abstract_syntax::WorldBuilder;

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
            .add_system(position_children_of_canvases_system())
            .flush()
            .add_thread_local(render_primitives_system::<GliumRenderer>())
            .add_thread_local(complete_render_system())
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
        &mut self.resources.insert(RelationshipMap::new());
        &mut self.resources.insert(AbsoluteOffsetMap::new());

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
                    WindowEvent::Resized(_) => {}
                    _ => return,
                },
                _ => (),
            }
        });
    }
}