use legion::*;
use legion::systems::*;
use legion::world::*;
use shrev::EventChannel;
use zodiac::*;
use crate::events::*;
use crate::layout::*;

#[system(simple)]
#[read_component(Root)]
#[write_component(LayoutRequest)]
pub fn root_resize(
    world: &mut SubWorld,
    command_buffer: &mut CommandBuffer,
    #[resource] event_channel: &mut EventChannel::<SystemEvent>,
    #[resource] event_readers: &mut LayoutEventReaderRegistry
) {
    for event in event_channel.read(&mut event_readers.resize_screen) {
        match event {
            SystemEvent::Window(SystemWindowEventType::RootWindowResize(dimensions)) => {
                for root in <Entity>::query()
                    .filter(component::<Root>())
                    .iter(world) {
                        command_buffer.add_component(*root, IncumbentLayoutBox::from(dimensions));
                        command_buffer.add_component(*root, LayoutBox::default());
                        command_buffer.add_component(*root, LayoutRequest::default());
                }
            },
            _ => {}
        }
    }
}