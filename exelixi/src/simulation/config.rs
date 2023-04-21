use std::path::PathBuf;

use crate::prelude::*;

// Resources
#[derive(Resource)]
pub struct Simulation {
    pub control: SimulationControl,
    // Total active running of the simulation
    pub run_for: Option<u32>,
    // Default path to save
    pub save_path: Option<PathBuf>,
}
impl Simulation {
    pub fn new(run_for: Option<u32>, save_path: Option<PathBuf>) -> Self {
        Self {
            control: SimulationControl::new(),
            run_for,
            save_path,
        }
    }
    // Dump current simulation information in a single line string.
}
