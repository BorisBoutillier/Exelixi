mod autospawn;
mod generation;
mod individual;

pub use autospawn::*;
pub use generation::*;
pub use individual::*;

use lib_genetic_algorithm as ga;
use std::collections::HashMap;

use crate::ecosystem::*;

#[derive(Resource, Default)]
pub struct GenerationEvolutions {
    pub per_species: HashMap<SpeciesId, GenerationEvolution>,
}
impl GenerationEvolutions {
    pub fn new(config: &EcosystemConfig) -> Self {
        let mut per_species = HashMap::new();
        for (species_id, organism_config) in config.species.iter() {
            if let ReproductionConfig::GenerationEvolution {
                generation_length: _,
                min_population: _,
                fertility_rate: _,
                mutation_chance: _,
                mutation_amplitude: _,
                child_spawn_distance: _,
            } = organism_config.reproduction
            {
                per_species.insert(*species_id, GenerationEvolution::new(organism_config));
            }
        }
        Self { per_species }
    }
}

pub struct GenerationEvolution {
    pub config: SpeciesConfig,
    // genetic algorithm
    pub genetic_algorithm: ga::GeneticAlgorithm<ga::RouletteWheelSelection>,
    //
    pub generation_length: u32,
    pub minimum_population: usize,
    pub fertility_rate: f32,
    pub child_spawn_distance: Option<f32>,
}
impl GenerationEvolution {
    pub fn new(config: &SpeciesConfig) -> Self {
        if let ReproductionConfig::GenerationEvolution {
            generation_length,
            min_population,
            fertility_rate,
            mutation_chance,
            mutation_amplitude,
            child_spawn_distance,
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
                child_spawn_distance,
            }
        } else {
            panic!("Generation Evolution created for incorrect Config")
        }
    }
}
