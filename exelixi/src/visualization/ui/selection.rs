use crate::prelude::*;

#[derive(Component)]
pub struct Selected;

pub fn selection_changed(
    mut deselected: RemovedComponents<Selected>,
    selected: Query<Entity, Added<Selected>>,
    mut organisms_sprite: Query<&mut Sprite, With<Organism>>,
) {
    for entity in deselected.iter() {
        if let Ok(mut sprite) = organisms_sprite.get_mut(entity) {
            sprite.color = Color::rgb(0.8, 0.3, 0.8)
        }
    }
    for entity in selected.iter() {
        if let Ok(mut sprite) = organisms_sprite.get_mut(entity) {
            sprite.color = Color::rgb(0.2, 0.9, 0.9);
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
