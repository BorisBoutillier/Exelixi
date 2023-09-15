mod config;
mod kdtree;
mod load;
mod organism;
mod position;
mod runtime;
mod save;
mod schedule;
mod stats;

pub use bevy::log;
pub use bevy::prelude::*;
pub use rand::Rng;
pub use rand::RngCore;
pub use rand_isaac::IsaacRng;
pub use serde::{Deserialize, Serialize};

pub use config::*;
pub use kdtree::*;
pub use load::*;
pub use organism::*;
pub use position::*;
pub use runtime::*;
pub use save::*;
pub use schedule::*;
pub use stats::*;

use rand::SeedableRng;
use std::path::PathBuf;

pub struct EcosystemPlugin {
    pub seed: Option<u64>,
    pub config_path: Option<PathBuf>,
    pub load_path: Option<PathBuf>,
}
impl Plugin for EcosystemPlugin {
    fn build(&self, app: &mut App) {
        let rng = if let Some(seed) = self.seed {
            IsaacRng::seed_from_u64(seed)
        } else {
            IsaacRng::from_entropy()
        };
        let ecosystem_config = EcosystemConfig::from_path(self.config_path.clone());
        app.add_event::<NewGenerationEvent>();
        app.add_event::<SaveEcosystemEvent>();
        app.add_event::<LoadEcosystemEvent>();
        if let Some(path) = self.load_path.as_ref() {
            let mut load_events = Events::<LoadEcosystemEvent>::default();
            load_events.send(LoadEcosystemEvent {
                path: path.to_string_lossy().into_owned(),
            });
            app.insert_resource(load_events);
        }
        app.register_type::<SpeciesId>()
            .register_type::<CellSensors>()
            .register_type::<Position>()
            .register_type::<Organism>()
            .register_type::<Body>()
            .register_type::<Leaf>()
            .register_type::<Mouth>()
            .register_type::<Brain>()
            .register_type::<nn::Network>()
            .register_type::<nn::Layer>()
            .register_type::<nn::Neuron>()
            .register_type::<Locomotion>()
            .register_type::<Eye>()
            .register_type::<Vec<SpeciesId>>()
            .register_type::<Vec<nn::Layer>>()
            .register_type::<Vec<nn::Neuron>>()
            .register_type::<Vec<f32>>();
        app.insert_resource(EcosystemRuntime::new(rng, &ecosystem_config));
        app.insert_resource(ecosystem_config);
        app.insert_resource(GenerationEvolutions::default());
        app.insert_resource(OrganismKdTree::default());
        app.add_schedule(EcosystemSchedule, EcosystemSchedule::new_schedule());
        //app.add_systems(Update, save_ecosystem_to_file());
        app.add_systems(Update, save_to_file.pipe(then_exit));
        app.add_systems(Update, load_ecosystem_from_file);
        app.add_systems(PreUpdate, initialize_on_new_config);
    }
}
