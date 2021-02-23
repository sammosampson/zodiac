use legion::*;
use legion::systems::*;
use zodiac_entities::components::*;

#[system(for_each)]
#[filter(component::<LayoutChange>())]
pub fn remove_layout_change(command_buffer: &mut CommandBuffer, entity: &Entity) {
    command_buffer.remove_component::<LayoutChange>(*entity);
}

#[system(for_each)]
#[filter(component::<Resized>())]
pub fn remove_resized(command_buffer: &mut CommandBuffer, entity: &Entity) {
    command_buffer.remove_component::<Resized>(*entity);
    command_buffer.remove_component::<Mapped>(*entity);
}