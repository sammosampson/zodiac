use legion::*;
use legion::systems::*;
use zodiac::*;
use crate::style::*;

#[system(for_each)]
#[filter(component::<Rebuild>())]
#[filter(component::<Style>())]
pub fn rebuild_related_element_on_style_change(
    command_buffer: &mut CommandBuffer,
    style_relationship: &StyleRelationship
) {
    command_buffer.add_component(style_relationship.into(), Rebuild::default());
}  