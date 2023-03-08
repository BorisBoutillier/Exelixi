use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    render::camera::ScalingMode,
    window::{PrimaryWindow, WindowResized},
};

const ZOOM_FACTOR: f32 = 1.1;

use crate::prelude::*;

#[derive(Component)]
pub struct MainCamera;

pub struct CameraPlugin {}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera);
        app.add_system(camera_movement);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(MainCamera);
}

#[allow(clippy::too_many_arguments)]
fn camera_movement(
    mut cameras: Query<&mut OrthographicProjection, With<MainCamera>>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    config: Res<SimulationConfig>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
    mut window_resized_events: EventReader<WindowResized>,
    ui_state: Res<UiState>,
) {
    let window_entity = primary_window.get_single().expect("Missing Primary window");
    let window_resized = window_resized_events
        .iter()
        .any(|e| e.window == window_entity);
    // Camera reset trigger on simulation config changed or Middle mouse button click
    let reset_camera = config.is_changed()
        || mouse_button_input.just_pressed(MouseButton::Middle)
        || window_resized;
    if reset_camera {
        let mut camera_ortho = cameras.get_single_mut().expect("No ortho camera found.");
        camera_ortho.viewport_origin = Vec2::new(0.5, 0.5);
        camera_ortho.scale = 1.1;
        camera_ortho.scaling_mode = ScalingMode::AutoMin {
            min_width: config.environment.width as f32,
            min_height: config.environment.height as f32,
        };
    }

    // Camera zooming triggered by mouse wheel up/down
    let mut zoom_update = 0.0;
    for event in mouse_wheel_events.iter() {
        zoom_update = event.y;
    }
    if zoom_update != 0.0 {
        let mut camera_ortho = cameras.get_single_mut().expect("No ortho camera found");
        camera_ortho.scale /= ZOOM_FACTOR.powf(zoom_update);
    }

    // Camera panning done pressing mouse button two and moving around.
    if mouse_button_input.pressed(MouseButton::Right) {
        for event in mouse_motion_events.iter() {
            let mut camera_ortho = cameras.get_single_mut().expect("No ortho camera found");
            camera_ortho.viewport_origin.x -= event.delta.x / 200.0;
            camera_ortho.viewport_origin.y -= event.delta.y / 200.0;
        }
    }
}
