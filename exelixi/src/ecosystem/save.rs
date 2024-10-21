use crate::ecosystem::*;

pub fn save_ecosystem_to_file(path: &PathBuf, world: &mut World) {
    // Save Entities, using Bevy Dynamic Scene
    let scene = {
        let mut organisms = world.query_filtered::<Entity, With<Organism>>();
        DynamicSceneBuilder::from_world(world)
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
            .allow_resource::<EcosystemStatistics>()
            .extract_entities(organisms.iter(world))
            .extract_resources()
            .build()
    };
    info!(
        "Saved {} entities and {} resources",
        scene.entities.len(),
        scene.resources.len()
    );
    let type_registry = world.resource::<AppTypeRegistry>().read();
    let world_ser = scene
        .serialize(&type_registry)
        .expect("Scene serialization failed.");
    std::fs::write(path, world_ser.as_bytes()).expect("Failed to write to save path");
    info!("Ecosystem has been saved to '{:?}'", path);
}
