use serde::{Deserialize, Serialize};

use super::SimulationConfig;

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
