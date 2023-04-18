use std::collections::HashMap;

use crate::prelude::*;

mod control;
mod eye_viewer;
mod mouth_viewer;
mod panels;
mod selection;
mod user_selection;

pub use control::*;
pub use panels::*;
pub use selection::*;
pub use user_selection::*;

pub const UI_STATUS_BAR_HEIGHT: f32 = 40.0;

#[derive(Resource, Default, Debug)]
pub struct UiState {
    pub simulation_open: bool,
    pub simulation_population_checked: HashMap<SpeciesId, bool>,
    pub simulation_energy_checked: HashMap<SpeciesId, bool>,
    pub selection_open: bool,
}
//
pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiState::default())
            .add_plugin(eye_viewer::EyeViewerPlugin)
            .add_plugin(mouth_viewer::MouthViewerPlugin)
            .add_system(ui_status_bar)
            .add_system(ui_simulation)
            .add_system(ui_selection)
            .add_system(user_selection)
            .add_system(selection_on_new_generation)
            .add_system(selection_changed.in_base_set(CoreSet::PostUpdate));
        app.add_plugin(InputManagerPlugin::<UiAction>::default())
            .add_startup_system(setup_ui_action)
            .add_system(ui_action_input);
    }
}
