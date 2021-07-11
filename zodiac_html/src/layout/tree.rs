use std::cell::RefCell;
use std::collections::HashMap;
use legion::systems::CommandBuffer;
use serde::*;
use legion::*;
use legion::world::*;
use zodiac::*;
use super::*;

pub fn layout_tree<'a>(world: &mut SubWorld, relationship_map: &'a RelationshipMap) -> LayoutTree<'a> {
    LayoutTree::<'a>::new(world, relationship_map)
}

#[derive(Debug)]
pub struct LayoutTree<'a>(
    &'a RelationshipMap,
    HashMap::<Entity, RefCell<LayoutNode>>
);

impl<'a> LayoutTree<'a> {
    fn new(world: &mut SubWorld, relationship_map: &'a RelationshipMap) -> Self {

        let layout_nodes: HashMap::<Entity, RefCell<LayoutNode>> = <(Entity, &LayoutBox, &ResolvedLayoutBox, Option<&LayoutRequest>)>::query()
            .iter(world)
            .map(|(entity, layout_box, resolved_layout_box, request)| 
                (*entity, RefCell::new(LayoutNode::new(*entity, *layout_box, *resolved_layout_box, request.into()))))
            .collect();

        Self(relationship_map, layout_nodes)
    }

    pub fn layout(&self, root: &Entity) {
        let mut root_node = self.1.get(root).unwrap().borrow_mut();
        root_node.layout(None, self);
    }  

    pub fn position(&self, root: &Entity, command_buffer: &mut CommandBuffer) {
        let mut root_node = self.1.get(root).unwrap().borrow_mut();
        root_node.position(None, self, command_buffer);
    }

    fn get(&self, entity: &Entity) -> Option<&RefCell<LayoutNode>> {
        self.1.get(entity)
    }
    
    fn get_previous_sibling(&self, entity: &Entity) -> Option<&RefCell<LayoutNode>> {
        if let Some(parent)= self.relationship_map().get_previous_sibling(&entity) {
            return self.1.get(&parent)
        }
        None
    }

    fn get_children(&'a self, entity: Entity) -> LayoutTreeChildrenIterator<'a> {
        LayoutTreeChildrenIterator::<'a>::new( &self, entity)
    }

    fn relationship_map(&self) -> &'a RelationshipMap {
        self.0
    }
}

pub struct LayoutTreeChildrenIterator<'a> {
    children: ChildrenRelationshipIterator<'a>,
    tree: &'a LayoutTree<'a>
}

impl<'a> LayoutTreeChildrenIterator<'a> {
    fn new(tree: &'a LayoutTree, parent: Entity) -> Self {
        Self {
            children: tree.relationship_map().get_children(&parent), 
            tree
        }
    }
}

impl <'a> Iterator for LayoutTreeChildrenIterator<'a> {
    type Item = (Entity, &'a RefCell<LayoutNode>);
    fn next(&mut self) -> Option<(Entity, &'a RefCell<LayoutNode>)> {
        loop {
            if let Some(child) = self.children.next() {
                if let Some(layout_node) = self.tree.get(&child) {
                    return Some((child, layout_node));
                }
            } else {
                return None;
            }
        }
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum LayoutStatus {
    Requested,
    Resolved,
    Resolving,
    Positioning,
    Positioned
}

impl Into<LayoutStatus> for Option<&LayoutRequest> {
    fn into(self) -> LayoutStatus {
        match self {
            None => LayoutStatus::Resolved,
            _ => LayoutStatus::Requested
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct LayoutNode {
    entity: Entity,
    layout_box: LayoutBox,
    resolved_layout_box: ResolvedLayoutBox, 
    status: LayoutStatus
}

impl LayoutNode {
    fn new(
        entity: Entity, 
        layout_box: LayoutBox, 
        resolved_layout_box: ResolvedLayoutBox, 
        status: LayoutStatus) -> Self {
        Self {
            entity,
            layout_box,
            resolved_layout_box,
            status
        }
    }

    fn layout<'a>(&mut self, parent_layout: Option<&LayoutNode>, tree: &LayoutTree<'a>) {
        if self.status == LayoutStatus::Requested {
            self.resolve_layout(parent_layout, tree);
            return;
        }

        for (_child, child_layout) in tree.get_children(self.entity) {
            child_layout.borrow_mut().layout(Some(self), tree);
        }
    }

    fn resolve_layout<'a>(&mut self, parent_layout: Option<&LayoutNode>, tree: &LayoutTree<'a>) {
        self.status = LayoutStatus::Resolving;   
        self.resolved_layout_box = ResolvedLayoutBox::from(self.layout_box);

        if let Some(parent_layout) = parent_layout {
            self.resolved_layout_box.resolve_from_parent(&self.layout_box, &parent_layout.resolved_layout_box);
        }

        for (_child, child_layout) in tree.get_children(self.entity) {
            child_layout.borrow_mut().resolve_layout(Some(self), tree);
            self.resolved_layout_box.resolve_from_child(&self.layout_box, &child_layout.borrow().resolved_layout_box);
        }

        self.resolved_layout_box.complete_children_resolution(&self.layout_box);

        self.status = LayoutStatus::Resolved;   
    }

    fn position<'a>(&mut self, parent_layout: Option<&LayoutNode>, tree: &LayoutTree<'a>, command_buffer: &mut CommandBuffer) {
        if self.status == LayoutStatus::Resolved {
            self.resolve_position(parent_layout, tree, command_buffer);
            return;
        }

        for (_child, child_layout) in tree.get_children(self.entity) {
            child_layout.borrow_mut().position(Some(self), tree, command_buffer);
        }
    }

    fn resolve_position<'a>(&mut self, parent_layout: Option<&LayoutNode>, tree: &LayoutTree<'a>, command_buffer: &mut CommandBuffer) {
        self.status = LayoutStatus::Positioning;   

        if let Some(previous_sibling_layout) = tree.get_previous_sibling(&self.entity) {
            self.resolved_layout_box.position_from_sibling(&previous_sibling_layout.borrow().resolved_layout_box);
        } else if let Some(parent_layout) = parent_layout {
            self.resolved_layout_box.position_from_parent(&parent_layout.resolved_layout_box);
        } 

        for (_child, child_layout) in tree.get_children(self.entity) {
            child_layout.borrow_mut().resolve_position(Some(self), tree, command_buffer);
        }
        
        command_buffer.add_component(self.entity, self.resolved_layout_box);
        command_buffer.add_component(self.entity, LayoutChange::default());
        
        self.status = LayoutStatus::Positioned;   
    }
}