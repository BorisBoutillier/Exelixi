use std::time::Instant;

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
            generation: 0,
            ga: ga::GeneticAlgorithm::new(
                ga::RouletteWheelSelection::default(),
                ga::UniformCrossover::default(),
                ga::GaussianMutation::new(0.01, 0.3),
            ),
            statistics: SimulationStatistics::default(),
            generation_start_time: Instant::now(),
            run_for,
        }
    }
    // Dump current simulation information in a single line string.
    pub fn sprint_state(&self, _config: &EcosystemConfig) -> String {
        format!(
            "Gen: {:03} , Avg: {:.1} , Pop start: {}, Pop end: {} , Uneaten food: {}",
            self.generation,
            self.statistics.latest_avg_energy(),
            self.statistics.latest_start_size(),
            self.statistics.latest_end_size(),
            self.statistics.latest_food_decay(),
        )
    }

    // Triggers a new generation
    pub fn new_generation(&mut self) {
        self.generation += 1;
        self.generation_start_time = Instant::now();
    }
}
