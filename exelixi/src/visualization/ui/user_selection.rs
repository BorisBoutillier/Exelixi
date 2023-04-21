use bevy::window::PrimaryWindow;

use crate::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn user_selection(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    kdtree: Res<OrganismKdTree>,
    selected: Query<Entity, With<Selected>>,
    config: Res<EcosystemConfig>,
    simulation: Res<Simulation>,
    mut egui_contexts: EguiContexts,
    mut simulation_action: ResMut<ActionState<SimulationAction>>,
) {
    // Detect mouse click
    // If the simulation is Fastest , this pauses the simulation
    // Otherwise it selects the closest organism
    if mouse_button_input.just_pressed(MouseButton::Left)
        && !egui_contexts.ctx_mut().wants_pointer_input()
    {
        if simulation.control.state == SimulationControlState::Fastest {
            simulation_action.press(SimulationAction::PauseUnpause);
        } else {
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
                    let position = Position::new(world_pos.x, world_pos.y, 0.0);
                    let mut nearest = kdtree
                        .per_species
                        .iter()
                        .filter_map(|(_, tree)| {
                            tree.nearest(&KdTreeEntry::new(&position, Entity::PLACEHOLDER))
                        })
                        .collect::<Vec<_>>();
                    nearest.sort_by(|v1, v2| {
                        v1.squared_distance
                            .partial_cmp(&v2.squared_distance)
                            .unwrap()
                    });
                    if let Some(v) = nearest.first() {
                        let nearest_entity = v.item.entity;
                        // Deselect currently selected organisms
                        for selected_entity in selected.iter() {
                            commands.entity(selected_entity).remove::<Selected>();
                        }
                        // Select the closet organism
                        commands.entity(nearest_entity).insert(Selected);
                    }
                }
            }
        }
    }
}
