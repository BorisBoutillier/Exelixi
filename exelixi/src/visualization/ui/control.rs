use bevy::app::AppExit;

use crate::prelude::*;

#[derive(Actionlike, Reflect, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum UiAction {
    OpenCloseSimulationPanel,
    OpenCloseSelectionPanel,
    Exit,
}

pub fn setup_ui_action(mut commands: Commands) {
    let input_map = InputMap::<UiAction>::new([
        (UiAction::OpenCloseSimulationPanel, KeyCode::Digit1),
        (UiAction::OpenCloseSelectionPanel, KeyCode::Digit2),
        (UiAction::Exit, KeyCode::Escape),
    ]);
    commands.insert_resource(input_map);
    commands.insert_resource(ActionState::<UiAction>::default());
}

pub fn ui_action_input(
    action_state: Res<ActionState<UiAction>>,
    mut ui_state: ResMut<UiState>,
    mut exit_event: EventWriter<AppExit>,
) {
    if action_state.just_pressed(&UiAction::OpenCloseSimulationPanel) {
        ui_state.simulation_open = !ui_state.simulation_open;
    }
    if action_state.just_pressed(&UiAction::OpenCloseSelectionPanel) {
        ui_state.selection_open = !ui_state.selection_open;
    }
    if action_state.just_pressed(&UiAction::Exit) {
        exit_event.send(AppExit::Success);
    }
}
