use crate::prelude::*;
mod camera;
mod floor;
mod organism;
mod ui;

use bevy::window::WindowResolution;
pub use camera::*;
pub use ui::*;

pub struct VisualizationPlugin;
impl Plugin for VisualizationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK));
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1500.0, 900.0),
                title: "Exelixi".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin);
        app.add_plugins(EguiPlugin)
            .add_plugins(ui::UiPlugin {})
            .add_plugins(camera::CameraPlugin {})
            .add_systems(Update, organism::organism_transform_update)
            .add_systems(Update, floor::show_floor)
            .add_systems(PostUpdate, organism::show_organism)
            .add_systems(Update, organism::sprite_lightness_from_body);
    }
}
