use std::fmt::Debug;
use log::{debug};

use legion::*;
use legion::storage::*;
use legion::systems::*;
use zodiac_entities::*;

pub fn create_world_builder<'a>(command_buffer: &'a mut CommandBuffer, root: Entity) -> WorldBuilder {
    WorldBuilder::<'a>::new(command_buffer, root)
}
pub struct WorldBuilder<'a> {
    command_buffer: &'a mut CommandBuffer,
    root_used_for_initial_entity: bool,
    current_entity: Entity,
    relationship_map: RelationshipMap
}

impl<'a> WorldBuilder<'a> {
    pub fn new(command_buffer: &'a mut CommandBuffer, root: Entity) -> Self {   
        let mut builder = Self {
            command_buffer,
            root_used_for_initial_entity: false,
            current_entity: root,
            relationship_map: RelationshipMap::new()
        };

        builder.set_relationship_component(builder.current_entity, Relationship::default());
        
        builder
    }

    pub fn set_root_used(&mut self) {
        self.root_used_for_initial_entity = true;
    }

    pub fn get_current_entity(&self) -> Entity {
        self.current_entity
    }

    pub fn complete_entity(&mut self) {
        debug!("completing entity {:?}", self.current_entity);
        if let Some(relationship) = self.get_relationship(self.current_entity) {
            if let Some(parent) = relationship.parent {
                self.current_entity = parent;
            }
        }
    }

    pub fn create_root_entity(&mut self) -> Entity {
        self.create_entity_with_component(Root::default());
        self.current_entity
    }

    pub fn create_control_entity(&mut self) -> Entity {
        self.create_entity_with_component(Control::default());
        self.current_entity
    }

    pub fn create_control_implementation(&mut self, source_entity: Entity) -> (Entity, SourceImplementation) {
        let source_implementation = SourceImplementation::from_source_entity(source_entity);
        self.create_entity_with_component(source_implementation);
        self.root_used_for_initial_entity = false;
        (self.current_entity, source_implementation)
    }    

    pub fn create_canvas_layout_content_entity(&mut self) -> Entity {
        self.create_entity_with_component(LayoutContent::canvas());
        self.current_entity
    }

    pub fn create_horizontal_layout_content_entity(&mut self) -> Entity {
        self.create_entity_with_component(LayoutContent::horizontal());
        self.current_entity
    }

    pub fn create_vertical_layout_content_entity(&mut self) -> Entity {
        self.create_entity_with_component(LayoutContent::vertical());
        self.current_entity
    }

    pub fn create_rectangle_entity(&mut self) -> Entity {
        self.create_entity_with_component(Renderable::rectangle());
        self.current_entity
    }
    
    pub fn create_circle_entity(&mut self) -> Entity {
        self.create_entity_with_component(Renderable::circle());
        self.current_entity
    }

    pub fn create_text_entity(&mut self) -> Entity {
        self.create_entity_with_component(Renderable::text());
        self.current_entity     
    }

    pub fn add_content_component(&mut self, content: &str) { 
        self.add_component_to_current_entity(Content::from(content));        
    }

    pub fn add_font_size_component(&mut self, size: u8) { 
        self.add_component_to_current_entity(FontSize::from(size));        
    }

    pub fn add_error_component_to_entity(&mut self, entity: Entity, error: BuildError) {
        debug!("Adding error ocurrence {:?} to {:?}", error, entity);
        self.add_component(entity, BuildErrorOccurrence::from(error));
    }

    pub fn add_canvas_layout_content_component(&mut self) {
        self.add_component_to_current_entity(LayoutContent::canvas());
    }
        
    pub fn add_left_component(&mut self, left: u16) {
        self.add_component_to_current_entity(Left::from(left));
    }
    
    pub fn add_top_component(&mut self, top: u16) {
        self.add_component_to_current_entity(Top::from(top));
    }
    
    pub fn add_width_component(&mut self, width: u16) {
        self.add_component_to_current_entity(Width::from(width));
    }
    
    pub fn add_height_component(&mut self, height: u16) {
        self.add_component_to_current_entity(Height::from(height));
    }

    pub fn add_radius_component(&mut self, radius: u16) {
        self.add_component_to_current_entity(Radius::from(radius));
    }
    
    pub fn add_stroke_width_component(&mut self, width: u16) {
        self.add_component_to_current_entity(StrokeWidth::from(width));
    }
    
    pub fn add_colour_component(&mut self, r: u8, g: u8, b: u8, a: u8) {
         self.add_component_to_current_entity(Colour::from((r, g, b, a)));
    }

    pub fn add_stroke_colour_component(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.add_component_to_current_entity(StrokeColour::from((r, g, b ,a)));
    }

    pub fn add_corner_radii_component(&mut self, left_top: u16, right_top: u16, right_bottom: u16, left_bottom: u16) {
        self.add_component_to_current_entity(CornerRadii::from((left_top, right_top, right_bottom, left_bottom)));
    }

    pub fn create_entity_with_component<T:Component + Debug>(&mut self, component: T) {
        if !self.root_used_for_initial_entity {
            self.add_component_to_current_entity(component);
            self.set_root_used();
            return;
        }

        let component_desc = format!("{:?}", component);
        let parent = self.current_entity;
        self.current_entity = self.command_buffer.push((component, ));
        debug!("creating entity with component {:?}: {}", self.current_entity, component_desc);
        self.setup_current_entity_relationships(parent);
    }

    fn setup_current_entity_relationships(&mut self, parent: Entity) {
        if let Some(mut parent_relationship) = self.get_relationship(parent) {
            if let Some(last_sibling) = parent_relationship.last_child {
                if let Some(mut last_sibling_relationship) = self.get_relationship(last_sibling) {
                    last_sibling_relationship.next_sibling = Some(self.current_entity);
                    self.add_component(last_sibling, last_sibling_relationship);
                }
            } else {
                parent_relationship.first_child = Some(self.current_entity);
            }
            parent_relationship.last_child = Some(self.current_entity);
            self.set_relationship_component(parent, parent_relationship);
        }

        self.add_parent_only_relationship_component(parent)
    }

    fn add_parent_only_relationship_component(&mut self, parent: Entity) {
        self.set_relationship_component(
            self.current_entity, 
            Relationship::for_parent_only(parent));
    }

    fn set_relationship_component(&mut self, entity: Entity, relationship: Relationship) {
        debug!("setting relationship for entity {:?}: {:?}", entity, relationship);
        
        self.add_component(entity, relationship);
        self.update_relationship_map(entity, relationship);
    }

    fn update_relationship_map(&mut self, entity: Entity, relationship: Relationship) {
        self.relationship_map.insert(entity, relationship);
    }

    fn get_relationship(&mut self, entity: Entity) -> Option<Relationship> {
        if let Some(relationship) = self.relationship_map.get(&entity) {
            return Some(*relationship);
        }
        None
    }

    fn add_component<T:Component>(&mut self, entity: Entity, component: T) {
        self.command_buffer.add_component(entity, component);
    }
    
    pub fn add_component_to_current_entity<T:Component + Debug>(&mut self, component: T) {
        debug!("adding entity component {:?}: {:?}", self.current_entity, component);
        self.add_component(self.current_entity, component)
    }
}