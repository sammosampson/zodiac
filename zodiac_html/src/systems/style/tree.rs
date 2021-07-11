use legion::*;
use legion::world::*;
use legion::systems::*;
use zodiac::*;
use crate::window::*;
use crate::style::*;

#[system(for_each)]
#[filter(component::<Rebuild>())]
#[filter(component::<Window>())]
#[read_component(Style)]
pub fn tag_default_style(
    world: &SubWorld,
    command_buffer: &mut CommandBuffer,
    #[resource] relationship_map: &RelationshipMap,
    entity: &Entity) {

    for child in relationship_map.get_children(entity) {
        let entry = world.entry_ref(child).unwrap();
        if let Ok(_style) = entry.into_component::<Style>() {
            command_buffer.add_component(child, DefaultStyle::default())    
        }
    }
}  