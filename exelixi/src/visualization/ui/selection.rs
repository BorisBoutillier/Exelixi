use crate::prelude::*;

const SELECTED_HUE: f32 = 180.0;
#[derive(Component)]
pub struct Selected;

pub fn selection_changed(
    mut deselected: RemovedComponents<Selected>,
    selected: Query<Entity, Added<Selected>>,
    mut organisms_sprite: Query<(&mut Sprite, &Organism)>,
) {
    for entity in deselected.iter() {
        if let Ok((mut sprite, organism)) = organisms_sprite.get_mut(entity) {
            let [_h, s, l, a] = sprite.color.as_hsla_f32();
            sprite.color = Color::hsla(organism.hue(), s, l, a);
        }
    }
    for entity in selected.iter() {
        if let Ok((mut sprite, _)) = organisms_sprite.get_mut(entity) {
            let [_h, s, l, a] = sprite.color.as_hsla_f32();
            sprite.color = Color::hsla(SELECTED_HUE, s, l, a);
        }
    }
}
