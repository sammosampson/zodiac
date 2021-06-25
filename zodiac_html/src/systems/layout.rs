use legion::*;
use legion::systems::*;
use legion::world::*;
use zodiac::*;
use crate::layout::*;

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

#[system(simple)]
#[read_component(LayoutRequest)]
#[read_component(LayoutBox)]
pub fn layout(#[resource] relationship_map: &RelationshipMap, world: &mut SubWorld, command_buffer: &mut CommandBuffer) {
    let layout_box_tree = layout_box_tree(world, relationship_map);

    for (entity, layout_box) in <(Entity, &LayoutBox)>::query().filter(component::<LayoutRequest>()).iter(world) {
        perform_layout(&layout_box_tree, command_buffer, entity, layout_box);
    }

    todo!()
    // for all LayoutRequest's order by hierarchy depth and do lowest first
    // when iterating layout through tree add entities to a done hash map, use this to exclude from running through again for other LayoutRequest iterations
     
}

fn perform_layout(layout_box_tree: &LayoutBoxTree, command_buffer: &mut CommandBuffer, entity: &Entity, parent_layout_box: &LayoutBox) {
    for (child, layout_box) in layout_box_tree.get_children(*entity) {        
        let layout_change = layout_box.layout(parent_layout_box);    
        command_buffer.add_component(*entity, layout_change);
        command_buffer.remove_component::<LayoutRequest>(*entity);        
        perform_layout(layout_box_tree, command_buffer,&child,&layout_box);
    }

    todo!()
    //layout box should be placed in a layout node
    // on the way down: parent box should be "applied" to layout node
    // on the way up: each child box should be "applied" to layout node
    // this should push constraints down, and allow resizing based on child sizes
    // once all child nodes have been "applied". get_layout can be called and resultant Layout component can be added to entity
    
}

#[system(for_each)]
#[filter(component::<Layout>())]
pub fn remove_layout(command_buffer: &mut CommandBuffer, entity: &Entity) {
    command_buffer.remove_component::<Layout>(*entity);
}