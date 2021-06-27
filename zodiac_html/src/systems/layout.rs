use legion::*;
use legion::systems::*;
use legion::world::*;
use zodiac::*;
use crate::layout::*;

#[system(for_each)]
#[filter(!component::<LayoutBox>())]
pub fn initialise_layout(command_buffer: &mut CommandBuffer, entity: &Entity) {
    command_buffer.add_component(*entity, LayoutBox::default());    
    command_buffer.add_component(*entity, IncumbentLayoutBox::default());    
    command_buffer.add_component(*entity, ResolvedLayoutBox::default());    
}

#[system(for_each)]
pub fn apply_layout_differences(
    command_buffer: &mut CommandBuffer,
    entity: &Entity,
    incumbent_layout_box: &IncumbentLayoutBox,
    layout_box: &mut LayoutBox) {
    
    if layout_box.apply(incumbent_layout_box) {
        command_buffer.add_component(*entity, LayoutRequest::default());
    }    
}

#[system(for_each)]
#[filter(component::<Root>())]
#[read_component(LayoutBox)]
#[read_component(ResolvedLayoutBox)]
#[read_component(LayoutRequest)]
pub fn layout(
    #[resource] relationship_map: &RelationshipMap, 
    command_buffer: &mut CommandBuffer,
    world: &mut SubWorld, 
    root: &Entity) {
    let layout_tree = layout_tree(world, relationship_map);
    layout_tree.layout(root);        
    layout_tree.position(root, command_buffer);        
}


#[system(for_each)]
#[filter(component::<LayoutChange>())]
pub fn remove_layout_changes(command_buffer: &mut CommandBuffer, entity: &Entity) {
    command_buffer.remove_component::<LayoutChange>(*entity);
}

#[system(for_each)]
#[filter(component::<LayoutRequest>())]
pub fn remove_layout_requests(command_buffer: &mut CommandBuffer, entity: &Entity) {
    command_buffer.remove_component::<LayoutRequest>(*entity);
}