use legion::*;
use legion::serialize::*;
use legion::storage::Component;
use log::{info};
use serde::Serialize;
use serde_json::*;

pub fn create_world_serializer() -> WorldSerializer {
    WorldSerializer::new()
}

#[derive(Default)]
pub struct WorldSerializer(Registry::<String>);

impl WorldSerializer {
    pub fn new() -> Self {
        let mut inner = Registry::<String>::default();
        inner.on_unknown(UnknownType::Ignore);
        
        Self(inner)
    }
    
    pub fn register_component<C: Component + Serialize + for<'de> serde::Deserialize<'de>>(&mut self, mapped_type_id: &'static str) {
        self.0.register::<C>(mapped_type_id.to_string());
    }

    pub fn serialize_world(&mut self, world: &World) -> Result<Value> {
        serde_json::to_value(world.as_serializable(passthrough(), &self.0))
    }

    pub fn log_world(&mut self, world: &World) {
        info!("{:#}", self.serialize_world(world).unwrap());
    }
}