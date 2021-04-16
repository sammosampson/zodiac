use log::{debug};
use std::collections::*;
use legion::*;
use legion::world::*;
use legion::systems::*;
use zodiac_entities::*;

#[system(simple)]
#[read_component(SourceFile)]
#[read_component(SourceFileInitialRead)]
#[read_component(SourceFileRoot)]
pub fn apply_initially_read_root_source_to_world (
    world: &mut SubWorld,
    command_buffer: &mut CommandBuffer) {
    if let Some(root_source ) = get_initially_read_root_source(world) {
        add_enity_with_source_implementation(command_buffer, root_source);
    }
}

#[system(simple)]
#[read_component(SourceFile)]
#[read_component(SourceFileRemoval)]
#[read_component(Root)]
#[read_component(Relationship)]
pub fn apply_created_source_to_world (
    world: &mut SubWorld,
    command_buffer: &mut CommandBuffer,
    #[resource] relationship_map: &mut RelationshipMap) {

        if get_created_sources(world).len() == 0 {
            return;
        }
    
        remove_root_children(world, command_buffer, relationship_map);
}

#[system(simple)]
#[read_component(SourceFile)]
#[read_component(SourceFileRemoval)]
#[read_component(Root)]
#[read_component(Relationship)]
pub fn apply_removed_source_to_world (
    world: &mut SubWorld,
    command_buffer: &mut CommandBuffer,
    #[resource] relationship_map: &mut RelationshipMap) {

    if get_removed_sources(world).len() == 0 {
        return;
    }

    remove_root_children(world, command_buffer, relationship_map);
}

#[system(simple)]
#[read_component(SourceFile)]
#[read_component(SourceFileChange)]
#[read_component(SourceFileRemoval)]
#[read_component(Relationship)]
#[read_component(SourceImplementation)]
pub fn apply_changed_source_to_world (
    world: &mut SubWorld,
    command_buffer: &mut CommandBuffer,
    #[resource] relationship_map: &mut RelationshipMap) {
    
    let changed_sources = get_changed_sources(world);

    if changed_sources.len() == 0 {
        return;
    }

    for (source_impl_entity, relationship, source_impl) in 
        <(Entity, &Relationship, &SourceImplementation)>::query().iter(world) {
        if !source_implementation_has_changed(source_impl, &changed_sources) {
            continue;
        }
        remove_source_implementations_children_from_world(
            command_buffer, 
            relationship_map, 
            *relationship, 
            *source_impl_entity);
    }
}

fn get_created_sources(world: &mut SubWorld) -> Vec::<Entity> {
    <Entity>
        ::query()
        .filter(component::<SourceFileCreation>())
        .iter(world)
        .map(|entity|*entity)
        .collect()
}

fn get_removed_sources(world: &mut SubWorld) -> Vec::<Entity> {
    <Entity>
        ::query()
        .filter(component::<SourceFileRemoval>())
        .iter(world)
        .map(|entity|*entity)
        .collect()
}

fn get_changed_sources(world: &mut SubWorld) -> HashMap::<Entity, Entity>{
    <Entity>
        ::query()
        .filter(component::<SourceFileChange>())
        .iter(world)
        .map(|entity|(*entity, *entity))
        .collect()
}

fn remove_root_children(world: &mut SubWorld, command_buffer: &mut CommandBuffer, relationship_map: &mut RelationshipMap) {
    if let Some((current_implementation, relationship)) = get_current_root_source_implementation(world) {
        remove_source_implementations_children_from_world(command_buffer, relationship_map, relationship, current_implementation);
    } else {
        todo!() //error?
    }
}

fn get_initially_read_root_source(world: &mut SubWorld) -> Option<Entity> {
    <Entity>
        ::query()
        .filter(component::<SourceFileRoot>() & component::<SourceFileInitialRead>())
        .iter(world)
        .map(|entity|*entity)
        .next()
}

fn get_current_root_source_implementation(world: &mut SubWorld) -> Option<(Entity, Relationship)> {
    <(Entity, &Relationship)>
        ::query()
        .filter(component::<Root>())
        .iter(world)
        .map(|(entity, relationship)|(*entity, *relationship))
        .next()
}

fn add_enity_with_source_implementation(command_buffer: &mut CommandBuffer, source: Entity) {
    command_buffer.push((source_implementation(source), Rebuild::default()));
}

fn source_implementation(root_source: Entity) -> SourceImplementation {
    SourceImplementation::from_source_entity(root_source)
}

fn source_implementation_has_changed(source_impl: &SourceImplementation, changed_sources: &HashMap<Entity, Entity>) -> bool {
    changed_sources.get(&source_impl.source_file_entity) != None
}

fn remove_source_implementations_children_from_world(
    command_buffer: &mut CommandBuffer,
    relationship_map: &mut RelationshipMap,
    relationship: Relationship,
    source_impl_entity: Entity) {

    debug!("Removing Source from world");    
    remove_children(command_buffer, relationship_map, source_impl_entity);
    reset_relationship(command_buffer, relationship, relationship_map, source_impl_entity);
    remove_mapped(command_buffer, source_impl_entity);
    remove_build_error(command_buffer, source_impl_entity);
    add_rebuild(command_buffer, source_impl_entity);
}


fn remove_children(
    command_buffer: &mut CommandBuffer,
    relationship_map: &RelationshipMap,
    entity: Entity) {

    for child in relationship_map.get_children(&entity) {
        debug!("marking child as removed: {:?}", child);
        command_buffer.add_component(child, Removed::default());
        remove_children(command_buffer, relationship_map, child);
    }
}

fn reset_relationship(
    command_buffer: &mut CommandBuffer, 
    relationship: Relationship, 
    relationship_map: &mut RelationshipMap, 
    entity: Entity) {

    let relationship = relationship.without_children();
    command_buffer.add_component(entity, relationship);
    relationship_map.insert(entity, relationship);
}

fn remove_mapped(command_buffer: &mut CommandBuffer, entity: Entity) {
    command_buffer.remove_component::<Mapped>(entity);
}

fn remove_build_error(command_buffer: &mut CommandBuffer, entity: Entity) {
    command_buffer.remove_component::<BuildErrorOccurrence>(entity);
}

fn add_rebuild(command_buffer: &mut CommandBuffer, entity: Entity) {
    command_buffer.add_component(entity, Rebuild::default());
}
