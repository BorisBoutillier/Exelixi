use bevy::reflect::TypePath;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Copy, Clone)]
pub struct SimulationControl {
    pub state: SimulationControlState,
    // Speed_factor, a factor of 1 is 60 steps per seconds,
    pub speed_factor: u32,
}
impl SimulationControl {
    pub fn new() -> Self {
        Self {
            state: SimulationControlState::Paused,
            speed_factor: 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SimulationControlState {
    // Simulation is currently paused
    Paused,
    // Simulation is running live at a speed of speed_factor * 60 steps per seconds,
    Run,
    // Simulation is running at maximum speed, giving back control to GUI each generation,
    Fastest,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, TypePath)]
pub enum SimulationAction {
    PauseUnpause,
    Accelerate,
    Decelerate,
    Fastest,
    Save,
}

pub fn setup_simulation_speed_action(mut commands: Commands) {
    let input_map = InputMap::<SimulationAction>::new([
        (KeyCode::Up, SimulationAction::Fastest),
        (KeyCode::Right, SimulationAction::Accelerate),
        (KeyCode::Left, SimulationAction::Decelerate),
        (KeyCode::Space, SimulationAction::PauseUnpause),
        (KeyCode::S, SimulationAction::Save),
    ]);
    commands.insert_resource(input_map);
    commands.insert_resource(ActionState::<SimulationAction>::default());
}

pub fn simulation_action_input(
    action_state: Res<ActionState<SimulationAction>>,
    mut simulation: ResMut<Simulation>,
    mut save_events: EventWriter<SaveEcosystemEvent>,
) {
    if action_state.just_pressed(SimulationAction::PauseUnpause) {
        if simulation.control.state == SimulationControlState::Paused {
            simulation.control.state = SimulationControlState::Run;
            simulation.control.speed_factor = 1;
        } else {
            simulation.control.state = SimulationControlState::Paused;
        }
    }
    if action_state.just_pressed(SimulationAction::Fastest) {
        simulation.control.state = SimulationControlState::Fastest;
    }
    if action_state.just_pressed(SimulationAction::Accelerate) {
        simulation.control.state = SimulationControlState::Run;
        simulation.control.speed_factor *= 2;
    }
    if action_state.just_pressed(SimulationAction::Decelerate) {
        simulation.control.state = SimulationControlState::Run;
        simulation.control.speed_factor = (simulation.control.speed_factor / 2).max(1);
    }
    if action_state.just_pressed(SimulationAction::Save) {
        save_events.send(SaveEcosystemEvent {
            path: simulation
                .save_path
                .clone()
                .unwrap_or("default.ecosim".into()),
            then_exit: false,
        });
    }
}
