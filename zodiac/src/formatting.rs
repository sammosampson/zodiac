#[cfg(not(test))]
use log::{info};
#[cfg(test)]
use std::{println as info};

use legion::*;
use legion::serialize::*;
use legion::storage::Component;
use serde::Serialize;
use serde_json::*;

pub fn create_world_serializer() -> WorldSerializer {
    WorldSerializer::new()
}

#[derive(Default)]
pub struct WorldSerializer { 
    registry: Registry::<String>,
    last_result: Option<Value>
}

impl WorldSerializer {
    pub fn new() -> Self {
        let mut registry = Registry::<String>::default();
        registry.on_unknown(UnknownType::Ignore);
        
        Self {
            registry,
            last_result: None
        }
    }
    
    pub fn register_component<C: Component + Serialize + for<'de> serde::Deserialize<'de>>(&mut self, mapped_type_id: &'static str) {
        self.registry.register::<C>(mapped_type_id.to_string());
    }

    pub fn serialize_world(&mut self, world: &World) -> Result<Value> {
        serde_json::to_value(world.as_serializable(passthrough(), &self.registry))
    }

    pub fn log_world(&mut self, world: &World) {
        let result = self.serialize_world(world).unwrap();
        
        if self.last_result == None {
            info!("{:#}", result);
        } else if result != self.last_result.take().unwrap() {
            info!("{:#}", result);
        }

        self.last_result = Some(result);
    }
}