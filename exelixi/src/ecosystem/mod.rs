mod config;
mod kdtree;
mod organism;
mod position;
mod save;
mod schedule;
mod stats;

pub use bevy::log;
pub use bevy::prelude::*;
pub use rand::Rng;
pub use rand::RngCore;
pub use serde::{Deserialize, Serialize};

pub use config::*;
pub use kdtree::*;
pub use organism::*;
pub use position::*;
pub use save::*;
pub use schedule::*;
pub use stats::*;

use rand_isaac::IsaacRng;
use std::path::PathBuf;

use rand::SeedableRng;
#[derive(Resource, Serialize, Deserialize)]
pub struct Ecosystem {
    pub rng: IsaacRng,
    pub steps: u32,
}

pub struct EcosystemPlugin {
    pub seed: Option<u64>,
    pub config_path: Option<PathBuf>,
}
impl Plugin for EcosystemPlugin {
    fn build(&self, app: &mut App) {
        let rng = if let Some(seed) = self.seed {
            IsaacRng::seed_from_u64(seed)
        } else {
            IsaacRng::from_entropy()
        };
        let ecosystem_config = EcosystemConfig::from_path(self.config_path.clone());
        app.add_system(spawn_starting_organisms.in_base_set(CoreSet::PreUpdate));
        app.add_event::<NewGenerationEvent>();
        app.add_event::<SaveEcosystemEvent>();
        app.register_type::<SpeciesId>()
            .register_type::<CellSensors>()
            .register_type::<Organism>()
            .register_type::<Body>()
            .register_type::<Leaf>()
            .register_type::<Mouth>()
            .register_type::<Brain>()
            .register_type::<nn::Network>()
            .register_type::<nn::Layer>()
            .register_type::<nn::Neuron>()
            .register_type::<Locomotion>()
            .register_type::<Eye>();
        app.insert_resource(Ecosystem { rng, steps: 0 });
        app.insert_resource(ecosystem_config);
        app.insert_resource(GenerationEvolutions::default());
        app.insert_resource(OrganismKdTree::default());
        app.add_schedule(EcosystemSchedule, EcosystemSchedule::new_schedule());
        app.add_system(save_ecosystem_to_file);
    }
}
