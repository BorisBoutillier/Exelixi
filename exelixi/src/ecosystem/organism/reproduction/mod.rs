mod autospawn;
mod birth;
mod generation;
mod individual;

pub use autospawn::*;
pub use birth::*;
pub use generation::*;
pub use individual::*;

use lib_genetic_algorithm as ga;
use std::collections::BTreeMap;

use crate::ecosystem::*;

#[derive(Resource, Default)]
pub struct GenerationEvolutions {
    pub per_species: BTreeMap<SpeciesId, GenerationEvolution>,
}
impl GenerationEvolutions {
    pub fn new(config: &EcosystemConfig) -> Self {
        let mut per_species = BTreeMap::new();
        for (species_id, organism_config) in config.species.iter() {
            if let ReproductionConfig::GenerationEvolution {
                generation_length: _,
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
    pub fertility_rate: f32,
    pub child_spawn_distance: Option<f32>,
}
impl GenerationEvolution {
    pub fn new(config: &SpeciesConfig) -> Self {
        if let ReproductionConfig::GenerationEvolution {
            generation_length,
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
                    ga::RouletteWheelSelection,
                    ga::UniformCrossover,
                    ga::GaussianMutation::new(mutation_chance, mutation_amplitude),
                ),
                fertility_rate,
                child_spawn_distance,
            }
        } else {
            panic!("Generation Evolution created for incorrect Config")
        }
    }
}
