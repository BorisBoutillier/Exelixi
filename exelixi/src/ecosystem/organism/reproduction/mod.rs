mod autospawn;
mod generation;
mod individual;

pub use autospawn::*;
pub use generation::*;
pub use individual::*;

use bevy::prelude::Resource;
use lib_genetic_algorithm as ga;
use std::collections::HashMap;

use crate::prelude::{OrganismConfig, ReproductionConfig};

#[derive(Resource, Default)]
pub struct GenerationEvolutions(HashMap<String, GenerationEvolution>);

pub struct GenerationEvolution {
    pub config: OrganismConfig,
    // genetic algorithm
    pub genetic_algorithm: ga::GeneticAlgorithm<ga::RouletteWheelSelection>,
    //
    pub generation_length: u32,
    pub minimum_population: usize,
    pub fertility_rate: f32,
    // Current generation for this organism
    pub current_generation: u32,
}
impl GenerationEvolution {
    pub fn new(config: &OrganismConfig) -> Self {
        if let ReproductionConfig::GenerationEvolution {
            generation_length,
            min_population,
            fertility_rate,
            mutation_chance,
            mutation_amplitude,
        } = config.reproduction
        {
            Self {
                config: config.clone(),
                generation_length,
                genetic_algorithm: ga::GeneticAlgorithm::new(
                    ga::RouletteWheelSelection::default(),
                    ga::UniformCrossover::default(),
                    ga::GaussianMutation::new(mutation_chance, mutation_amplitude),
                ),
                minimum_population: min_population,
                fertility_rate,
                current_generation: 0,
            }
        } else {
            panic!("Generation Evolution created for incorrect Config")
        }
    }
}
