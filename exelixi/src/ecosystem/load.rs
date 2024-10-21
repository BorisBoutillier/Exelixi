use bevy::scene::serde::SceneDeserializer;
use serde::de::DeserializeSeed;

use crate::ecosystem::*;

pub fn load_ecosystem_from_file(path: &PathBuf, world: &mut World) {
    match std::fs::read_to_string(path) {
        Ok(data) => {
            let type_registry = world.resource::<AppTypeRegistry>().0.clone();
            let mut deserializer = ron::de::Deserializer::from_str(&data).unwrap();
            let scene_deserializer = SceneDeserializer {
                type_registry: &type_registry.read(),
            };
            let scene = scene_deserializer.deserialize(&mut deserializer).unwrap();
            let mut entity_map = bevy::ecs::entity::EntityHashMap::default();
            scene.write_to_world(world, &mut entity_map).unwrap();
            info!("Ecosystem loaded from '{path:?}'");
        }
        Err(err) => {
            error!("Ecosystem at path '{path:?}' could not be loaded: {err:?}");
        }
    }
}
