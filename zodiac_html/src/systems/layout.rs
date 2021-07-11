use legion::*;
use legion::systems::*;
use legion::world::*;
use log::info;
use zodiac::*;
use crate::window::*;
use crate::layout::*;
use crate::style::*;

#[system(for_each)]
#[filter(!component::<Root>())]
#[filter(!component::<Style>())]
#[filter(!component::<LayoutBox>())]
pub fn initialise_element_layout(command_buffer: &mut CommandBuffer, entity: &Entity) {
    info!("initialising element layout: {:?}", entity);
    command_buffer.add_component(*entity, StyleLayoutBox::default());    
    command_buffer.add_component(*entity, LayoutBox::default());    
    command_buffer.add_component(*entity, ResolvedLayoutBox::default());    
}

#[system(for_each)]
pub fn apply_layout_differences(
    command_buffer: &mut CommandBuffer,
    entity: &Entity,
    style_layout_box: &StyleLayoutBox,
    layout_box: &mut LayoutBox) {
    if layout_box.apply(style_layout_box) {
        info!("applying layout differences: {:?}", entity);
        command_buffer.add_component(*entity, LayoutRequest::default());
    }    
}

#[system(for_each)]
#[filter(component::<Window>())]
#[read_component(LayoutBox)]
#[read_component(ResolvedLayoutBox)]
#[read_component(LayoutRequest)]
pub fn layout(
    #[resource] relationship_map: &RelationshipMap, 
    command_buffer: &mut CommandBuffer,
    world: &mut SubWorld, 
    entity: &Entity) {
    info!("performing layout: {:?}", entity);
    let layout_tree = layout_tree(world, relationship_map);
    layout_tree.layout(entity);        
    layout_tree.position(entity, command_buffer);        
}    

#[system(for_each)]
#[filter(component::<LayoutChange>())]
pub fn remove_layout_changes(command_buffer: &mut CommandBuffer, entity: &Entity) {
    info!("removing layout changes: {:?}", entity);
    command_buffer.remove_component::<LayoutChange>(*entity);
}

#[system(for_each)]
#[filter(component::<LayoutRequest>())]
pub fn remove_layout_requests(command_buffer: &mut CommandBuffer, entity: &Entity) {
    info!("removing layout request: {:?}", entity);
    command_buffer.remove_component::<LayoutRequest>(*entity);
}