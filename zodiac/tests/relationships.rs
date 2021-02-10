use legion::{World};
use rand::*;
use zodiac_entities::components::*;
use zodiac::relationships::*;

fn create_container_component() -> HorizontalLayoutContent {
    HorizontalLayoutContent {}
}

fn create_circle_component() -> Circle {
    Circle {}
}

fn create_text_component() -> Text {
    Text {}
}

fn create_position_component() -> Position {
    Position { x: random(), y: random() }
}

fn create_radius_component() -> Radius {
    Radius { radius: random() }
}

#[test]
fn sibling_relationship_iterator_iterates() {
    let mut world = World::default();
    
    let parent = create_container_component();
    let child1 = create_circle_component();
    let child1_component1 = create_position_component();
    let child1_component2 = create_radius_component();
    let child2 = create_text_component();
    let child2_component1 = create_position_component();

    let parent_relationship = Relationship { parent: None, next_sibling: None};
    let parent_entity = world.push((parent, parent_relationship));
    
    let child2_relationship = Relationship { parent: Some(parent_entity), next_sibling: None};
    let child2 = world.push((child2, child2_component1, child2_relationship));
    let child1_relationship = Relationship { parent: Some(parent_entity), next_sibling: Some(child2)};
    let child1 = world.push((child1, child1_component1, child1_component2, child1_relationship));

    let mut it = SiblingRelationshipIterator::new(&mut world, child1);
    assert_eq!(Some(child2), it.next());
    assert_eq!(None, it.next());
}


#[test]
fn sibling_relationship_iterator_returns_none_where_no_relationship() {
    let mut world = World::default();
    let child1 = world.push((create_circle_component(),));

    let mut it = SiblingRelationshipIterator::new(&mut world, child1);
    assert_eq!(None, it.next());
}