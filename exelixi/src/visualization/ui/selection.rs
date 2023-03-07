use crate::prelude::*;

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
