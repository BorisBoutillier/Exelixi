use std::collections::BTreeMap;

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
    pub simulation_population_checked: BTreeMap<SpeciesId, bool>,
    pub simulation_energy_checked: BTreeMap<SpeciesId, bool>,
    pub selection_open: bool,
}
//
pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiState::default())
            .add_plugins(eye_viewer::EyeViewerPlugin)
            .add_plugins(mouth_viewer::MouthViewerPlugin)
            .add_systems(
                Update,
                (ui_status_bar, ui_simulation, ui_selection, user_selection),
            )
            .add_systems(PostUpdate, selection_changed);
        app.add_plugins(InputManagerPlugin::<UiAction>::default())
            .add_systems(Startup, setup_ui_action)
            .add_systems(Update, ui_action_input);
    }
}
