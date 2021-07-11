use legion::systems::*;
use std::collections::HashMap;
use legion::*;
use legion::world::*;
use zodiac::*;
use crate::borders::*;
use crate::colour::*;
use crate::layout::*;
use super::selectors::*;
use super::Style;
use super::DefaultStyle;

macro_rules! build_default_style_tree_system {
    (
        $(components {$(
            $component:ident
        )*})?
    ) => {
        paste::item! {
            #[system(for_each)]
            #[filter(component::<Rebuild>())]
            #[filter(component::<DefaultStyle>())]
            #[read_component(ElementSelector)]
            $($(
                #[read_component([<$component:camel>])]
            )*)?
            pub fn build_default_style_tree(
                #[resource] relationship_map: &RelationshipMap,
                #[resource] style_trees: &mut StyleTrees,
                world: &mut SubWorld,
                entity: &Entity) {
                    style_trees.build_default(world, relationship_map, *entity);
            }
        }
    }
}

macro_rules! apply_styles_to_elements_system {
    (
        $(components {$(
            $component:ident
        )*})?
    ) => {
        paste::item! {
            #[system(for_each)]
            #[filter(component::<Rebuild>())]
            #[filter(!component::<Style>())]
            $($(
                #[read_component([<$component:camel>])]
            )*)?
            pub fn apply_styles_to_elements(
                command_buffer: &mut CommandBuffer,
                world: &SubWorld,
                #[resource] style_trees: &mut StyleTrees,
                element: &Element,
                style_relationship: Option<&StyleRelationship>,
                entity: &Entity) {
                if let Some(default_style) = style_trees.get_applicable_node(element) {
                    let default_style_components = default_style.components();
                    if let Some(style) = style_relationship {
                        let mut style_components = CopyableComponentSet::from_world(world, style.into());
                        style_components.merge_from(default_style_components);
                        style_components.copy_to_element(entity, command_buffer);
                    } else {
                        default_style_components.copy_to_element(entity, command_buffer);
                    }
                }
            }
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
            #[derive(Debug)]
            pub struct CopyableComponentSet  {
                $($(
                    [<$component>]: [<$component:camel>],
                )*)?
            }

            impl CopyableComponentSet {
                fn from_world(world: &SubWorld, entity: Entity) -> Self {
                    let entry = world.entry_ref(entity).unwrap();
                    Self::new(&entry)                 
                }

                fn new(entry: &EntryRef) -> Self {
                    Self {
                        $($(
                            [<$component>]: *entry.get_component::<[<$component:camel>]>().unwrap(),
                        )*)?
                    }
                }

                fn merge_from(&mut self, from_set: &CopyableComponentSet) {
                    $($(
                        if !self.[<$component>].is_set() {
                            self.[<$component>] = from_set.[<$component>];
                        }
                    )*)?
                }
                
                fn copy_to_element(&self, to_entity: &Entity, command_buffer: &mut CommandBuffer) {
                    $($(
                        command_buffer.add_component(*to_entity, self.[<$component>]);
                    )*)?
                }
            }
        }
    }
}
macro_rules! style {
    (
        $(components {$(
            $component:ident
        )*})?
    ) => {
        copyable_component_set! {
            components {
            $($(
                $component
            )*)?
            }
        }
        build_default_style_tree_system! {
            components {
            $($(
                $component
            )*)?
            }
        }
        apply_styles_to_elements_system! {
            components {
            $($(
                $component
            )*)?
            }
        }
    }
}

style! {
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

    fn components(&self) -> &CopyableComponentSet {
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
