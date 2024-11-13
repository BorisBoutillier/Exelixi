use std::collections::BTreeMap;

use crate::ecosystem::*;

#[derive(Resource, Reflect, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct EcosystemRuntime {
    pub steps: u32,
    pub generation: BTreeMap<SpeciesId, u32>,
    pub population: BTreeMap<SpeciesId, usize>,
}
impl EcosystemRuntime {
    pub fn new(config: &EcosystemConfig) -> Self {
        let mut generation = BTreeMap::new();
        let mut population = BTreeMap::new();
        for species in config.species.keys() {
            generation.insert(*species, 0);
            population.insert(*species, 0);
        }
        Self {
            steps: 0,
            generation,
            population,
        }
    }
    pub fn increment_generation(&mut self, species: &SpeciesId) {
        if let Some(generation) = self.generation.get_mut(species) {
            *generation += 1;
        }
    }
    pub fn increase_population(&mut self, species: &SpeciesId) {
        if let Some(population) = self.population.get_mut(species) {
            *population += 1;
        } else {
            panic!("Unconfigured Species: {species:?}");
        }
    }
    pub fn decrease_population(&mut self, species: &SpeciesId) {
        if let Some(population) = self.population.get_mut(species) {
            *population -= 1;
        } else {
            panic!("Unconfigured Species: {species:?}");
        }
    }
}

pub fn initialize_on_new_config(mut commands: Commands, config: Res<EcosystemConfig>) {
    if config.is_changed() {
        // Rebuild all the 'compiled' resources
        commands.insert_resource(GenerationEvolutions::new(&config));
    }
}

// At the beginning of the simulation, ensure each species' population is at the minimum.
// If under the minimum, create random organism of this species, at a random position.
pub fn ensure_minimum_population(
    organisms: Query<&Organism>,
    mut organism_lives: ResMut<OrganismsLifecycle>,
    ecosystem: ResMut<EcosystemRuntime>,
    ecosystem_config: Res<EcosystemConfig>,
) {
    let mut counts = BTreeMap::new();
    for organism in organisms.iter() {
        *counts.entry(organism.species()).or_insert(0) += 1;
        assert!(ecosystem.population.contains_key(&organism.species()))
    }
    for (species, &population) in ecosystem.population.iter() {
        assert_eq!(
            counts.get(species).unwrap_or(&0),
            &population,
            "At tick {}, For {species:?}, name {}",
            ecosystem.steps,
            ecosystem_config.species[species].name
        );
        for _ in population..ecosystem_config.species[species].minimum_population {
            organism_lives.births.push(OrganismBirth {
                species: *species,
                position: None,
                energy: None,
                chromosome: None,
            });
        }
    }
}
