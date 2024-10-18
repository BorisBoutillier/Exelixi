use bevy::app::AppExit;

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
        let scene = DynamicSceneBuilder::from_world(world)
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
            .allow_resource::<GlobalEntropy<WyRand>>()
            .allow_resource::<EcosystemConfig>()
            .allow_resource::<EcosystemRuntime>()
            .extract_entities(organisms.iter())
            .extract_resources()
            .build();
        info!(
            "Saved {} entities and {} resources",
            scene.entities.len(),
            scene.resources.len()
        );
        let world_ser = scene
            .serialize(&type_registry)
            .expect("Scene serialization failed.");
        std::fs::write(&event.path, world_ser.as_bytes()).expect("Failed to write to save path");
        println!("Ecosystem has been saved to '{:?}'", event.path);

        event.then_exit
    })
}

pub fn then_exit(In(then_exit): In<bool>, mut exit_events: EventWriter<AppExit>) {
    if then_exit {
        exit_events.send_default();
    }
}
