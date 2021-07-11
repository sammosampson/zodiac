use shrev::*;
use legion::*;
use legion::systems::*;
use zodiac::*;
use crate::window::*;
use crate::borders::*;
use crate::layout::*;
use crate::style::*;
use crate::colour::*;
use crate::size::*;
use crate::events::*;
use crate::systems::*;

pub fn html_builder() -> HtmlBuilder {
    HtmlBuilder::default()
}

#[derive(Default, Debug)]
pub struct HtmlBuilder {
}

impl ApplicationBundleBuilder for HtmlBuilder {
    fn description(&self) -> String {
        "html webrender rendering".to_string()
    }

    fn setup_build_systems(&self, _: &mut Builder) {
    }

    fn setup_layout_systems(&self, builder: &mut Builder) {
        builder
            .add_system(tag_default_style_system())
            .add_system(initialise_element_layout_system())
            .add_system(rebuild_related_element_on_style_change_system())
            .flush()
            .add_system(build_default_style_tree_system())
            .flush()
            .add_system(apply_styles_to_elements_system())
            .flush()
            .add_system(deconstruct_border_system())
            .flush()
            .add_system(deconstruct_border_colour_system())
            .add_system(deconstruct_border_width_system())
            .add_system(deconstruct_border_style_system())
            .flush()
            .add_system(upconstruct_border_top_system())
            .add_system(upconstruct_border_left_system())
            .add_system(upconstruct_border_bottom_system())
            .add_system(upconstruct_border_right_system())
            .flush()
            .add_system(compose_full_border_system())
            .flush()
            .add_system(root_resize_system())
            .add_system(compose_display_to_layout_box_system())
            .flush()
            .add_system(apply_layout_differences_system())
            .flush()
            .add_system(layout_system());
    }

    fn setup_rendering_systems(&self, _: &mut Builder) {
    }

    fn setup_cleanup_systems(&self, builder: &mut Builder) {
        builder
            .add_system(remove_layout_changes_system())
            .add_system(remove_layout_requests_system());
    }

    fn setup_final_functions(&self, _: &mut Builder) {
    }

    fn setup_resources(&self, resources: &mut Resources, event_channel: &mut EventChannel<SystemEvent>) -> Result<(), ZodiacError>  {
        resources.insert(create_layout_event_reader_registry(event_channel));
        resources.insert(create_style_trees());
        Ok(())
    }

    fn register_components_for_world_serializiation(&self, world_serializer: &mut WorldSerializer) {
        world_serializer.register_component::<Window>(stringify!(Window));
        world_serializer.register_component::<WindowOpen>(stringify!(WindowOpen));
        world_serializer.register_component::<Height>(stringify!(Height));
        world_serializer.register_component::<Width>(stringify!(Width));
        world_serializer.register_component::<Title>(stringify!(Title));
        world_serializer.register_component::<Style>(stringify!(Style));
        world_serializer.register_component::<DefaultStyle>(stringify!(DefaultStyle));
        world_serializer.register_component::<Element>(stringify!(Element));
        world_serializer.register_component::<ElementType>(stringify!(ElementType));
        world_serializer.register_component::<ElementSelector>(stringify!(ElementSelector));
        world_serializer.register_component::<BorderWidth>(stringify!(BorderWidth));
        world_serializer.register_component::<BorderColour>(stringify!(BorderColour));
        world_serializer.register_component::<BorderTop>(stringify!(BorderTop));
        world_serializer.register_component::<BorderTopStyle>(stringify!(BorderTopStyle));
        world_serializer.register_component::<BorderTopColour>(stringify!(BorderTopColour));
        world_serializer.register_component::<BorderTopWidth>(stringify!(BorderTopWidth));
        world_serializer.register_component::<BorderBottom>(stringify!(BorderBottom));
        world_serializer.register_component::<BorderBottomStyle>(stringify!(BorderBottomStyle));
        world_serializer.register_component::<BorderBottomColour>(stringify!(BorderBottomColour));
        world_serializer.register_component::<BorderBottomWidth>(stringify!(BorderBottomWidth));
        world_serializer.register_component::<BorderLeft>(stringify!(BorderLeft));
        world_serializer.register_component::<BorderLeftStyle>(stringify!(BorderLeftStyle));
        world_serializer.register_component::<BorderLeftColour>(stringify!(BorderLeftColour));
        world_serializer.register_component::<BorderLeftWidth>(stringify!(BorderLeftWidth));
        world_serializer.register_component::<BorderRight>(stringify!(BorderRight));
        world_serializer.register_component::<BorderRightStyle>(stringify!(BorderRightStyle));
        world_serializer.register_component::<BorderRightColour>(stringify!(BorderRightColour));
        world_serializer.register_component::<BorderRightWidth>(stringify!(BorderRightWidth));
        world_serializer.register_component::<BorderRadius>(stringify!(BorderRadius));
        world_serializer.register_component::<BorderStyles>(stringify!(BorderStyles));
        world_serializer.register_component::<BorderStyle>(stringify!(BorderStyle));
        world_serializer.register_component::<BorderValues>(stringify!(BorderValues));
        world_serializer.register_component::<Margin>(stringify!(Margin));
        world_serializer.register_component::<Padding>(stringify!(Padding));
        world_serializer.register_component::<FullBorder>(stringify!(FullBorder));
        world_serializer.register_component::<BackgroundColour>(stringify!(BackgroundColour));
        world_serializer.register_component::<Display>(stringify!(Display));
        world_serializer.register_component::<DisplayTypes>(stringify!(DisplayTypes));
        world_serializer.register_component::<Size>(stringify!(Size));
        world_serializer.register_component::<Colour>(stringify!(Colour));
        world_serializer.register_component::<LayoutRequest>(stringify!(LayoutRequest));
        world_serializer.register_component::<LayoutBox>(stringify!(LayoutBox));
        world_serializer.register_component::<StyleLayoutBox>(stringify!(StyleLayoutBox));
        world_serializer.register_component::<ResolvedLayoutBox>(stringify!(ResolvedLayoutBox));
        world_serializer.register_component::<LayoutStatus>(stringify!(LayoutStatus));
        world_serializer.register_component::<LayoutDirection>(stringify!(LayoutDirection));
        world_serializer.register_component::<LayoutOffsetRect>(stringify!(LayoutOffsetRect));
        world_serializer.register_component::<ResolvedLayoutOffsetRect>(stringify!(ResolvedLayoutOffsetRect));
        world_serializer.register_component::<LayoutDimensions>(stringify!(LayoutDimensions));
        world_serializer.register_component::<ResolvedLayoutDimensions>(stringify!(ResolvedLayoutDimensions));
        world_serializer.register_component::<LayoutDistance>(stringify!(LayoutDistance));
        world_serializer.register_component::<ResolvedLayoutDistance>(stringify!(ResolvedLayoutDistance));
    }
}