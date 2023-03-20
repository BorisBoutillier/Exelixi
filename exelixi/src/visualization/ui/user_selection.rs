use bevy::window::PrimaryWindow;

use crate::prelude::*;

pub fn user_selection(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    organisms: Query<(Entity, &Transform), With<Organism>>,
    selected: Query<Entity, With<Selected>>,
    config: Res<EcosystemConfig>,
) {
    // Detect mouse click
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let window = primary_window.get_single().expect("Missing primary window");
        let (camera, camera_global_transform) = cameras.single();
        if let Some(pos) = window.cursor_position() {
            let world_pos = camera
                .viewport_to_world_2d(camera_global_transform, pos)
                .unwrap();
            // Filter out clicks outside the environment
            if (-0.5..0.5).contains(&(world_pos.x / config.environment.width as f32))
                && (-0.5..0.5).contains(&(world_pos.y / config.environment.height as f32))
            {
                // Find closest organism
                let mut organism_dists = organisms
                    .iter()
                    .map(|(e, t)| (e, (t.translation.truncate() - world_pos).length()))
                    .collect::<Vec<_>>();
                organism_dists.sort_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap());
                if let Some((closest_entity, _)) = organism_dists.first() {
                    // Deselect currently selected organisms
                    for selected_entity in selected.iter() {
                        commands.entity(selected_entity).remove::<Selected>();
                    }
                    // Select the closet organism
                    commands.entity(*closest_entity).insert(Selected);
                }
            }
        }
    }
}
