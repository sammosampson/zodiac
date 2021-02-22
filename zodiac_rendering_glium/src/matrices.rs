pub fn identity_matrix() -> [[f32; 4]; 4] {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0f32]
    ]
}

pub fn orthographic_camera_matrix(width:u32, height: u32) -> [[f32; 2]; 2]  {
    let perspective = {
        let aspect_ratio = height as f32 / width as f32;
        [
            [aspect_ratio, 0.0],
            [0.0, aspect_ratio]
        ]
    };
    perspective
}
pub fn orthographic_view_matrix(screen_width:u32, screen_height: u32, resolution_width: u32, resolution_height: u32) -> [[f32; 2]; 2]  {
    let perspective = {
        let scale_width = resolution_width as f32 / screen_width as f32;
        let scale_height = resolution_height as f32 / screen_height as f32;
        [
            [scale_width, 0.0],
            [0.0, scale_height]
        ]
    };
    perspective
}