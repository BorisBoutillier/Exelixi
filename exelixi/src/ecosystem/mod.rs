mod config;
mod organism;
mod position;
mod schedule;
mod stats;

pub use bevy::log;
pub use bevy::prelude::*;
pub use rand::Rng;
pub use rand::RngCore;

pub use config::*;
pub use organism::*;
pub use position::*;
pub use schedule::*;
pub use stats::*;

pub use crate::prelude::Simulation;

use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::path::PathBuf;

#[derive(Resource)]
pub struct EcosystemRng(pub ChaCha8Rng);

pub struct EcosystemPlugin {
    pub seed: Option<u64>,
    pub config_path: Option<PathBuf>,
}
impl Plugin for EcosystemPlugin {
    fn build(&self, app: &mut App) {
        let rng = if let Some(seed) = self.seed {
            ChaCha8Rng::seed_from_u64(seed)
        } else {
            ChaCha8Rng::from_entropy()
        };
        let ecosystem_config = EcosystemConfig::from_path(self.config_path.clone());
        app.add_system(spawn_starting_organisms.in_base_set(CoreSet::PreUpdate));
        app.add_event::<NewGenerationEvent>();
        app.insert_resource(EcosystemRng(rng));
        app.insert_resource(ecosystem_config);
        app.insert_resource(GenerationEvolutions::default());
        app.add_schedule(EcosystemSchedule, EcosystemSchedule::new_schedule());
    }
}
