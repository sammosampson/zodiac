use std::{collections::HashMap};
use legion::*;
use zodiac_entities::*;
use crate::building::*;
use crate::source::*;

pub fn create_import_control_lookup() -> ImportControlEntityLookup {
    ImportControlEntityLookup::new()
}

pub type ImportControlEntityLookup = HashMap<String, Entity>;



pub struct MutableBuildResources<'a> {
    pub world_builder: &'a mut WorldBuilder<'a>,
    pub import_control_lookup: ImportControlEntityLookup
}

impl <'a> MutableBuildResources<'a> {
    pub fn new(world_builder: &'a mut WorldBuilder<'a>) -> Self {
        Self {
            world_builder,
            import_control_lookup: create_import_control_lookup()
        }
    }
}

pub struct BuildResources<'a, T:SourceReader> {
    pub source_entity_lookup: &'a SourceEntityLookup,
    pub source_location_lookup: &'a SourceLocationLookup,
    pub source_tokens_lookup: &'a SourceTokensLookup,
    pub source_reader: &'a T
}

pub fn build<'a, T:SourceReader + 'a> (
    root_source_implementation_entity: &Entity,
    source_implementation: &SourceImplementation,
    build_resources: &BuildResources<T>,
    build_resources_mut: &mut MutableBuildResources) -> Result<(), BuildError> {
        
    let mut current_entity_builders = vec!();
    let tokens = build_resources.source_tokens_lookup
        .get(&source_implementation.source_file_entity)
        .ok_or(BuildError::new(*root_source_implementation_entity, BuildErrorReason::ControlSourceFileDoesNotExist))?;

    for token in tokens {
        match token {
            AbstractSyntaxToken::Root => current_entity_builders.push(create_root_builder(build_resources_mut)),
            AbstractSyntaxToken::Import => current_entity_builders.push(create_import_builder(*root_source_implementation_entity, *source_implementation)),
            AbstractSyntaxToken::Circle => current_entity_builders.push(create_circle_builder(build_resources_mut)),
            AbstractSyntaxToken::Rectangle => current_entity_builders.push(create_rectangle_builder(build_resources_mut)),
            AbstractSyntaxToken::Text => current_entity_builders.push(create_text_builder(build_resources_mut)),
            AbstractSyntaxToken::Control => current_entity_builders.push(create_control_builder(build_resources_mut)),
            AbstractSyntaxToken::ControlImplementation(control_name) => current_entity_builders.push(create_control_implementation_builder(control_name.to_owned())),
            AbstractSyntaxToken::CanvasLayoutContent => current_entity_builders.push(create_canvas_builder(build_resources_mut)),
            AbstractSyntaxToken::HorizontalLayoutContent => current_entity_builders.push(create_horizontal_layout_builder(build_resources_mut)),
            AbstractSyntaxToken::VerticalLayoutContent => current_entity_builders.push(create_vertical_layout_builder(build_resources_mut)),
            AbstractSyntaxToken::CompleteControl => {
                if let Some(current_entity_builder) = current_entity_builders.pop() {
                    current_entity_builder.build(build_resources, build_resources_mut)?
                }
            },
            property_token => {
                if let Some(mut current_entity_builder) = current_entity_builders.pop() {
                    current_entity_builder.process_property_token(property_token)?;
                    current_entity_builders.push(current_entity_builder);
                }
            }
        }
    }

    Ok(())
}

pub fn create_root_builder<T:SourceReader>(build_resources_mut: &mut MutableBuildResources) -> Box<dyn EntityBuilder<T>> {
    let builder = StandardBuilder::new(build_resources_mut.world_builder.create_root_entity());
    build_resources_mut.world_builder.add_canvas_layout_content_component();
    Box::new(builder)
}

pub fn create_import_builder<T:SourceReader>(root_entity: Entity, source_implementation: SourceImplementation) -> Box<dyn EntityBuilder<T>> {
    Box::new(ImportBuilder::new(root_entity, source_implementation))
}

pub fn create_control_builder<T:SourceReader>(build_resources_mut: &mut MutableBuildResources) -> Box<dyn EntityBuilder<T>> {
    let builder = StandardBuilder::new(build_resources_mut.world_builder.create_control_entity());
    build_resources_mut.world_builder.add_canvas_layout_content_component();
    Box::new(builder)
}

pub fn create_control_implementation_builder<T:SourceReader>(control_name: String) -> Box<dyn EntityBuilder<T>> {
    Box::new(ControlImplementationBuilder::new(control_name))
}

pub fn create_rectangle_builder<T:SourceReader>(build_resources: &mut MutableBuildResources) -> Box<dyn EntityBuilder<T>> {
    let mut builder = StandardBuilder::new(build_resources.world_builder.create_rectangle_entity());
    
    builder
        .with_required_property(AbstractSyntaxNodeType::Colour)
        .with_required_property(AbstractSyntaxNodeType::StrokeColour)
        .with_required_property(AbstractSyntaxNodeType::StrokeWidth)
        .with_required_property(AbstractSyntaxNodeType::CornerRadii)
        .with_optional_property(AbstractSyntaxNodeType::Width)
        .with_optional_property(AbstractSyntaxNodeType::Height)
        .with_optional_property(AbstractSyntaxNodeType::Left)
        .with_optional_property(AbstractSyntaxNodeType::Top);
    
    Box::new(builder)
}

pub fn create_circle_builder<T:SourceReader>(build_resources: &mut MutableBuildResources) -> Box<dyn EntityBuilder<T>> {
    let mut builder = StandardBuilder::new(build_resources.world_builder.create_circle_entity());
    
    builder
        .with_required_property(AbstractSyntaxNodeType::Colour)
        .with_required_property(AbstractSyntaxNodeType::StrokeColour)
        .with_required_property(AbstractSyntaxNodeType::StrokeWidth)
        .with_optional_property(AbstractSyntaxNodeType::Radius)
        .with_optional_property(AbstractSyntaxNodeType::Left)
        .with_optional_property(AbstractSyntaxNodeType::Top);

    Box::new(builder)
}

pub fn create_text_builder<T:SourceReader>(build_resources: &mut MutableBuildResources) -> Box<dyn EntityBuilder<T>> {
    let mut builder = StandardBuilder::new(build_resources.world_builder.create_canvas_layout_content_entity());
    
    builder
        .with_required_property(AbstractSyntaxNodeType::Content)
        .with_required_property(AbstractSyntaxNodeType::Colour)
        .with_optional_property(AbstractSyntaxNodeType::Width)
        .with_optional_property(AbstractSyntaxNodeType::Height)
        .with_optional_property(AbstractSyntaxNodeType::Left)
        .with_optional_property(AbstractSyntaxNodeType::Top);

    Box::new(builder)
}

pub fn create_canvas_builder<T:SourceReader>(build_resources: &mut MutableBuildResources) -> Box<dyn EntityBuilder<T>> {
    let mut builder = StandardBuilder::new(build_resources.world_builder.create_canvas_layout_content_entity());
    
    builder
        .with_optional_property(AbstractSyntaxNodeType::Width)
        .with_optional_property(AbstractSyntaxNodeType::Height)
        .with_optional_property(AbstractSyntaxNodeType::Left)
        .with_optional_property(AbstractSyntaxNodeType::Top);

    Box::new(builder)
}

pub fn create_horizontal_layout_builder<T:SourceReader>(build_resources: &mut MutableBuildResources) -> Box<dyn EntityBuilder<T>> {
    let mut builder = StandardBuilder::new(build_resources.world_builder.create_horizontal_layout_content_entity());
    
    builder
        .with_optional_property(AbstractSyntaxNodeType::Width)
        .with_optional_property(AbstractSyntaxNodeType::Height)
        .with_optional_property(AbstractSyntaxNodeType::Left)
        .with_optional_property(AbstractSyntaxNodeType::Top);

    Box::new(builder)
}

pub fn create_vertical_layout_builder<T:SourceReader>(build_resources: &mut MutableBuildResources) -> Box<dyn EntityBuilder<T>> {
    let mut builder = StandardBuilder::new(build_resources.world_builder.create_vertical_layout_content_entity());
    
    builder
        .with_optional_property(AbstractSyntaxNodeType::Width)
        .with_optional_property(AbstractSyntaxNodeType::Height)
        .with_optional_property(AbstractSyntaxNodeType::Left)
        .with_optional_property(AbstractSyntaxNodeType::Top);

    Box::new(builder)
}

pub trait EntityBuilder<T:SourceReader> {
    fn process_property_token(&mut self, token: &AbstractSyntaxToken)-> Result<(), BuildError>; 
    fn build<'a>(&self, build_resources: &BuildResources<T>, build_resources_mut: &mut MutableBuildResources<'a>) -> Result<(), BuildError>;
}
struct StandardBuilder {
    entity: Entity,
    tokens: AbstractSyntaxPropertyTokens
}

impl StandardBuilder {
    fn new(entity: Entity) -> Self {
        Self {
            entity,
            tokens: AbstractSyntaxPropertyTokens::new()
        }
    }
    
    fn with_required_property(&mut self, property: AbstractSyntaxNodeType) -> &mut Self {
        self.tokens.with_required_property(property);
        self
    }

    fn with_optional_property(&mut self, property: AbstractSyntaxNodeType) -> &mut Self {
        self.tokens.with_optional_property(property);
        self
    }

    fn create_parse_error(&self, reason: BuildErrorReason) -> Result<(), BuildError> {
        Err(BuildError::new(self.entity, reason))
    }
}

impl<T:SourceReader> EntityBuilder<T> for StandardBuilder {
    fn process_property_token(&mut self, token: &AbstractSyntaxToken) -> Result<(), BuildError> {
        self.tokens.process_property_token(self.entity, token)?;
        Ok(())
    }

    fn build(&self, _: &BuildResources<T>, build_resources_mut: &mut MutableBuildResources) -> Result<(), BuildError> {
        println!("building entity {:?}", self.entity);
        let processed_tokens = self.tokens.get_processed_tokens(self.entity)?;
        for token in processed_tokens {
            match token {
                AbstractSyntaxToken::Left(left) => build_resources_mut.world_builder.add_left_component(left),
                AbstractSyntaxToken::Top(top) => build_resources_mut.world_builder.add_top_component(top),
                AbstractSyntaxToken::Width(width) => build_resources_mut.world_builder.add_width_component(width),
                AbstractSyntaxToken::Height(height) => build_resources_mut.world_builder.add_height_component(height),
                AbstractSyntaxToken::Radius(radius) => build_resources_mut.world_builder.add_radius_component(radius),
                AbstractSyntaxToken::StrokeWidth(width) => build_resources_mut.world_builder.add_stroke_width_component(width),
                AbstractSyntaxToken::Content(content) => build_resources_mut.world_builder.add_text_content_components(&content),
                AbstractSyntaxToken::Colour((r, g, b ,a)) => build_resources_mut.world_builder.add_colour_component(r, g, b ,a),
                AbstractSyntaxToken::StrokeColour((r, g, b ,a)) => build_resources_mut.world_builder.add_stroke_colour_component(r, g, b ,a),
                AbstractSyntaxToken::CornerRadii((left_top, right_top, right_bottom, left_bottom)) => build_resources_mut.world_builder.add_corner_radii_component(
                    left_top,
                    right_top,
                    right_bottom,
                    left_bottom),
                value => return self.create_parse_error(BuildErrorReason::UnexpectedToken(AbstractSyntaxNodeType::from(&value)))
            }
        }

        build_resources_mut.world_builder.complete_entity();

        Ok(())
    }
}

struct ImportBuilder {
    root_entity: Entity,
    source_implementation: SourceImplementation,
    tokens: AbstractSyntaxPropertyTokens
}

impl ImportBuilder {
    fn new(root_entity: Entity, source_implementation: SourceImplementation) -> Self {
        let mut tokens = AbstractSyntaxPropertyTokens::new();

        tokens
            .with_required_property(AbstractSyntaxNodeType::Name)
            .with_required_property(AbstractSyntaxNodeType::Path);
            
        Self {
            root_entity,
            source_implementation,
            tokens
        }
    }

    fn create_parse_error(&self, reason: BuildErrorReason) -> Result<(), BuildError> {
        Err(BuildError::new(self.root_entity, reason))
    }
}

impl<T:SourceReader> EntityBuilder<T> for ImportBuilder {
    fn process_property_token(&mut self, token: &AbstractSyntaxToken)-> Result<(), BuildError> {
        self.tokens.process_property_token(self.root_entity, token)?;
        Ok(())
    }

    fn build(&self, build_resources: &BuildResources<T>, build_resources_mut: &mut MutableBuildResources) -> Result<(), BuildError> {
        println!("building import {:?}", self.root_entity);
        
        let mut import_relative_path = String::default();
        let mut import_control_name = String::default();

        for token in self.tokens.get_processed_tokens(self.root_entity)? {
            match token {
                AbstractSyntaxToken::Path(path) => import_relative_path.push_str(&path),
                AbstractSyntaxToken::Name(name) => import_control_name.push_str(&name),
                value => return self.create_parse_error(BuildErrorReason::UnexpectedToken(AbstractSyntaxNodeType::from(&value)))
            }
        }

        let root_location = build_resources.source_location_lookup
            .get(&self.source_implementation.source_file_entity)
            .unwrap();
        
        if let Ok(full_import_control_location) = build_resources.source_reader.get_relative_source_location(root_location,  &import_relative_path) {
            if let Some(import_entity)  = build_resources.source_entity_lookup.get(&full_import_control_location) {
                build_resources_mut.import_control_lookup.insert(import_control_name, *import_entity);
            }
            else {
                return self.create_parse_error(BuildErrorReason::ControlSourceDoesNotExist(String::from(full_import_control_location)));
            }
        } else {
            return self.create_parse_error(BuildErrorReason::SourceLocationDoesNotExist(import_relative_path))
        }

        Ok(())
    }
}

struct ControlImplementationBuilder {
    control_name: String
}

impl ControlImplementationBuilder {
    fn new(control_name: String) -> Self {
        Self {
            control_name
        }
    }
}

impl<T:SourceReader> EntityBuilder<T> for ControlImplementationBuilder {
    fn process_property_token(&mut self, _: &AbstractSyntaxToken)-> Result<(), BuildError> {
        Ok(())
    }

    fn build<'a>(&self, build_resources: &BuildResources<T>, build_resources_mut: &mut MutableBuildResources<'a>) -> Result<(), BuildError> {
        if let Some(source_entity) = build_resources_mut.import_control_lookup.get(&self.control_name) {
            let (control_impl_entity, source_implementation)  = build_resources_mut
                .world_builder
                .create_control_implementation(*source_entity);

            build(
                &control_impl_entity, 
                &source_implementation,
                build_resources,
                build_resources_mut)
        }
        else {
            return Err(BuildError::new(
                build_resources_mut.world_builder.get_current_entity(), 
                BuildErrorReason::ControlDoesNotExist(self.control_name.clone())
            ));
        }
    }
}