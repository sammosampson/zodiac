use legion::*;
use legion::systems::*;
use legion::world::*;
use zodiac::*;
use mox::*;
use crate::layout::*;
use crate::colour::*;
use crate::borders::*;
use crate::size::*;
use super::selectors::*;
use super::tree::*;
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

                pub fn new(entry: &EntryRef) -> Self {
                    Self {
                        $($(
                            [<$component>]: *entry.get_component::<[<$component:camel>]>().unwrap(),
                        )*)?
                    }
                }

                pub fn merge_from(&mut self, from_set: &CopyableComponentSet) {
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

macro_rules! style_element {
    (
        $(extra_components {$(
            [$extra_component:expr]
        )*})?
        $(attributes {$(
            $attr:ident $(($attr_ty:ty))?
        )*})?
    ) => {
        element! {
            <style>
            Style::default(),
            extra_components {
                [ElementSelector::default()]
                $($(
                [$extra_component]
                )*)?
            }
            attributes {
                element_selector(ElementType)
                $($(
                $attr($($attr_ty)?)
                )*)?
            }             
        }
    }
}

pub fn default_style() -> Node {
    mox!(
        <style>
            <style
                element_selector=ElementType::Div
                display=DisplayTypes::Block
            />
            <style
                element_selector=ElementType::Span
                display=DisplayTypes::Inline
            />
        </style>
    )
}

macro_rules! style {
    (
        $(extra_components {$(
            [$extra_component:expr]
        )*})?
        $(attributes {$(
            $attr:ident $(($attr_ty:ty))?
        )*})?
    ) => {
        style_element! {
            extra_components {
            $($(
            [$extra_component]
            )*)?
            }
            attributes {
            $($(
            $attr($($attr_ty)?)
            )*)?
            }
        }
        copyable_component_set! {
            components {
            $($(
                $attr
            )*)?
            }
        }
        build_default_style_tree_system! {
            components {
            $($(
                $attr
            )*)?
            }
        }
        apply_styles_to_elements_system! {
            components {
            $($(
                $attr
            )*)?
            }
        }
    }
}

style! {
    extra_components {
        [Border::default()]
        [BorderWidth::default()]
        [BorderColour::default()]
        [BorderTop::default()]
        [BorderTopStyle::default()]
        [BorderTopColour::default()]
        [BorderTopWidth::default()]
        [BorderBottom::default()]
        [BorderBottomStyle::default()]
        [BorderBottomColour::default()]
        [BorderBottomWidth::default()]
        [BorderLeft::default()]
        [BorderLeftStyle::default()]
        [BorderLeftColour::default()]
        [BorderLeftWidth::default()]
        [BorderRight::default()]
        [BorderRightStyle::default()]
        [BorderRightColour::default()]
        [BorderRightWidth::default()]
        [BorderRadius::default()]
        [BorderStyle::default()]
        [Display::default()]
        [Margin::default()]
        [Padding::default()]
        [BackgroundColour::default()]
    }
    attributes {
        border(BorderValues)
        border_bottom(BorderValues)
        border_bottom_colour(Colour)
        border_bottom_style(BorderStyles)
        border_bottom_width(Size)
        border_colour(Colour)
        border_left(BorderValues)
        border_left_colour(Colour)
        border_left_style(BorderStyles)
        border_left_width(Size)
        border_radius(Size)
        border_right(BorderValues)
        border_right_colour(Colour)
        border_right_style(BorderStyles)
        border_right_width(Size)
        border_style(BorderStyles)
        border_top(BorderValues)
        border_top_colour(Colour)
        border_top_style(BorderStyles)
        border_top_width(Size)
        border_width(Size)
        background_colour(Colour)
        display(DisplayTypes)
        margin(MarginSizes)
        padding(PaddingSizes)
    }
}
