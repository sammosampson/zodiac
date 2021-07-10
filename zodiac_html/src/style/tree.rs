use std::collections::HashMap;
use legion::*;
use legion::world::*;
use legion::systems::*;
use zodiac::*;

use crate::borders::*;
use crate::colour::*;
use crate::layout::*;
use super::selectors::*;
use super::Style;
use super::DefaultStyle;

pub fn create_syle_tree() -> StyleTree {
    StyleTree::default()
}

#[derive(Default)]
pub struct StyleTree(Option<StyleNode>);

impl StyleTree {
    pub fn build_default(&mut self, world: &mut SubWorld, relationship_map: &RelationshipMap, root: Entity) {
        self.0 = Some(self.build_node(world, relationship_map, root, true));        
    }

    fn build_node(&mut self, world: &mut SubWorld, relationship_map: &RelationshipMap, entity: Entity, is_default: bool) -> StyleNode {
        let mut node = StyleNode::new(world, entity, is_default);
        for child in relationship_map.get_children(&entity) {
            node.add_child(self.build_node(world, relationship_map, child, false));
        }
        node
    }

    pub fn apply_to_element(&mut self, element: &Element, entity: &Entity, command_buffer: &mut CommandBuffer) {
        if let Some(root) = &self.0 {
            root.apply_to_element(entity, element, command_buffer)
        }
    }
}

macro_rules! copyable_component_set {
    (
        $(components {$(
            $component:ident
        )*})?
    ) => {
        paste::item! {
            #[system(for_each)]
            #[filter(component::<Rebuild>())]
            #[filter(component::<DefaultStyle>())]
            $($(
                #[read_component([<$component:camel>])]
            )*)?
            pub fn build_default_style_tree(
                #[resource] relationship_map: &RelationshipMap,
                #[resource] style_tree: &mut StyleTree,
                world: &mut SubWorld,
                entity: &Entity) {
                    style_tree.build_default(world, relationship_map, *entity);
            }

            #[system(for_each)]
            #[filter(component::<Rebuild>())]
            #[filter(!component::<Style>())]
            $($(
                #[read_component([<$component:camel>])]
            )*)?
            pub fn apply_default_style_to_elements(
                #[resource] style_tree: &mut StyleTree,
                command_buffer: &mut CommandBuffer,
                element: &Element,
                entity: &Entity) {
                    style_tree.apply_to_element(element, entity, command_buffer);
            }

            pub struct CopyableComponentSet  {
                is_default: bool,
                $($(
                    [<$component>]: [<$component:camel>],
                )*)?
            }

            impl CopyableComponentSet {
                fn new(entry: EntryRef, is_default: bool) -> Self {
                    Self {
                        is_default,
                        $($(
                            [<$component>]: *entry.get_component::<[<$component:camel>]>().unwrap(),
                        )*)?
                    }
                }
                
                fn copy_to_element(&self, to_entity: &Entity, command_buffer: &mut CommandBuffer) {
                    $($(
                        if self.[<$component>].is_set() || self.is_default {
                            command_buffer.add_component(*to_entity, self.[<$component>])
                        }
                    )*)?
                }
            }
        }
    }
}

copyable_component_set! {
    components {
        border
        border_bottom
        border_bottom_colour
        border_bottom_style
        border_bottom_width
        border_colour
        border_left
        border_left_colour
        border_left_style
        border_left_width
        border_radius
        border_right
        border_right_colour
        border_right_style
        border_right_width
        border_style
        border_top
        border_top_colour
        border_top_style
        border_top_width
        border_width
        background_colour
        display
        margin
        padding
    }
}

pub struct StyleNode {
    selector: Option<ElementType>,
    children: HashMap<ElementType, StyleNode>,
    component_set: CopyableComponentSet
}

impl StyleNode {
    fn new(world: &mut SubWorld, entity: Entity, is_default: bool) -> Self {
        let entry = world.entry_ref(entity).unwrap();
        
        Self {
            selector: StyleNode::extract_selector(entry.get_component::<ElementSelector>().ok()),
            children: HashMap::<ElementType, StyleNode>::default(),
            component_set: CopyableComponentSet::new(entry, is_default)
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
            self.component_set.copy_to_element(entity, command_buffer);
        }

        if let Some(child) = self.children.get(&element.into()) {
            child.apply_to_element(entity, element, command_buffer);
        }
    }
}
