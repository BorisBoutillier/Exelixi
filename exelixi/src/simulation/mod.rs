use std::time::Duration;

mod config;
mod control;
use crate::prelude::*;
pub use config::*;
pub use control::*;
use ga::PopulationStatistics;

#[derive(Default)]
pub struct SimulationStatistics {
    pub population: Vec<PopulationStatistics>,
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
    pub age: u32,
    pub generation: u32,
    pub ga: ga::GeneticAlgorithm<ga::RouletteWheelSelection>,
    pub statistics: SimulationStatistics,
    // Total active running of the simulation
    pub duration: Duration,
}
impl Simulation {
    pub fn new() -> Self {
        Self {
            control: SimulationControl::default(),
            age: 0,
            generation: 0,
            ga: ga::GeneticAlgorithm::new(
                ga::RouletteWheelSelection::default(),
                ga::UniformCrossover::default(),
                ga::GaussianMutation::new(0.01, 0.3),
            ),
            statistics: SimulationStatistics::default(),
            duration: Duration::ZERO,
        }
    }
    // Number simulation steps per seconds for this simulation
    pub fn sts(&self, config: &SimulationConfig) -> f32 {
        if self.duration.is_zero() {
            0.0
        } else {
            (self.generation * config.generation_length + self.age) as f32
                / self.duration.as_secs_f32()
        }
    }
    // Dump current simulation information in a single line string.
    pub fn sprint_state(&self, config: &SimulationConfig) -> String {
        let size = self.statistics.latest_size();
        let dead = self.statistics.latest_dead();
        format!(
            "Gen: {:03} , Sts: {:.2} , Avg: {:.1} , Pop: {}/{}",
            self.generation,
            self.sts(config),
            self.statistics.latest_avg_fitness(),
            dead,
            size,
        )
    }
}
impl Default for Simulation {
    fn default() -> Self {
        Self::new()
    }
}
