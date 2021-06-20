use legion::*;
use legion::systems::*;
use zodiac::*;
use crate::testing::*;

#[system(for_each)]
#[filter(component::<TestRenderable>())]
pub fn queue_render_primitives(
    entity: &Entity, 
    layout_change: &LayoutChange, 
    command_buffer: &mut CommandBuffer,
    #[resource] render_queue: &mut TestRenderQueue) {
    render_queue.queue_for_render(
        command_buffer,
        entity,
        [layout_change.left, layout_change.top],
        [layout_change.width, layout_change.height]);
}