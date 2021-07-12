use std::collections::HashMap;
use legion::*;
use legion::world::*;
use zodiac::*;
use crate::layout::*;
use super::selectors::*;
use super::embedding::*;


pub fn create_style_trees() -> StyleTrees {
    StyleTrees::default()
}

#[derive(Default)]
pub struct StyleTrees {
    default: Option<StyleTree>
}

impl StyleTrees {
    pub fn build_default(&mut self, world: &mut SubWorld, relationship_map: &RelationshipMap, root: Entity) {
        self.default = Some(StyleTree::build(world, relationship_map, root));
    }

    pub fn get_applicable_node(&self, element: &Element) -> Option<&StyleNode> {
        if let Some(default) = &self.default {
            return Some(default.get_applicable_node(element));
        }
        None
    }
}

pub struct StyleTree(StyleNode);

impl StyleTree {
    pub fn build(world: &mut SubWorld, relationship_map: &RelationshipMap, root: Entity) -> Self {
        let mut tree = Self(StyleTree::build_node(world, relationship_map, root));
        tree.merge_tree();
        tree
    }

    fn build_node(world: &mut SubWorld, relationship_map: &RelationshipMap, entity: Entity) -> StyleNode {
        let mut node = StyleNode::new(world, entity);
        for child in relationship_map.get_children(&entity) {
            node.add_child(StyleTree::build_node(world, relationship_map, child));
        }
        node
    }

    fn merge_tree(&mut self) {
        &self.0.merge_with_children();
    }

    fn get_applicable_node(&self, element: &Element) -> &StyleNode {
        self.0.get_applicable_node(element)
    }
}

pub struct StyleNode {
    selector: Option<ElementType>,
    children: HashMap<ElementType, StyleNode>,
    component_set: CopyableComponentSet
}

impl StyleNode {
    fn new(world: &mut SubWorld, entity: Entity) -> Self {
        let entry = world.entry_ref(entity).unwrap();
        let component_set = CopyableComponentSet::new(&entry);
        Self {
            selector: StyleNode::extract_selector(entry.get_component::<ElementSelector>()),
            children: HashMap::<ElementType, StyleNode>::default(),
            component_set
        }
    }

    pub fn components(&self) -> &CopyableComponentSet {
        &self.component_set
    }

    fn extract_selector(from: Result<&ElementSelector, ComponentError>) -> Option<ElementType> {
        if let Ok(selector) = from {
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

    fn merge_with_children(&mut self) {
        for child in self.children.values_mut() {
            child.component_set.merge_from(&self.component_set);
            child.merge_with_children();
        }
    }

    fn get_applicable_node(&self, element: &Element) -> &Self {
        if let Some(child) = self.children.get(&element.into()) {
            return child.get_applicable_node(element);
        } else {
            self
        }
    }
}
