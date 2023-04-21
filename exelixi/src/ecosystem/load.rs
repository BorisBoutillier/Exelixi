use bevy::{ecs::entity::EntityMap, scene::serde::SceneDeserializer};
use serde::de::DeserializeSeed;

use crate::ecosystem::*;

pub struct LoadEcosystemEvent {
    pub path: String,
}

pub fn load_ecosystem_from_file(world: &mut World) {
    let load_events = world.get_resource::<Events<LoadEcosystemEvent>>().unwrap();
    let mut data = None;
    for event in load_events.get_reader().iter(load_events) {
        println!("LOADING {}", event.path);
        data = Some(std::fs::read_to_string(&event.path).unwrap());
    }
    if let Some(data) = data {
        let split = data.split(SAVE_SEP).collect::<Vec<_>>();
        if split.len() != 3 {
            println!("Invalid content");
            return;
        }
        let entities_ser = split[0];
        let config_ser = split[1];
        let runtime_ser = split[2];
        match ron::from_str::<EcosystemRuntime>(runtime_ser) {
            Ok(runtime) => {
                world.insert_resource(runtime);
            }
            Err(err) => {
                println!("EcosystemRuntime could not be loaded: {err}");
                return;
            }
        }
        match ron::from_str::<EcosystemConfig>(config_ser) {
            Ok(mut config) => {
                config.update_after_load();
                world.insert_resource(config);
            }
            Err(err) => {
                println!("EcosystemConfig could not be loaded: {err}");
                return;
            }
        }
        let type_registry = world.resource::<AppTypeRegistry>().0.clone();
        let mut deserializer = ron::de::Deserializer::from_str(entities_ser).unwrap();
        let scene_deserializer = SceneDeserializer {
            type_registry: &type_registry.read(),
        };
        let scene = scene_deserializer.deserialize(&mut deserializer).unwrap();
        let mut entity_map = EntityMap::default();
        scene.write_to_world(world, &mut entity_map).unwrap();
    }
}
