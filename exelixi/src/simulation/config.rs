use std::path::PathBuf;

use crate::prelude::*;

// Resources
#[derive(Resource, Default)]
pub struct Simulation {
    pub control: SimulationControl,
    pub load: Option<PathBuf>,
    // Total active running of the simulation
    pub run: Option<u32>,
    // Default path to save
    pub save: Option<PathBuf>,
    // Defines if the simulation should exit after doing all defined steps in load/run/save
    pub exit: bool,
}
