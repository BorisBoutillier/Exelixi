use crate::prelude::*;
mod camera;
mod floor;
mod organism;
mod ui;

pub use camera::*;
pub use ui::*;

pub struct VisualizationPlugin;
impl Plugin for VisualizationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_plugin(ui::UiPlugin {})
            .add_plugin(camera::CameraPlugin {})
            .add_system(organism::organism_transform_update)
            .add_system(floor::show_floor)
            .add_system(organism::show_organism.in_base_set(CoreSet::PostUpdate));
    }
}
