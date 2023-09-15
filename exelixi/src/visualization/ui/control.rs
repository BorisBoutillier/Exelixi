use bevy::{app::AppExit, reflect::TypePath};

use crate::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, TypePath)]
pub enum UiAction {
    OpenCloseSimulationPanel,
    OpenCloseSelectionPanel,
    Exit,
}

pub fn setup_ui_action(mut commands: Commands) {
    let input_map = InputMap::<UiAction>::new([
        (KeyCode::Key1, UiAction::OpenCloseSimulationPanel),
        (KeyCode::Key2, UiAction::OpenCloseSelectionPanel),
        (KeyCode::Escape, UiAction::Exit),
    ]);
    commands.insert_resource(input_map);
    commands.insert_resource(ActionState::<UiAction>::default());
}

pub fn ui_action_input(
    action_state: Res<ActionState<UiAction>>,
    mut ui_state: ResMut<UiState>,
    mut exit_event: EventWriter<AppExit>,
) {
    if action_state.just_pressed(UiAction::OpenCloseSimulationPanel) {
        ui_state.simulation_open = !ui_state.simulation_open;
    }
    if action_state.just_pressed(UiAction::OpenCloseSelectionPanel) {
        ui_state.selection_open = !ui_state.selection_open;
    }
    if action_state.just_pressed(UiAction::Exit) {
        exit_event.send(AppExit);
    }
}
