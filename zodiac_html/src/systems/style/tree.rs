use std::collections::HashMap;
use legion::*;
use legion::world::*;
use legion::systems::*;
use zodiac::*;
use crate::Element;
use crate::ElementType;
use crate::borders::*;
use crate::window::*;
use crate::style::*;


#[system(for_each)]
#[filter(component::<Rebuild>())]
#[filter(component::<Window>())]
pub fn tag_default_style(
    command_buffer: &mut CommandBuffer,
    #[resource] relationship_map: &RelationshipMap,
    world: &mut SubWorld,
    entity: &Entity) {

    for child in relationship_map.get_children(entity) {
        let entry = world.entry_ref(child).unwrap();
        if let Ok(_style) = entry.into_component::<Style>() {
            command_buffer.add_component(child, DefaultStyle::default())    
        }
    }
}

#[system(for_each)]
#[filter(component::<Rebuild>())]
#[filter(component::<DefaultStyle>())]
pub fn build_default_style_tree(
    #[resource] relationship_map: &RelationshipMap,
    #[resource] style_tree: &mut StyleTree,
    world: &mut SubWorld,
    entity: &Entity) {
        style_tree.build(world, relationship_map, *entity);

}

#[system(for_each)]
#[filter(component::<Rebuild>())]
#[filter(!component::<Style>())]
pub fn apply_default_style_to_elements(
    #[resource] style_tree: &mut StyleTree,
    command_buffer: &mut CommandBuffer,
    element: &Element,
    entity: &Entity) {
        style_tree.apply_to_element(element, entity, command_buffer);

}

#[derive(Default)]
pub struct StyleTree(Option<StyleNode>);

impl StyleTree {
    fn build(&mut self, world: &mut SubWorld, relationship_map: &RelationshipMap, root: Entity) {
        self.0 = Some(self.build_node(world, relationship_map, root));        
    }

    fn build_node(&mut self, world: &mut SubWorld, relationship_map: &RelationshipMap, entity: Entity) -> StyleNode {
        let mut node = StyleNode::new(world, entity);
        for child in relationship_map.get_children(&entity) {
            node.add_child(self.build_node(world, relationship_map, child));
        }
        node
    }

    fn apply_to_element(&mut self, element: &Element, entity: &Entity, command_buffer: &mut CommandBuffer) {
        if let Some(root) = &self.0 {
            root.apply_to_element(entity, element, command_buffer)
        }
    }
}

pub struct StyleNode {
    selector: Option<ElementType>,
    children: HashMap<ElementType, StyleNode>,
    border: Border
}

impl StyleNode {
    fn new(world: &mut SubWorld, entity: Entity) -> Self {
        let entry = world.entry_ref(entity).unwrap();
        
        Self {
            selector: StyleNode::extract_selector(entry.get_component::<ElementSelector>().ok()),
            border: *entry.get_component::<Border>().unwrap(),
            children: HashMap::<ElementType, StyleNode>::default()
        }
    }

    fn extract_selector(from: Option<&ElementSelector>) -> Option<ElementType> {
        if let Some(selector) = from {
            Some(selector.into())
        } else {
            None
        }
    }

    fn add_child(&mut self, child: StyleNode) {
        if let Some(selector) = child.selector {
            self.children.insert(selector, child);
        }
    }

    fn apply_to_element(&self, entity: &Entity, element: &Element, command_buffer: &mut CommandBuffer) {
        if element.matches_selector(self.selector) {
            command_buffer.add_component(*entity, self.border)
        }

        if let Some(child) = self.children.get(&element.into()) {
            child.apply_to_element(entity, element, command_buffer);
        }
    }
}