use ga::Statistics;

use crate::prelude::*;

#[derive(Component)]
pub struct Velocity {
    pub linear: f32,
    pub angular: f32,
}
#[derive(Component)]
pub struct Food {}

#[derive(Component)]
pub struct Stomach {
    pub satiation: f32,
}
impl Stomach {
    pub fn new() -> Self {
        Self { satiation: 0.0 }
    }
}

impl Default for Stomach {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Component)]
pub struct Selected {}

// Resources
pub struct Simulation {
    pub age: u32,
    pub generation: u32,
    pub ga: ga::GeneticAlgorithm<ga::RouletteWheelSelection>,
    pub statistics: Statistics,
}
impl Simulation {
    pub fn new() -> Self {
        Self {
            age: 0,
            generation: 0,
            ga: ga::GeneticAlgorithm::new(
                ga::RouletteWheelSelection::default(),
                ga::UniformCrossover::default(),
                ga::GaussianMutation::new(0.01, 0.3),
            ),
            statistics: Statistics::default(),
        }
    }
}
impl Default for Simulation {
    fn default() -> Self {
        Self::new()
    }
}
