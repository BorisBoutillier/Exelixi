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
        .add_plugin(FrameTimeDiagnosticsPlugin::default());
        app.add_plugin(EguiPlugin)
            .add_plugin(ui::UiPlugin {})
            .add_plugin(camera::CameraPlugin {})
            .add_system(organism::organism_transform_update)
            .add_system(floor::show_floor)
            .add_system(organism::show_organism.in_base_set(CoreSet::PostUpdate));
    }
}
