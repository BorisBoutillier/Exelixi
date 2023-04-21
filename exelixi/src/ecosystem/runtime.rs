use std::collections::BTreeMap;

use crate::ecosystem::*;

#[derive(Resource, Serialize, Deserialize)]
pub struct EcosystemRuntime {
    pub rng: IsaacRng,
    pub steps: u32,
    pub generation: BTreeMap<SpeciesId, u32>,
}
impl EcosystemRuntime {
    pub fn new(rng: IsaacRng, config: &EcosystemConfig) -> Self {
        let mut generation = BTreeMap::new();
        for species in config.species.keys() {
            generation.insert(*species, 0);
        }
        Self {
            rng,
            steps: 0,
            generation,
        }
    }
    pub fn increate_generation(&mut self, species: &SpeciesId) {
        if let Some(generation) = self.generation.get_mut(species) {
            *generation += 1;
        }
    }
}

pub fn initialize_on_new_config(mut commands: Commands, config: Res<EcosystemConfig>) {
    if config.is_changed() {
        // Rebuild all the 'compiled' resources
        commands.insert_resource(EcosystemStatistics::new(&config));
        commands.insert_resource(GenerationEvolutions::new(&config));
    }
}
