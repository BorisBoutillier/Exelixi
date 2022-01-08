use std::fmt;
use std::time::Duration;

use crate::prelude::*;
use ga::Statistics;

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
// Resources
pub struct Simulation {
    pub speed: SimulationSpeed,
    pub age: u32,
    pub generation: u32,
    pub ga: ga::GeneticAlgorithm<ga::RouletteWheelSelection>,
    pub statistics: Statistics,
    pub duration: Duration,
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
            statistics: Statistics::default(),
            duration: Duration::ZERO,
        }
    }
}
impl Default for Simulation {
    fn default() -> Self {
        Self::new()
    }
}
//
// Resources
pub struct SimulationConfig {
    pub generation_length: u32,
    pub starting_foods: u32,
    pub starting_animals: u32,
    pub environment_size: Size,
}
impl SimulationConfig {
    pub fn new() -> Self {
        Self {
            generation_length: 2500,
            starting_foods: 30,
            starting_animals: 20,
            environment_size: Size::new(1200.0, 700.0),
        }
    }
}
impl Default for SimulationConfig {
    fn default() -> Self {
        Self::new()
    }
}
