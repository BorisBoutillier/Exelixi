use crate::prelude::*;

const SELECTED_HUE: f32 = 180.0;
#[derive(Component)]
pub struct Selected;

pub fn selection_changed(
    mut deselected: RemovedComponents<Selected>,
    selected: Query<Entity, Added<Selected>>,
    mut organisms_sprite: Query<(&mut Sprite, &Organism)>,
) {
    for entity in deselected.read() {
        if let Ok((mut sprite, organism)) = organisms_sprite.get_mut(entity) {
            sprite.color = sprite.color.with_hue(organism.hue());
        }
    }
    for entity in selected.iter() {
        if let Ok((mut sprite, _)) = organisms_sprite.get_mut(entity) {
            sprite.color = sprite.color.with_hue(SELECTED_HUE);
        }
    }
}
