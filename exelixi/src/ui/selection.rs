use crate::prelude::*;

pub fn selection_changed(
    deselected: RemovedComponents<Selected>,
    selected: Query<Entity, Added<Selected>>,
    mut animals_sprite: Query<&mut Sprite, With<Animal>>,
) {
    for entity in deselected.iter() {
        if let Ok(mut sprite) = animals_sprite.get_mut(entity) {
            sprite.color = Color::rgb(0.8, 0.3, 0.8)
        }
    }
    for entity in selected.iter() {
        if let Ok(mut sprite) = animals_sprite.get_mut(entity) {
            sprite.color = Color::rgb(0.2, 0.9, 0.9);
        }
    }
}
