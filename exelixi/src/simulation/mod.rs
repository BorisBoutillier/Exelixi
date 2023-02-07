use std::time::Instant;

mod config;
mod control;
mod stats;
mod systems;

pub use config::*;
pub use control::*;
pub use stats::*;
pub use systems::*;

use crate::prelude::*;
// Resources
#[derive(Resource)]
pub struct Simulation {
    pub control: SimulationControl,
    pub steps: u32,
    pub generation: u32,
    pub ga: ga::GeneticAlgorithm<ga::RouletteWheelSelection>,
    pub statistics: SimulationStatistics,
    // Total active running of the simulation
    pub generation_start_time: Instant,
}
impl Simulation {
    pub fn new() -> Self {
        Self {
            control: SimulationControl::default(),
            steps: 0,
            generation: 0,
            ga: ga::GeneticAlgorithm::new(
                ga::RouletteWheelSelection::default(),
                ga::UniformCrossover::default(),
                ga::GaussianMutation::new(0.01, 0.3),
            ),
            statistics: SimulationStatistics::default(),
            generation_start_time: Instant::now(),
        }
    }
    // Number simulation steps per seconds for this simulation
    pub fn sts(&self, config: &SimulationConfig) -> f32 {
        config.generation_length as f32
            / (Instant::now() - self.generation_start_time).as_secs_f32()
    }
    // Dump current simulation information in a single line string.
    pub fn sprint_state(&self, config: &SimulationConfig) -> String {
        format!(
            "Gen: {:03} , Sts: {:.2} , Avg: {:.1} , Pop: {}/{} , Lost food: {}",
            self.generation,
            self.sts(config),
            self.statistics.latest_avg_energy(),
            self.statistics.latest_end_size(),
            self.statistics.latest_start_size(),
            self.statistics.latest_food_decay(),
        )
    }

    // Triggers a new generation
    pub fn new_generation(&mut self) {
        self.generation += 1;
        self.generation_start_time = Instant::now();
    }
}
impl Default for Simulation {
    fn default() -> Self {
        Self::new()
    }
}
