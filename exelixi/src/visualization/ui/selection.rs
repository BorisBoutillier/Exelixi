use crate::prelude::*;

const SELECTED_HUE: f32 = 180.0;
#[derive(Component)]
pub struct Selected;

pub fn selection_changed(
    mut deselected: RemovedComponents<Selected>,
    config: Res<EcosystemConfig>,
    selected: Query<Entity, Added<Selected>>,
    mut organisms_sprite: Query<(&mut Sprite, &Organism)>,
) {
    for entity in deselected.iter() {
        if let Ok((mut sprite, organism)) = organisms_sprite.get_mut(entity) {
            let visualization_config = &config.organisms_per_name[&organism.name].visualization;
            let [_h, s, l, a] = sprite.color.as_hsla_f32();
            sprite.color = Color::hsla(visualization_config.hue, s, l, a);
        }
    }
    for entity in selected.iter() {
        if let Ok((mut sprite, _)) = organisms_sprite.get_mut(entity) {
            let [_h, s, l, a] = sprite.color.as_hsla_f32();
            sprite.color = Color::hsla(SELECTED_HUE, s, l, a);
        }
    }
}

pub fn selection_on_new_generation(
    mut commands: Commands,
    mut new_generation_event: EventReader<NewGenerationEvent>,
    organisms: Query<Entity, With<Organism>>,
) {
    for _event in new_generation_event.iter() {
        if let Some(one_entity) = organisms.iter().next() {
            commands.entity(one_entity).insert(Selected {});
        }
    }
}
