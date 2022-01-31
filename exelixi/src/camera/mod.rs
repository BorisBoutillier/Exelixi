use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    render::camera::ScalingMode,
    window::WindowResized,
};

const ENV_VIEW_RESET_MARGIN_PCT: f32 = 1.1;
const ZOOM_FACTOR: f32 = 1.1;

use crate::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Default, Component)]
pub struct VisibleArea(pub Rect<f32>);
pub struct CameraPlugin {}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera);
        app.add_system(camera_movement);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera)
        .insert(VisibleArea::default());
}

#[allow(clippy::too_many_arguments)]
fn camera_movement(
    mut cameras: Query<(&mut VisibleArea, &mut OrthographicProjection), With<MainCamera>>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    config: Res<SimulationConfig>,
    windows: Res<Windows>,
    mut window_resized_events: EventReader<WindowResized>,
    ui_state: Res<UiState>,
) {
    let window = windows.get_primary().expect("No primary window found.");
    let window_resized = window_resized_events.iter().any(|e| e.id == window.id());
    // Camera reset trigger on simulation config changed or Middle mouse button click
    let reset_camera = config.is_changed()
        || mouse_button_input.just_pressed(MouseButton::Middle)
        || window_resized;
    if reset_camera {
        let (mut visible_area, mut camera_ortho) =
            cameras.get_single_mut().expect("No ortho camera found.");
        visible_area.0.left = if ui_state.stat_panel {
            UI_LEFT_PANEL_WIDTH
        } else {
            0.0
        };
        visible_area.0.right = window.width();
        visible_area.0.bottom = UI_STATUS_BAR_HEIGHT;
        visible_area.0.top = window.height();
        let view_width = visible_area.0.right - visible_area.0.left;
        let view_height = visible_area.0.top - visible_area.0.bottom;
        let view_ratio = view_width / view_height;
        let mut visible_width = config.environment.width * ENV_VIEW_RESET_MARGIN_PCT;
        let mut visible_height = config.environment.height * ENV_VIEW_RESET_MARGIN_PCT;
        let visible_ratio = visible_width / visible_height;
        if visible_ratio > view_ratio {
            visible_height = visible_width / view_ratio;
        } else {
            visible_width = visible_height * view_ratio;
        }
        camera_ortho.left = -visible_width / 2.0 - visible_area.0.left * visible_width / view_width;
        camera_ortho.right = visible_width / 2.0;
        camera_ortho.top = visible_height / 2.0;
        camera_ortho.bottom =
            -visible_height / 2.0 - visible_area.0.bottom * visible_height / view_height;
        camera_ortho.scale = 1.0;
        camera_ortho.scaling_mode = ScalingMode::None;
    }

    // Camera zooming triggered by mouse wheel up/down
    let mut zoom_update = 0.0;
    for event in mouse_wheel_events.iter() {
        zoom_update = event.y;
    }
    if zoom_update != 0.0 {
        let (_, mut camera_ortho) = cameras.get_single_mut().expect("No ortho camera found");
        camera_ortho.scale /= ZOOM_FACTOR.powf(zoom_update);
    }

    // Camera panning done pressing mouse button two and moving around.
    if mouse_button_input.pressed(MouseButton::Right) {
        for event in mouse_motion_events.iter() {
            let (_, mut camera_ortho) = cameras.get_single_mut().expect("No ortho camera found");
            camera_ortho.left -= event.delta.x;
            camera_ortho.right -= event.delta.x;
            camera_ortho.top += event.delta.y;
            camera_ortho.bottom += event.delta.y;
        }
    }
}
