use legion::*;
use legion::storage::*;
use legion::systems::*;
use crate::components::*;
use crate::relationships::*;

pub trait EntityBuilder {
    fn add_entity_with_component<T:Component>(&mut self, component: T) -> Entity;
    fn add_component_to_entity<T:Component>(&mut self, entity: Entity, component: T);
}

impl EntityBuilder for CommandBuffer {
    fn add_entity_with_component<T:Component>(&mut self, component: T) -> Entity {
        self.push((component, ))
    }

    fn add_component_to_entity<T:Component>(&mut self, entity: Entity, component: T)  {
        self.add_component(entity, component);
    }
}

impl EntityBuilder for World {
    fn add_entity_with_component<T:Component>(&mut self, component: T) -> Entity {
        self.push((component, ))
    }
    
    fn add_component_to_entity<T:Component>(&mut self, entity: Entity, component: T)  {
        if let Some(mut entry) = self.entry(entity) {
            entry.add_component(component);
        }
    }
}

pub type WorldWorldEntityBuilder<'a> = WorldEntityBuilder<'a, World>;

pub fn world_entity_builder_for_world_with_root<'a>(world: &'a mut World) -> WorldWorldEntityBuilder<'a> {
    let root_entity = world.push((SourceFile::default(), ));
    world_entity_builder_for_world(world, root_entity)
}

pub fn world_entity_builder_for_world<'a>(world: &'a mut World, root: Entity) -> WorldWorldEntityBuilder<'a> {
    WorldWorldEntityBuilder::<'a>::new(world, root)
}

pub type CommandBufferWorldEntityBuilder<'a> = WorldEntityBuilder<'a, CommandBuffer>;

pub fn world_entity_builder_for_command_buffer<'a>(command_buffer: &'a mut CommandBuffer, root: Entity) -> CommandBufferWorldEntityBuilder {
    CommandBufferWorldEntityBuilder::<'a>::new(command_buffer, root)
}

pub struct WorldEntityBuilder<'a, TEntityBuilder: EntityBuilder> {
    entity_builder: &'a mut TEntityBuilder,
    root_used_for_initial_entity: bool,
    current_entity: Entity,
    relationship_map: RelationshipMap
}

impl<'a, TEntityBuilder: EntityBuilder> WorldEntityBuilder<'a, TEntityBuilder> {
    pub fn new(entity_builder: &'a mut TEntityBuilder, root: Entity) -> Self {   
        let mut builder = Self {
            entity_builder,
            root_used_for_initial_entity: false,
            current_entity: root,
            relationship_map: RelationshipMap::new()
        };

        builder.set_relationship_component(builder.current_entity, Relationship::default());
        
        builder
    }

    pub fn get_current_entity(&self) -> Entity {
        self.current_entity
    }

    pub fn complete_entity(&mut self) {
        if let Some(relationship) = self.get_relationship(self.current_entity) {
            if let Some(parent) = relationship.parent {
                self.current_entity = parent;
            }
        }
    } 
    
    pub fn create_root_entity(&mut self) {
        self.create_entity_with_component(Root::default());
        self.add_component_to_current_entity(LayoutContent::canvas());
    }
    
    pub fn create_canvas_layout_content_entity(&mut self) {
        self.create_entity_with_component(LayoutContent::canvas());
    }

    pub fn create_horizontal_layout_content_entity(&mut self) {
        self.create_entity_with_component(LayoutContent::horizontal());
    }

    pub fn create_vertical_layout_content_entity(&mut self) {
        self.create_entity_with_component(LayoutContent::vertical());
    }

    pub fn create_rectangle_entity(&mut self) {
        self.create_entity_with_component(Renderable::rectangle());
    }
    
    pub fn create_circle_entity(&mut self) {
        self.create_entity_with_component(Renderable::circle());
    }

    pub fn create_glyph_entity(&mut self) {
        self.create_entity_with_component(Renderable::glyph());
    }

    pub fn add_character_component(&mut self, character: char, position: usize) {
        self.add_component_to_current_entity(Character { character, position });
    }
    
    pub fn add_left_component(&mut self, left: u16) {
        self.add_component_to_current_entity(Left { left });
    }
    
    pub fn add_top_component(&mut self, top: u16) {
        self.add_component_to_current_entity(Top { top });
    }
    
    pub fn add_width_component(&mut self, width: u16) {
        self.add_component_to_current_entity(Width { width });
    }
    
    pub fn add_height_component(&mut self, height: u16) {
        self.add_component_to_current_entity(Height { height });
    }

    pub fn add_radius_component(&mut self, radius: u16) {
        self.add_component_to_current_entity(Radius { radius });
    }
    
    pub fn add_stroke_width_component(&mut self, width: u16) {
        self.add_component_to_current_entity(StrokeWidth { width });
    }
    
    pub fn add_glyph_index_component(&mut self, index: u16) {
        self.add_component_to_current_entity(GlyphIndex { index });
    }
    
    pub fn add_colour_component(&mut self, r: f32, g: f32, b: f32, a: f32) {
         self.add_component_to_current_entity(Colour { r, g, b ,a });
    }

    pub fn add_stroke_colour_component(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.add_component_to_current_entity(StrokeColour { r, g, b ,a });
    }

    pub fn add_corner_radii_component(&mut self, left_top: u16, right_top: u16, right_bottom: u16, left_bottom: u16) {
        self.add_component_to_current_entity(CornerRadii { left_top, right_top, right_bottom, left_bottom });
    }

    pub fn create_entity_with_component<T:Component>(&mut self, component: T) {
        if !self.root_used_for_initial_entity {
            self.add_component_to_current_entity(component);
            self.root_used_for_initial_entity = true;
            return;
        }
        let parent = self.current_entity;
        self.current_entity = self.entity_builder.add_entity_with_component(component);
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
        self.entity_builder.add_component_to_entity(entity, component);
    }
    
    pub fn add_component_to_current_entity<T:Component>(&mut self, component: T) {
        self.add_component(self.current_entity, component)
    }
}