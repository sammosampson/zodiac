use log::{debug};
use legion::*;
use zodiac_entities::*;
use crate::building::*;
use super::*;

pub fn create_root_builder<T:SourceReader>(build_resources_mut: &mut MutableBuildResources) -> Box<dyn EntityBuilder<T>> {
    let builder = StandardBuilder::new(build_resources_mut.world_builder.create_root_entity());
    build_resources_mut.world_builder.add_canvas_layout_content_component();
    Box::new(builder)
}

pub fn create_control_builder<T:SourceReader>(build_resources_mut: &mut MutableBuildResources) -> Box<dyn EntityBuilder<T>> {
    let builder = StandardBuilder::new(build_resources_mut.world_builder.create_control_entity());
    build_resources_mut.world_builder.add_canvas_layout_content_component();
    Box::new(builder)
}

pub fn create_rectangle_builder<T:SourceReader>(build_resources: &mut MutableBuildResources) -> Box<dyn EntityBuilder<T>> {
    let mut builder = StandardBuilder::new(build_resources.world_builder.create_rectangle_entity());
    
    builder
        .with_required_token(AbstractSyntaxNodeType::Colour)
        .with_required_token(AbstractSyntaxNodeType::StrokeColour)
        .with_required_token(AbstractSyntaxNodeType::StrokeWidth)
        .with_required_token(AbstractSyntaxNodeType::CornerRadii)
        .with_optional_token(AbstractSyntaxNodeType::Width)
        .with_optional_token(AbstractSyntaxNodeType::Height)
        .with_optional_token(AbstractSyntaxNodeType::Left)
        .with_optional_token(AbstractSyntaxNodeType::Top);
    
    Box::new(builder)
}

pub fn create_circle_builder<T:SourceReader>(build_resources: &mut MutableBuildResources) -> Box<dyn EntityBuilder<T>> {
    let mut builder = StandardBuilder::new(build_resources.world_builder.create_circle_entity());
    
    builder
        .with_required_token(AbstractSyntaxNodeType::Colour)
        .with_required_token(AbstractSyntaxNodeType::StrokeColour)
        .with_required_token(AbstractSyntaxNodeType::StrokeWidth)
        .with_optional_token(AbstractSyntaxNodeType::Radius)
        .with_optional_token(AbstractSyntaxNodeType::Left)
        .with_optional_token(AbstractSyntaxNodeType::Top);

    Box::new(builder)
}

pub fn create_text_builder<T:SourceReader>(build_resources: &mut MutableBuildResources) -> Box<dyn EntityBuilder<T>> {
    let mut builder = StandardBuilder::new(build_resources.world_builder.create_text_entity());
    
    builder
        .with_required_token(AbstractSyntaxNodeType::Content)
        .with_required_token(AbstractSyntaxNodeType::FontSize)
        .with_required_token(AbstractSyntaxNodeType::Colour)
        .with_optional_token(AbstractSyntaxNodeType::Width)
        .with_optional_token(AbstractSyntaxNodeType::Height)
        .with_optional_token(AbstractSyntaxNodeType::Left)
        .with_optional_token(AbstractSyntaxNodeType::Top);

    Box::new(builder)
}

pub fn create_canvas_builder<T:SourceReader>(build_resources: &mut MutableBuildResources) -> Box<dyn EntityBuilder<T>> {
    let mut builder = StandardBuilder::new(build_resources.world_builder.create_canvas_layout_content_entity());
    
    builder
        .with_optional_token(AbstractSyntaxNodeType::Width)
        .with_optional_token(AbstractSyntaxNodeType::Height)
        .with_optional_token(AbstractSyntaxNodeType::Left)
        .with_optional_token(AbstractSyntaxNodeType::Top);

    Box::new(builder)
}

pub fn create_horizontal_layout_builder<T:SourceReader>(build_resources: &mut MutableBuildResources) -> Box<dyn EntityBuilder<T>> {
    let mut builder = StandardBuilder::new(build_resources.world_builder.create_horizontal_layout_content_entity());
    
    builder
        .with_optional_token(AbstractSyntaxNodeType::Width)
        .with_optional_token(AbstractSyntaxNodeType::Height)
        .with_optional_token(AbstractSyntaxNodeType::Left)
        .with_optional_token(AbstractSyntaxNodeType::Top);

    Box::new(builder)
}

pub fn create_vertical_layout_builder<T:SourceReader>(build_resources: &mut MutableBuildResources) -> Box<dyn EntityBuilder<T>> {
    let mut builder = StandardBuilder::new(build_resources.world_builder.create_vertical_layout_content_entity());
    
    builder
        .with_optional_token(AbstractSyntaxNodeType::Width)
        .with_optional_token(AbstractSyntaxNodeType::Height)
        .with_optional_token(AbstractSyntaxNodeType::Left)
        .with_optional_token(AbstractSyntaxNodeType::Top);

    Box::new(builder)
}

struct StandardBuilder {
    entity: Entity,
    tokens: AbstractSyntaxPropertyTokenRequirements
}

impl StandardBuilder {
    fn new(entity: Entity) -> Self {
        Self {
            entity,
            tokens: AbstractSyntaxPropertyTokenRequirements::new()
        }
    }
    
    fn with_required_token(&mut self, property: AbstractSyntaxNodeType) -> &mut Self {
        self.tokens.with_required_token(property);
        self
    }

    fn with_optional_token(&mut self, property: AbstractSyntaxNodeType) -> &mut Self {
        self.tokens.with_optional_token(property);
        self
    }
}

impl<T:SourceReader> EntityBuilder<T> for StandardBuilder {
    fn get_entity<'a>(&self, _: &mut MutableBuildResources<'a>) -> Entity {
        self.entity
    }

    fn process_token(&mut self, token: &AbstractSyntaxToken) -> Result<(), BuildError> {
        self.tokens.process_token(token)?;
        Ok(())
    }

    fn build(&self, _: &BuildResources<T>, build_resources_mut: &mut MutableBuildResources) -> Result<(), BuildError> {
        debug!("building entity {:?}", self.entity);
        let processed_tokens = self.tokens.get_processed_tokens()?;
        for token in processed_tokens {
            match token {
                AbstractSyntaxToken::Left(left) => build_resources_mut.world_builder.add_left_component(left),
                AbstractSyntaxToken::Top(top) => build_resources_mut.world_builder.add_top_component(top),
                AbstractSyntaxToken::Width(width) => build_resources_mut.world_builder.add_width_component(width),
                AbstractSyntaxToken::Height(height) => build_resources_mut.world_builder.add_height_component(height),
                AbstractSyntaxToken::Radius(radius) => build_resources_mut.world_builder.add_radius_component(radius),
                AbstractSyntaxToken::StrokeWidth(width) => build_resources_mut.world_builder.add_stroke_width_component(width),
                AbstractSyntaxToken::FontSize(size) => build_resources_mut.world_builder.add_font_size_component(size),
                AbstractSyntaxToken::Content(content) => build_resources_mut.world_builder.add_content_component(&content),
                AbstractSyntaxToken::Colour((r, g, b ,a)) => build_resources_mut.world_builder.add_colour_component(r, g, b ,a),
                AbstractSyntaxToken::StrokeColour((r, g, b ,a)) => build_resources_mut.world_builder.add_stroke_colour_component(r, g, b ,a),
                AbstractSyntaxToken::CornerRadii((left_top, right_top, right_bottom, left_bottom)) => build_resources_mut.world_builder.add_corner_radii_component(
                    left_top,
                    right_top,
                    right_bottom,
                    left_bottom),
                value => return Err(BuildError::UnexpectedToken(AbstractSyntaxNodeType::from(&value)))
            }
        }

        build_resources_mut.world_builder.complete_entity();

        Ok(())
    }
}