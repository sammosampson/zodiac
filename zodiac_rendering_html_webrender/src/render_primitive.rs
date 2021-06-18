use serde::*;

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenderPrimitive {
    pub id: u64,
    pub dimensions: webrender::euclid::Rect<f32, webrender::api::units::LayoutPixel>,
    pub border: Option<RenderPrimitiveBorder>,
    pub background_colour: webrender::api::ColorF,
    pub is_interactive: bool
}


#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenderPrimitiveBorder {
    pub radius: webrender::api::BorderRadius,
    pub widths: webrender::api::units::LayoutSideOffsets,
    pub details: webrender::api::BorderDetails

}