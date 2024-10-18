use bevy::scene::serde::SceneDeserializer;
use serde::de::DeserializeSeed;

use crate::ecosystem::*;

#[derive(Event)]
pub struct LoadEcosystemEvent {
    pub path: String,
}

pub fn load_ecosystem_from_file(world: &mut World) {
    let load_events = world.get_resource::<Events<LoadEcosystemEvent>>().unwrap();
    let mut data = None;
    for event in load_events.get_reader().read(load_events) {
        println!("LOADING {}", event.path);
        data = Some(std::fs::read_to_string(&event.path).unwrap());
    }
    if let Some(data) = data {
        let type_registry = world.resource::<AppTypeRegistry>().0.clone();
        let mut deserializer = ron::de::Deserializer::from_str(&data).unwrap();
        let scene_deserializer = SceneDeserializer {
            type_registry: &type_registry.read(),
        };
        let scene = scene_deserializer.deserialize(&mut deserializer).unwrap();
        let mut entity_map = bevy::ecs::entity::EntityHashMap::default();
        scene.write_to_world(world, &mut entity_map).unwrap();
    }
}
