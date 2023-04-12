use crate::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum UiAction {
    OpenCloseSimulationPanel,
}

pub fn setup_ui_action(mut commands: Commands) {
    let input_map =
        InputMap::<UiAction>::new([(KeyCode::Key1, UiAction::OpenCloseSimulationPanel)]);
    commands.insert_resource(input_map);
    commands.insert_resource(ActionState::<UiAction>::default());
}

pub fn ui_action_input(action_state: Res<ActionState<UiAction>>, mut ui_state: ResMut<UiState>) {
    if action_state.just_pressed(UiAction::OpenCloseSimulationPanel) {
        ui_state.ui_simulation_open = !ui_state.ui_simulation_open;
        println!("NEW State {:?}", ui_state);
    }
}
