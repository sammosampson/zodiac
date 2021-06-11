use crate::Dimensions;

pub trait Renderer {
    fn get_window_dimensions(&self) -> Dimensions;
}