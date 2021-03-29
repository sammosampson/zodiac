use std::time::*;
use legion::*;
use glium::*;
use glium::glutin::event_loop::*;
use glutin::event::*;
use zodiac_entities::*;
use zodiac_resources::*;
use zodiac_parsing::*;
use zodiac_layout::*;
use zodiac_rendering::*;
use zodiac_rendering_glium::*;

#[derive(Debug)]
pub enum ZodiacError {
    FailedToRender(RendererError),
    FailedToFileMonitorFiles(FileMonitorError)
}

impl From<RendererError> for ZodiacError {
    fn from(error: RendererError) -> Self {
        ZodiacError::FailedToRender(error)
    }
}

impl From<FileMonitorError> for ZodiacError {
    fn from(error: FileMonitorError) -> Self {
        ZodiacError::FailedToFileMonitorFiles(error)
    }
}

pub struct Application {
    pub world: World, 
    pub resources: Resources,
    schedule: Schedule,
    relative_zod_folder_path: &'static str,
    file_monitor_poll: Duration
}

impl Application {
    pub fn build() -> Self {
        let world = World::default();
        let resources = Resources::default();
        let schedule = Schedule::builder()
            .add_thread_local(recurisve_source_location_build_system())
            .flush()
            .add_thread_local(source_file_monitoring_system())
            .flush()
            .add_system(set_root_layout_system())
            .flush()
            .add_system(source_file_parse_system::<FileSourceReader>())
            .flush()
            .add_system(build_relationship_map_system())
            .add_system(build_text_colour_map_system())
            .flush()
            .add_system(format_glyphs_system())
            .flush()
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
            schedule,
            relative_zod_folder_path: "",
            file_monitor_poll: Duration::from_secs(1)
        }
    }

    pub fn initialise(mut self, relative_zod_folder_path: &'static str) -> Application  {
        self.relative_zod_folder_path = relative_zod_folder_path;
        self
    }
    
    pub fn run(mut self) -> Result<(), ZodiacError> {
        let event_loop: EventLoop<()> = EventLoop::new();

        let file_paths = FilePaths::new(self.relative_zod_folder_path);
        
        &mut self.resources.insert(file_paths);
        &mut self.resources.insert(create_source_file_reader());
        &mut self.resources.insert(create_source_file_entity_lookup());
        &mut self.resources.insert(create_source_location_lookup());
        &mut self.resources.insert(monitor_files(file_paths, self.file_monitor_poll)?);
        &mut self.resources.insert(create_text_colour_map());
        &mut self.resources.insert(create_relationship_map());
        &mut self.resources.insert(create_layout_type_map());
        &mut self.resources.insert(create_left_offset_map());
        &mut self.resources.insert(create_top_offset_map());
        &mut self.resources.insert(create_width_map());
        &mut self.resources.insert(create_height_map());
        &mut self.resources.insert(create_minimum_width_map());
        &mut self.resources.insert(create_minimum_height_map());
        &mut self.resources.insert(GliumRenderer::new(&event_loop)?);

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