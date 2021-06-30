use legion::*;
use serde::*;

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Relationship {
    pub parent: Option<Entity>,
    pub next_sibling: Option<Entity>,
    pub previous_sibling: Option<Entity>,
    pub first_child: Option<Entity>,
    pub last_child: Option<Entity>
}


impl Relationship {
    pub fn for_parent_only(parent: Entity) -> Self {
        Self {
            parent: Some(parent),
            next_sibling: None,
            previous_sibling: None,
            first_child: None,
            last_child: None 
        }
    }
    
    pub fn without_children(&self) -> Self {
        Self {
            parent: self.parent,
            next_sibling: self.next_sibling,
            previous_sibling: self.previous_sibling,
            first_child: None,
            last_child: None 
        }
    }
}
