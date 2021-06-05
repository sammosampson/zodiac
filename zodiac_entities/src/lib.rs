pub mod components;
pub mod relationships;
pub mod systems;
pub mod events;
pub mod initialisation;

pub use components::*;
use legion::Entity;
pub use relationships::*;
pub use systems::mapping::*;
pub use systems::relationships::*;
pub use systems::remove_entities::*;
pub use events::*;
pub use initialisation::*;

use std::collections::HashMap;

pub type EntityMap = HashMap<u64, Entity>;
