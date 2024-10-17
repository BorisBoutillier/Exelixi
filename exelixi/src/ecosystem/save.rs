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
        let mut scene = DynamicScene {
            entities: vec![],
            resources: vec![],
        };
        for entity in organisms.iter() {
            let entry = DynamicEntity {
                entity,
                components: vec![
                    get_reflect_data::<Position>(entity, world, &type_registry),
                    get_reflect_data::<Organism>(entity, world, &type_registry),
                    get_reflect_data::<Body>(entity, world, &type_registry),
                    get_reflect_data::<Brain>(entity, world, &type_registry),
                    get_reflect_data::<Mouth>(entity, world, &type_registry),
                    get_reflect_data::<Leaf>(entity, world, &type_registry),
                    get_reflect_data::<Locomotion>(entity, world, &type_registry),
                    get_reflect_data::<Eye>(entity, world, &type_registry),
                ]
                .into_iter()
                .flatten()
                .collect(),
            };

            scene.entities.push(entry);
        }
        let entities_ser = scene.serialize(&type_registry).expect("OHoh");

        // Serialize Resource manually, as they are not yet part of the DynamicScene
        let config = world.get_resource::<EcosystemConfig>().unwrap();
        let config_ser = ron::to_string(config).unwrap();
        let ecosystem = world.get_resource::<EcosystemRuntime>().unwrap();
        let ecosystem_ser = ron::to_string(ecosystem).unwrap();

        // Manually separate in file
        let data = [entities_ser, config_ser, ecosystem_ser].join(SAVE_SEP);
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

fn get_reflect_data<T: Component>(
    entity: Entity,
    world: &World,
    registry: &RwLockReadGuard<'_, TypeRegistry>,
) -> Option<Box<dyn Reflect>> {
    world
        .component_id::<T>()
        .and_then(|component_id| world.components().get_info(component_id))
        .and_then(|info| registry.get(info.type_id().unwrap()))
        .and_then(|registration| registration.data::<ReflectComponent>())
        .and_then(|reflect_component| reflect_component.reflect(world.entity(entity)))
        .map(|reflect_component| reflect_component.clone_value())
}
