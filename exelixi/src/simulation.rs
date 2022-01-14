use std::fmt;
use std::time::Duration;

use crate::prelude::*;
use ga::PopulationStatistics;

/// Number of simulation step we do per frame in Normal speed mode
pub const STEP_PER_FRAME_NORMAL: usize = 1;
/// Number of simulation step we do per frame in Fast speed mode
pub const STEP_PER_FRAME_FAST: usize = 4;
/// Maximum duration the simulation steps car run per frame
pub const MAX_SIMULATION_DURATION_PER_FRAME: f32 = 1.0 / 60.0;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SimulationSpeed {
    /// Simulation is paused, no steps are done
    Paused,
    /// Simulation speed of 60 step per seconds
    Normal,
    /// Simulation speed of 180 steps per seconds
    Fast,
    /// Fastest number of steps per seconds
    Fastest,
}
impl fmt::Display for SimulationSpeed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SimulationSpeed::Paused => "Paused",
                SimulationSpeed::Normal => "Normal",
                SimulationSpeed::Fast => "Fast",
                SimulationSpeed::Fastest => "Fastest",
            }
        )
    }
}
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
    pub speed: SimulationSpeed,
    pub age: u32,
    pub generation: u32,
    pub ga: ga::GeneticAlgorithm<ga::RouletteWheelSelection>,
    pub statistics: SimulationStatistics,
    // Total active running of the simulation
    pub duration: Duration,
    // Internal. Used to control simulation speed
    pub cur_steps: usize,
    pub cur_steps_duration: Duration,
}
impl Simulation {
    pub fn new() -> Self {
        Self {
            speed: SimulationSpeed::Paused,
            age: 0,
            generation: 0,
            ga: ga::GeneticAlgorithm::new(
                ga::RouletteWheelSelection::default(),
                ga::UniformCrossover::default(),
                ga::GaussianMutation::new(0.01, 0.3),
            ),
            statistics: SimulationStatistics::default(),
            duration: Duration::ZERO,
            cur_steps_duration: Duration::ZERO,
            cur_steps: 0,
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

pub struct EnvironmentConfig {
    // Size of floor
    pub size: Size,
    // Presence of wall on the boundary.
    // Without walls the world is a torus
    pub wall: bool,
}

impl Default for EnvironmentConfig {
    fn default() -> Self {
        Self {
            size: Size::new(1200.0, 800.0),
            wall: true,
        }
    }
}
//
// Resources
pub struct SimulationConfig {
    pub generation_length: u32,
    // Number of random animals to spawn in first generation
    pub start_population: i32,
    // Minimum number of animals in each generation. Randomized if 'missing'
    pub min_population: i32,
    // Number of child one surviving animal spawn in next generation
    pub fertility_rate: f32,
    // Minimum fitness required at end of generation to survive
    pub death_threshold: f32,
    // Average number of food that spawns per step
    pub food_spawn_rate: f64,
    pub environment: EnvironmentConfig,
}
impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            generation_length: 2500,
            start_population: 20,
            min_population: 1,
            fertility_rate: 1.1,
            death_threshold: 2.0,
            environment: EnvironmentConfig::default(),
            food_spawn_rate: (20.0 * 20.0) / 2500.0,
        }
    }
}
