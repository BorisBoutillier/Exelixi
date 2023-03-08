use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Copy, Clone)]
pub struct SimulationControl {
    pub state: SimulationControlState,
    // Speed_factor, a speed of 1 is 60 steps per seconds,
    pub speed_factor: u32,
}
impl SimulationControl {
    pub fn new(config: &SimulationConfig) -> Self {
        Self {
            state: if config.with_gui {
                SimulationControlState::Paused
            } else {
                SimulationControlState::Fastest
            },
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

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum SimulationSpeedAction {
    PauseUnpause,
    Accelerate,
    Decelerate,
    Fastest,
}

pub fn setup_simulation_speed_action(mut commands: Commands) {
    let input_map = InputMap::<SimulationSpeedAction>::new([
        (KeyCode::Up, SimulationSpeedAction::Fastest),
        (KeyCode::Right, SimulationSpeedAction::Accelerate),
        (KeyCode::Left, SimulationSpeedAction::Decelerate),
        (KeyCode::Space, SimulationSpeedAction::PauseUnpause),
    ]);
    commands.insert_resource(input_map);
    commands.insert_resource(ActionState::<SimulationSpeedAction>::default());
}

pub fn simulation_speed_action_input(
    action_state: Res<ActionState<SimulationSpeedAction>>,
    mut simulation: ResMut<Simulation>,
) {
    if action_state.just_pressed(SimulationSpeedAction::PauseUnpause) {
        if simulation.control.state == SimulationControlState::Paused {
            simulation.control.state = SimulationControlState::Run;
            simulation.control.speed_factor = 1;
        } else {
            simulation.control.state = SimulationControlState::Paused;
        }
    }
    if action_state.just_pressed(SimulationSpeedAction::Fastest) {
        simulation.control.state = SimulationControlState::Fastest;
    }
    if action_state.just_pressed(SimulationSpeedAction::Accelerate) {
        simulation.control.state = SimulationControlState::Run;
        simulation.control.speed_factor *= 2;
    }
    if action_state.just_pressed(SimulationSpeedAction::Decelerate) {
        simulation.control.state = SimulationControlState::Run;
        simulation.control.speed_factor = (simulation.control.speed_factor / 2).max(1);
    }
}
