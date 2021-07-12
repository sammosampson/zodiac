use serde::*;
use zodiac::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RenderPrimitiveType {
    None,
    Element(ElementRenderPrimitive),
    Text(TextRenderPrimitive)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenderPrimitive {
    pub id: u64,
    pub dimensions: webrender::euclid::Rect<f32, webrender::api::units::LayoutPixel>,
    primitive_type: RenderPrimitiveType,
}

impl RenderPrimitive {
    pub fn set_element(&mut self, element: ElementRenderPrimitive) {
        self.primitive_type = RenderPrimitiveType::Element(element);
    }

    pub fn render(&self, pipeline_id: webrender::api::PipelineId, builder: &mut webrender::api::DisplayListBuilder) {
        let space_and_clip = Self::define_space_and_clip(pipeline_id);
        match self.primitive_type {
            RenderPrimitiveType::Element(element) => element.render(&self, pipeline_id, builder, space_and_clip),
            _ => todo!(),
        }
    }

    fn define_space_and_clip(pipeline_id: webrender::api::PipelineId) -> webrender::api::SpaceAndClipInfo {
        webrender::api::SpaceAndClipInfo::root_scroll(pipeline_id)
    }

    fn push_hit_test(
        &self,
        item_props: &webrender::api::CommonItemProperties,
        builder: &mut webrender::api::DisplayListBuilder,
    ) {
        builder.push_hit_test(item_props, (self.id, 0));

    }

    fn define_common_properties(
        &self,
        primitive: &RenderPrimitive,
        pipeline_id: webrender::api::PipelineId,
        clip_id: webrender::api::ClipId
    ) -> webrender::api::CommonItemProperties
    {
        webrender::api::CommonItemProperties {
            clip_id,
            clip_rect: primitive.dimensions,
            spatial_id: webrender::api::SpatialId::root_scroll_node(pipeline_id),
            flags: webrender::api::PrimitiveFlags::empty()
        }
    }
}

impl From<&ComponentId> for RenderPrimitive {
    fn from(id: &ComponentId) -> Self {
        Self {
            id: id.into(),
            dimensions: webrender::euclid::Rect::default(),
            primitive_type: RenderPrimitiveType::None
        }
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElementRenderPrimitive {
    pub border: Option<RenderPrimitiveBorder>,
    pub background_colour: webrender::api::ColorF,
    pub is_interactive: bool
}

impl ElementRenderPrimitive {
    pub fn new(border: Option<RenderPrimitiveBorder>, background_colour: webrender::api::ColorF) -> Self {
        Self {
            border,
            background_colour,    
            is_interactive: true
        }
    }

    fn render(
        &self,
        primitive: &RenderPrimitive,
        pipeline_id: webrender::api::PipelineId,
        builder: &mut webrender::api::DisplayListBuilder,
        space_and_clip: webrender::api::SpaceAndClipInfo
    ) {
        if self.can_render() {
            let clip_id = self.define_clip(primitive, pipeline_id, space_and_clip, builder);
            let item_props = primitive.define_common_properties(primitive, pipeline_id, clip_id);
            self.push_bounds_rect(primitive, &item_props, builder);
            primitive.push_hit_test(&item_props, builder);
            self.push_border(primitive, space_and_clip, builder);
        }
    }

    fn define_clip(
        &self,
        primitive: &RenderPrimitive,
        pipeline_id: webrender::api::PipelineId,
        space_and_clip: webrender::api::SpaceAndClipInfo,
        builder: &mut webrender::api::DisplayListBuilder,
    ) -> webrender::api::ClipId 
    {
        if let Some(border) = &self.border {
            builder.define_clip(
                &space_and_clip,
                primitive.dimensions,
                vec![webrender::api::ComplexClipRegion::new(
                    primitive.dimensions,
                    border.radius,
                    webrender::api::ClipMode::Clip
                )],
            )
        } else {
            webrender::api::ClipId::root(pipeline_id)
        }
    }

    fn push_bounds_rect(
        &self,
        primitive: &RenderPrimitive,
        item_props: &webrender::api::CommonItemProperties,
        builder: &mut webrender::api::DisplayListBuilder,
    ) {
        builder.push_rect(
            item_props, 
            primitive.dimensions, 
            self.background_colour
        );
    }

    fn push_border(
        &self,
        primitive: &RenderPrimitive,
        space_and_clip: webrender::api::SpaceAndClipInfo,
        builder: &mut webrender::api::DisplayListBuilder
    ) {
        if let Some(border) = &self.border {
            let common = webrender::api::CommonItemProperties::new(primitive.dimensions, space_and_clip);
            builder.push_border(
                &common,
                primitive.dimensions,
                border.widths,
                border.details
            );
        }
    }

    fn can_render(&self) -> bool {
        self.is_interactive || self.background_colour.a > 0.0
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenderPrimitiveBorder {
    pub radius: webrender::api::BorderRadius,
    pub widths: webrender::api::units::LayoutSideOffsets,
    pub details: webrender::api::BorderDetails
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextRenderPrimitive {
}