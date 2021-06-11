use log::info;
use shrev::*;
use zodiac::*;

pub fn create_layout_event_reader_registry(event_channel: &mut EventChannel::<SystemEvent>) -> LayoutEventReaderRegistry {
    LayoutEventReaderRegistry::register(event_channel)
}
pub struct LayoutEventReaderRegistry{
    pub (crate) resize_screen: ReaderId<SystemEvent>
}

impl LayoutEventReaderRegistry {
    fn register(event_channel: &mut EventChannel::<SystemEvent>) -> Self {
        info!("registering layout event readers");
        Self {
            resize_screen: event_channel.register_reader() 
        }
    }
}