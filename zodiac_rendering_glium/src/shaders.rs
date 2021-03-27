use glium::*;

pub fn get_vertex_shader_src() -> String {
    String::from_utf8_lossy(include_bytes!("../shaders/vertex_shader.glsl")).to_string()
}

pub fn get_fragment_shader_src() -> String {
    #[cfg(feature = "render-quads_only")]  
    return String::from_utf8_lossy(include_bytes!("../shaders/colour_only_fragment_shader.glsl")).to_string();
    #[cfg(not(feature = "render-quads_only"))]  
    return String::from_utf8_lossy(include_bytes!("../shaders/fragment_shader.glsl")).to_string();
}

pub fn get_geometry_shader_src() -> String {
    String::from_utf8_lossy(include_bytes!("../shaders/geometry_shader.glsl")).to_string()
}

pub fn create_shader_program(display: &Display) -> Result<Program, ProgramCreationError> {
    Program::from_source(display, &get_vertex_shader_src(), &get_fragment_shader_src(), Some(&get_geometry_shader_src()))
}
    