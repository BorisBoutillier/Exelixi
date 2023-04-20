use std::{collections::HashMap, path::PathBuf};

use bevy_egui::egui::Color32;

use crate::ecosystem::*;

use super::{environment::EnvironmentConfig, species::SpeciesConfig, *};

#[derive(Serialize, Deserialize, Clone)]
pub struct EcosystemStatsConfig {
    // Defines the number of steps between each data aggregation.
    // If not provided, with be 0.1*smallest generation length.
    pub aggregation_rate: Option<u32>,
}
#[derive(Serialize, Deserialize, Resource, Clone)]
pub struct UserEcosystemConfig {
    // Configuration information regarding the environment
    pub environment: EnvironmentConfig,
    // Configuration information regarding the organisms
    pub species: Vec<SpeciesConfig>,
    // Configuration information regarding statistics creation
    pub statistics: EcosystemStatsConfig,
}
//
// Resources
//
#[derive(Resource, Clone)]
pub struct EcosystemConfig {
    pub environment: EnvironmentConfig,
    pub statistics_aggregation_rate: u32,
    pub species: HashMap<SpeciesId, SpeciesConfig>,
}
impl EcosystemConfig {
    pub fn from_path(path: Option<PathBuf>) -> Self {
        let user_config = match path {
            None => {
                let config = ron::from_str(include_str!("../../../../configs/default.ron"))
                    .expect("default_config.ron is not correct");
                log::info!("EcosystemConfig loaded from configs/default.ron");
                config
            }
            Some(path) => {
                if let Ok(ron_string) = std::fs::read_to_string(path.as_path()) {
                    match ron::from_str::<UserEcosystemConfig>(&ron_string) {
                        Ok(config) => {
                            log::info!("EcosystemConfig loaded from {:?}", path.as_os_str());
                            config
                        }
                        Err(err) => {
                            panic!("EcosystemConfig could not be loaded from {:?}, invalid content in the file: {err}",path.as_os_str());
                        }
                    }
                } else {
                    log::error!(
                        "EcosystemConfig could not be loaded from {:?}, file does not exists.",
                        path.as_os_str()
                    );
                    panic!();
                }
            }
        };
        Self::from_user_config(user_config)
    }
    pub fn from_user_config(user_config: UserEcosystemConfig) -> Self {
        let mut species = HashMap::new();
        let mut species_name_to_id = HashMap::new();
        for (i, mut species_config) in user_config.species.into_iter().enumerate() {
            let species_id = SpeciesId::new(i as u8);
            species_config.id = species_id;
            species_name_to_id.insert(species_config.name.clone(), species_id);
            species.insert(species_id, species_config);
        }
        let mut min_generation_length = u32::MAX;
        for species in species.values_mut() {
            species.update(&species_name_to_id);
            if let ReproductionConfig::GenerationEvolution {
                generation_length,
                min_population: _,
                fertility_rate: _,
                mutation_chance: _,
                mutation_amplitude: _,
                child_spawn_distance: _,
            } = species.reproduction
            {
                min_generation_length = min_generation_length.min(generation_length);
            }
        }
        let statistics_aggregation_rate =
            user_config.statistics.aggregation_rate.unwrap_or_else(|| {
                if min_generation_length != u32::MAX {
                    (min_generation_length / 10).max(1)
                } else {
                    1000
                }
            });
        Self {
            environment: user_config.environment,
            species,
            statistics_aggregation_rate,
        }
    }
    pub fn get_egui_color(
        &self,
        species_id: &SpeciesId,
        saturation: f32,
        lightness: f32,
    ) -> Color32 {
        let hue = self.species[species_id].visualization.hue;
        let [r, g, b, _] = Color::hsl(hue, saturation, lightness).as_rgba_f32();
        Color32::from_rgb((r * 256.0) as u8, (g * 256.0) as u8, (b * 256.0) as u8)
    }
    pub fn get_species_name(&self, species_id: &SpeciesId) -> &str {
        &self.species[species_id].name
    }
}
