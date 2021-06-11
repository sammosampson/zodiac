use zodiac::*;

pub trait Renderer {
    fn get_window_dimensions(&self) -> Dimensions;
}