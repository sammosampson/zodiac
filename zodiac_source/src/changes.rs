use std::cell::RefCell;
use std::collections::HashSet;
use std::hash::Hash;
use std::io::SeekFrom;
use legion::systems::CommandBuffer;
use log::debug;
use zodiac_entities::*;
use std::fmt::Debug;

pub struct SourceBuildMaps<'a> {
    pub relationship_map: &'a mut RelationshipMap,
    pub entity_map: &'a mut EntityMap
}

pub trait SourceBuildChange: SourceBuildChangeClone + Debug + 'static {
    fn apply<'a>(&self, command_buffer: &mut CommandBuffer, maps: &mut SourceBuildMaps<'a>);
}


pub trait SourceBuildChangeClone {
    fn clone_box(&self) -> Box<dyn SourceBuildChange>;
}

pub trait SourceBuildChangesClone {
    fn clone_boxed_vec(&self) -> Vec<Box<dyn SourceBuildChange>>;
}

impl SourceBuildChangesClone for Vec<Box<dyn SourceBuildChange>> {
    fn clone_boxed_vec(&self) -> Vec<Box<dyn SourceBuildChange>> {
        self.iter().map(|change|change.clone_box()).collect()   
    }
}

impl<T> SourceBuildChangeClone for T where T: 'static + SourceBuildChange + Clone {
    fn clone_box(&self) -> Box<dyn SourceBuildChange> {
        Box::new(self.clone())
    }
}

#[derive(Default, Debug)]
pub struct SourceBuildChanges {
    inner: RefCell::<Vec::<Box::<dyn SourceBuildChange>>>
}

impl Clone for SourceBuildChanges {
    fn clone(&self) -> Self {
        Self {
            inner: RefCell::new(self.inner.borrow_mut().clone_boxed_vec())
        }
    }
}

impl SourceBuildChanges {
    fn push(&self, change: impl SourceBuildChange) {
        self.inner.borrow_mut().push(Box::new(change));
    }

    pub fn has_changed(&self) -> bool {
        self.inner.borrow().len() > 0
    }

    fn commit(&self) {
        self.inner.borrow_mut().clear();
    }

    pub fn apply<'a>(&self, command_buffer: &mut CommandBuffer, maps: &mut SourceBuildMaps<'a>) {
        for change in self.inner.borrow().iter() {
            change.apply(command_buffer, maps);
        }
    }
}

#[derive(Default, Debug)]
pub struct SourceBuildChangeState {
    changes: SourceBuildChanges
}

impl SourceBuildChangeState {
    pub fn push_change(&self, change: impl SourceBuildChange) {
        println!("pushing change {:?}", change);
        self.changes.push(change);        
    }

    pub fn commit(&self) -> SourceBuildChanges {
        let changes = self.changes.clone();
        self.changes.commit();
        changes
    }

    pub fn has_changed(&self) -> bool {
        self.changes.has_changed()
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NodeChanges<T> where T: Eq + PartialEq + Hash {
    added: Vec<T>,
    removed: Vec<T>
}

impl<T> NodeChanges<T> where T: Eq + PartialEq + Hash + Clone {
    pub fn between(current: &Vec<T>, previous: &Vec<T>) -> NodeChanges<T> {
        let current: HashSet<&T> = current.iter().collect();
        let previous: HashSet<&T> = previous.iter().collect();
        let additions = current.difference(&previous);
        let deletions = previous.difference(&current);
        let added: Vec<T> = additions.map(|node| (**node).clone()).collect(); 
        let removed: Vec<T> = deletions.map(|node| (**node).clone()).collect();
        
        NodeChanges { 
            added,
            removed
        }
    }

    pub fn process_additions(&self, processor: &mut impl FnMut(&T) -> ()) {
        for addition in &self.added {
            processor(addition);
        }    
    }

    pub fn process_removals(&self, processor: &mut impl FnMut(&T) -> ()) {
        for removal in &self.removed {
            processor(removal);
        }    
    }
}