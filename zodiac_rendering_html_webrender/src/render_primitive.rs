use serde::*;
use zodiac::*;

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenderPrimitive {
    pub id: u64,
    pub dimensions: webrender::euclid::Rect<f32, webrender::api::units::LayoutPixel>,
    pub border: Option<RenderPrimitiveBorder>,
    pub background_colour: webrender::api::ColorF,
    pub is_interactive: bool
}

impl From<&ComponentId> for RenderPrimitive {
    fn from(id: &ComponentId) -> Self {
        Self {
            id: id.into(),
            dimensions: webrender::euclid::Rect::default(),
            border: None,
            background_colour: webrender::api::ColorF::default(),
            is_interactive: true
        }
    }
}


#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenderPrimitiveBorder {
    pub radius: webrender::api::BorderRadius,
    pub widths: webrender::api::units::LayoutSideOffsets,
    pub details: webrender::api::BorderDetails
}