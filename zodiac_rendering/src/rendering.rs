use zodiac_entities::*;

pub trait Renderer {
    fn get_window_dimensions(&self) -> Dimensions;
}