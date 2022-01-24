use std::time::Instant;

mod config;
mod control;
mod systems;

pub use config::*;
pub use control::*;
pub use systems::*;

use crate::prelude::*;
use ga::PopulationStatistics;

#[derive(Default)]
pub struct SimulationStatistics {
    pub population: Vec<PopulationStatistics>,
    pub food_decay: u32,
}

impl SimulationStatistics {
    pub fn latest_dead(&self) -> usize {
        let len = self.population.len();
        match len {
            0 => 0,
            n => self.population[n - 1].dead(),
        }
    }
    pub fn latest_size(&self) -> usize {
        let len = self.population.len();
        match len {
            0 => 0,
            n => self.population[n - 1].size(),
        }
    }
    pub fn latest_avg_fitness(&self) -> f32 {
        let len = self.population.len();
        match len {
            0 => 0.0,
            n => self.population[n - 1].avg_fitness(),
        }
    }
    pub fn latest_min_fitness(&self) -> f32 {
        let len = self.population.len();
        match len {
            0 => 0.0,
            n => self.population[n - 1].min_fitness(),
        }
    }
    pub fn latest_max_fitness(&self) -> f32 {
        let len = self.population.len();
        match len {
            0 => 0.0,
            n => self.population[n - 1].max_fitness(),
        }
    }
    pub fn update(&mut self, population_stat: PopulationStatistics) {
        self.population.push(population_stat);
    }
}
// Resources
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
        let size = self.statistics.latest_size();
        let dead = self.statistics.latest_dead();
        format!(
            "Gen: {:03} , Sts: {:.2} , Avg: {:.1} , Pop: {}/{} , Lost food: {}",
            self.generation,
            self.sts(config),
            self.statistics.latest_avg_fitness(),
            dead,
            size,
            self.statistics.food_decay
        )
    }

    // Triggers a new generation
    pub fn new_generation(&mut self) {
        self.generation += 1;
        self.statistics.food_decay = 0;
        self.generation_start_time = Instant::now();
    }
}
impl Default for Simulation {
    fn default() -> Self {
        Self::new()
    }
}
