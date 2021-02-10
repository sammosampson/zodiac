use legion::*;
use legion::storage::*;
use zodiac_entities::components::*;

pub struct WorldEntityBuilder<'a> {
    world: &'a mut World,
    current_entity: Entity,
}

impl<'a> WorldEntityBuilder<'a> {
    fn add_component<T:Component>(&mut self, entity: Entity, component: T) {
        if let Some(mut entry) = self.world.entry(entity) {
            entry.add_component(component);
        }
    }
    pub fn add_component_to_current_entity<T:Component>(&mut self, component: T) {
        self.add_component(self.current_entity, component)
    }

    fn create_entity_with_component<T:Component>(&mut self, component: T){
        self.current_entity = self.world.push((component, ));
    }
    
    pub fn create_horizontal_layout_content_entity(&mut self) {
        self.create_entity_with_component(HorizontalLayoutContent {});
    }

    pub fn create_rectangle_entity(&mut self) {
        self.create_entity_with_component(Rectangle {});
    }
    
    pub fn create_circle_entity(&mut self) {
        self.create_entity_with_component(Circle {});
    }
    
    pub fn create_text_entity(&mut self) {
        self.create_entity_with_component(Text {});
    }
    
    pub fn add_position_component(&mut self, x: u16, y: u16) {
        self.add_component_to_current_entity(Position { x, y});
    }
    
    pub fn add_dimensions_component(&mut self, x: u16, y: u16) {
        self.add_component_to_current_entity(Dimensions { x, y});
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

    pub fn for_world(world: &'a mut World) -> Self {
        let screen_entity = world.push((CanvasLayoutContent {},));
        Self {
            world,
            current_entity: screen_entity
        }
    }
}