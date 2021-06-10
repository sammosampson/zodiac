mod components;
mod relationships;
mod systems;
mod events;
mod formatting;
mod initialisation;

pub use formatting::*;
pub use components::*;
pub use relationships::*;
pub use systems::mapping::*;
pub use systems::relationships::*;
pub use systems::remove_entities::*;
pub use systems::world_vision::*;
pub use events::*;
pub use initialisation::*;
pub use formatting::WorldSerializer;

use legion::Entity;
use std::collections::HashMap;

pub type EntityMap = HashMap<u64, Entity>;
