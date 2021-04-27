pub mod source;
pub mod rendering;

pub use source::*;
pub use rendering::*;
use zodiac_entities::*;
use zodiac_layout::standard_layout;

pub fn test_builders(dimensions: Dimensions) -> Vec::<Box::<dyn ApplicationBundleBuilder>> {
    vec!(
        Box::new(test_source_file_building()),
        Box::new(test_source_building()),
        Box::new(standard_layout()),
        Box::new(standard_test_rendering()),
        Box::new(test_renderer(dimensions)))
}