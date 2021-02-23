use legion::*;
use legion::storage::*;
use crate::components::*;

pub struct WorldEntityBuilder<'a> {
    world: &'a mut World,
    current_entity: Entity,
}

impl<'a> WorldEntityBuilder<'a> {
    pub fn for_world(world: &'a mut World) -> Self {
        let screen_entity = world.push((
            Root {},
            LayoutContent { layout_type: LayoutType::Canvas }, 
            Relationship { parent: None, next_sibling: None, first_child: None, last_child: None }
        ));
        
        Self {
            world,
            current_entity: screen_entity
        }
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
    
    pub fn create_canvas_layout_content_entity(&mut self) {
        self.create_entity_with_component(LayoutContent { layout_type: LayoutType::Canvas });
    }
    
    pub fn create_horizontal_layout_content_entity(&mut self) {
        self.create_entity_with_component(LayoutContent { layout_type: LayoutType::Horizontal });
    }

    pub fn create_vertical_layout_content_entity(&mut self) {
        self.create_entity_with_component(LayoutContent { layout_type: LayoutType::Vertical });
    }

    pub fn create_rectangle_entity(&mut self) {
        self.create_entity_with_component(Renderable { render_type: RenderType::Rectangle });
    }
    
    pub fn create_circle_entity(&mut self) {
        self.create_entity_with_component(Renderable { render_type: RenderType::Circle });
    }
    
    pub fn create_text_entity(&mut self) {
        self.create_entity_with_component(Renderable { render_type: RenderType::Text });
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

    pub fn add_corner_radii_component(&mut self, left_top: f32, right_top: f32, right_bottom: f32, left_bottom: f32) {
        self.add_component_to_current_entity(CornerRadii { left_top, right_top, right_bottom, left_bottom });
    }

    pub fn create_entity_with_component<T:Component>(&mut self, component: T) {
        let parent = self.current_entity;
        self.current_entity = self.world.push((component,));
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
            self.add_component(parent, parent_relationship);
        }

        self.add_relationship_component(Some(parent), None, None, None)
    }

    fn add_relationship_component(
        &mut self,
        parent: Option<Entity>, 
        next_sibling: Option<Entity>, 
        first_child: Option<Entity>, 
        last_child: Option<Entity>) {
        self.add_component_to_current_entity(Relationship { parent, next_sibling, first_child, last_child });
    }

    fn get_relationship(&mut self, entity: Entity) -> Option<Relationship> {
        if let Some(entry) = self.world.entry(entity) {
            return Some(*entry.get_component::<Relationship>().unwrap());
        }
        None
    }

    fn add_component<T:Component>(&mut self, entity: Entity, component: T) {
        if let Some(mut entry) = self.world.entry(entity) {
            entry.add_component(component);
        }
    }
    
    pub fn add_component_to_current_entity<T:Component>(&mut self, component: T) {
        self.add_component(self.current_entity, component)
    }
}