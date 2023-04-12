use crate::prelude::*;

// Resources
#[derive(Resource)]
pub struct Simulation {
    pub control: SimulationControl,
    pub steps: u32,
    // Total active running of the simulation
    pub run_for: Option<u32>,
}
impl Simulation {
    pub fn new(run_for: Option<u32>) -> Self {
        let start_state = if run_for.is_some() {
            SimulationControlState::Fastest
        } else {
            SimulationControlState::Paused
        };
        Self {
            control: SimulationControl::new(start_state),
            steps: 0,
            run_for,
        }
    }
    // Dump current simulation information in a single line string.
}
