use std::time::*;
use legion::*;
use glium::*;
use glium::glutin::event_loop::*;
use glutin::event::*;
use glium::backend::glutin::*;
use glium::texture::*;
use zodiac_parsing::tokenization::source::SourceTokenizer;
use zodiac_parsing::tokenization::abstract_syntax::{AbstractSyntaxTokenizer, AbstractSyntaxTokenError};
use zodiac_resources::file_system;
use zodiac_rendering_glium::systems::*;
use zodiac_rendering_glium::shaders::*;
use zodiac_rendering_glium::display::*;
use zodiac_rendering_glium::fonts::*;
use crate::abstract_syntax::world_building::WorldBuilder;

#[derive(Debug)]
pub enum ZodiacError {
    FailedToLoadZodFile(file_system::Error),
    FailedParse(AbstractSyntaxTokenError),
    FailedToDisplayWindow(DisplayCreationError),
    FailedToCreateShaders(ProgramCreationError),
    FailedToLoadFont(TextureCreationError)
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

impl From<DisplayCreationError> for ZodiacError {
    fn from(error: DisplayCreationError) -> Self {
        ZodiacError::FailedToDisplayWindow(error)
    }
}

impl From<ProgramCreationError> for ZodiacError {
    fn from(error: ProgramCreationError) -> Self {
        ZodiacError::FailedToCreateShaders(error)
    }
}

impl From<TextureCreationError> for ZodiacError {
    fn from(error: TextureCreationError) -> Self {
        ZodiacError::FailedToLoadFont(error)
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
            //.add_system(window_event_loop_system())
            .add_thread_local(render_primitives_system())
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
        let display = create_display(&event_loop)?;
        let shader_program = create_shader_program(&display)?;
        let font_array = create_font_array(&display)?;

        &mut self.resources.insert(display);
        &mut self.resources.insert(shader_program);
        &mut self.resources.insert(font_array);

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
                    _ => return,
                },
                _ => (),
            }
        });
    }
}