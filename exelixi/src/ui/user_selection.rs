use crate::prelude::*;

pub fn user_selection(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    // query to get camera transform
    cameras: Query<(&VisibleArea, &OrthographicProjection), With<MainCamera>>,
    animals: Query<(Entity, &Transform), With<Animal>>,
    selected: Query<Entity, With<Selected>>,
) {
    // Detect mouse click
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let window = windows.get_primary().unwrap();
        let (visible_area, ortho) = cameras.single();
        if let Some(pos) = window.cursor_position() {
            // Check User has click inside the visible area
            if (visible_area.0.left..visible_area.0.right).contains(&pos.x)
                && (visible_area.0.bottom..visible_area.0.top).contains(&pos.y)
            {
                // Convert click to world position
                let world_pos = Vec2::new(
                    (ortho.left + (pos.x / window.width()) * (ortho.right - ortho.left))
                        * ortho.scale,
                    (ortho.bottom + (pos.y / window.height()) * (ortho.top - ortho.bottom))
                        * ortho.scale,
                );

                // Find closest animal
                let mut animal_dists = animals
                    .iter()
                    .map(|(e, t)| (e, (t.translation.truncate() - world_pos).length()))
                    .collect::<Vec<_>>();
                animal_dists.sort_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap());
                if let Some((closest_entity, _)) = animal_dists.first() {
                    // Deselect currently selected animals
                    for selected_entity in selected.iter() {
                        commands.entity(selected_entity).remove::<Selected>();
                    }
                    // Select the closet animal
                    commands.entity(*closest_entity).insert(Selected);
                }
            }
        }
    }
}
