use bevy::{app::AppExit, reflect::TypeRegistry, scene::DynamicEntity};
use std::sync::RwLockReadGuard;

use crate::ecosystem::*;
pub const SAVE_SEP: &str = "\n########\n";

#[derive(Event)]
pub struct SaveEcosystemEvent {
    pub path: PathBuf,
    pub then_exit: bool,
}

//pub fn save_ecosystem_to_file() -> impl IntoSystemConfigs {
//    save_to_file.pipe(then_exit)
//}
pub fn save_to_file(
    world: &World,
    mut save_events: EventReader<SaveEcosystemEvent>,
    organisms: Query<Entity, With<Organism>>,
    registry: Res<AppTypeRegistry>,
) -> bool {
    save_events.read().any(|event| {
        // Save Entities, using Bevy Dynamic Scene
        let type_registry = registry.read();
        let mut scene = DynamicSceneBuilder::from_world(world)
            .deny_all()
            .allow::<Position>()
            .allow::<Organism>()
            .allow::<Body>()
            .allow::<Brain>()
            .allow::<Mouth>()
            .allow::<Leaf>()
            .allow::<Locomotion>()
            .allow::<Eye>()
            .deny_all_resources()
            //.allow_resource::<EcosystemConfig>()
            //.allow_resource::<EcosystemRuntime>()
            .extract_entities(organisms.iter())
            .extract_resources()
            .build();
        println!("Entities: {}", scene.entities.len());
        println!("Resources: {}", scene.resources.len());
        let world_ser = scene
            .serialize(&type_registry)
            .expect("Scene serialization failed.");

        // Serialize Resource manually, as they are not yet part of the DynamicScene
        let config = world.get_resource::<EcosystemConfig>().unwrap();
        let config_ser = ron::to_string(config).unwrap();
        let ecosystem = world.get_resource::<EcosystemRuntime>().unwrap();
        let ecosystem_ser = ron::to_string(ecosystem).unwrap();

        // Manually separate in file
        let data = [world_ser, config_ser, ecosystem_ser].join(SAVE_SEP);
        std::fs::write(&event.path, data.as_bytes()).expect("ohoh2");
        println!("Ecosystem has been saved to '{:?}'", event.path);

        event.then_exit
    })
}

pub fn then_exit(In(then_exit): In<bool>, mut exit_events: EventWriter<AppExit>) {
    if then_exit {
        exit_events.send_default();
    }
}
