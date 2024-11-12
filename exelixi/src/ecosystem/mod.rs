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
pub use serde::{Deserialize, Serialize};

use bevy_rand::prelude::*;
pub use config::*;
pub use kdtree::*;
pub use load::*;
pub use organism::*;
pub use position::*;
pub use runtime::*;
pub use save::*;
pub use schedule::*;
pub use stats::*;

use std::path::PathBuf;

pub struct EcosystemPlugin {
    pub seed: Option<u64>,
    pub config_path: Option<PathBuf>,
}
impl Plugin for EcosystemPlugin {
    fn build(&self, app: &mut App) {
        let entropy_plugin = if let Some(seed) = self.seed {
            EntropyPlugin::<WyRand>::with_seed(seed.to_ne_bytes())
        } else {
            EntropyPlugin::<WyRand>::default()
        };
        app.add_plugins(entropy_plugin);
        use bevy_trait_query::RegisterExt;
        app.register_component_as::<dyn EnergyActor, Brain>();
        app.register_component_as::<dyn EnergyActor, Eye>();
        app.register_component_as::<dyn EnergyActor, Leaf>();
        app.register_component_as::<dyn EnergyActor, Locomotion>();
        app.register_component_as::<dyn EnergyActor, Mouth>();
        app.register_component_as::<dyn EnergyActor, Uterus>();
        app.register_type::<SpeciesId>()
            .register_type::<CellSensors>()
            .register_type::<Position>()
            .register_type::<Organism>()
            .register_type::<Body>()
            .register_type::<Brain>()
            .register_type::<Eye>()
            .register_type::<Leaf>()
            .register_type::<Locomotion>()
            .register_type::<Mouth>()
            .register_type::<Uterus>()
            .register_type::<nn::Network>()
            .register_type::<nn::Layer>()
            .register_type::<nn::Neuron>()
            .register_type::<EcosystemConfig>()
            .register_type::<EcosystemRuntime>()
            .register_type::<EcosystemStatistics>()
            .register_type::<SpeciesStatistics>()
            .register_type::<SpeciesStatistic>()
            .register_type::<SpeciesConfig>()
            .register_type::<BodyConfig>()
            .register_type::<EyeConfig>()
            .register_type::<Vec<SpeciesId>>()
            .register_type::<Vec<nn::Layer>>()
            .register_type::<Vec<nn::Neuron>>()
            .register_type::<Vec<f32>>();

        let ecosystem_config = EcosystemConfig::from_path(self.config_path.clone());
        app.insert_resource(EcosystemRuntime::new(&ecosystem_config));
        app.insert_resource(EcosystemStatistics::new(&ecosystem_config));
        app.insert_resource(GenerationEvolutions::new(&ecosystem_config));
        app.insert_resource(ecosystem_config);

        app.insert_resource(OrganismKdTree::default());
        app.add_schedule(EcosystemSchedule::new_schedule());
        app.add_systems(PreUpdate, initialize_on_new_config);
    }
}
