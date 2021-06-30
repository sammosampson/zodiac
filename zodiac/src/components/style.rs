use legion::*;
use serde::*;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct StyleRelationship(Entity);

impl From<Entity> for StyleRelationship {
    fn from(entity: Entity) -> Self {
        Self(entity)
    }
}

impl Into<Entity> for &StyleRelationship {
    fn into(self) -> Entity {
        self.0
    }
}
