mod components;
mod relationships;
mod systems;
mod events;
mod formatting;
mod initialisation;
mod application_state;
mod source;
mod rendering;
mod layout;
pub mod testing;

pub use application_state::*;
pub use source::*;
pub use formatting::*;
pub use components::*;
pub use relationships::*;
pub use systems::mapping::*;
pub use systems::events::*;
pub use systems::relationships::*;
pub use systems::remove_entities::*;
pub use systems::running::*;
pub use systems::world_vision::*;
pub use systems::cleanup::*;
pub use systems::measurement::*;
pub use systems::positioning::*;
pub use systems::resizing::*;
pub use rendering::*;
pub use events::*;
pub use initialisation::*;
pub use formatting::WorldSerializer;
pub use layout::*;

use legion::Entity;
use std::collections::HashMap;

pub type EntityMap = HashMap<u64, Entity>;
