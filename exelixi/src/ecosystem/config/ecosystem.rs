use std::{collections::HashMap, path::PathBuf};

use bevy_egui::egui::Color32;

use crate::ecosystem::*;

use super::{environment::EnvironmentConfig, organism::OrganismConfig, *};

#[derive(Serialize, Deserialize, Clone)]
pub struct EcosystemStatsConfig {
    // Defines the number of steps between each data aggregation.
    // If not provided, with be 0.1*smallest generation length.
    pub aggregation_rate: Option<u32>,
}
//
// Resources
//
#[derive(Serialize, Deserialize, Resource, Clone)]
pub struct EcosystemConfig {
    // Configuration information regarding the environment
    pub environment: EnvironmentConfig,
    // Configuration information regarding the organisms
    pub organisms: Vec<OrganismConfig>,
    // Configuration information regarding statistics creation
    pub statistics: EcosystemStatsConfig,
    #[serde(skip)]
    pub organisms_per_name: HashMap<String, OrganismConfig>,
}
impl EcosystemConfig {
    pub fn from_path(path: Option<PathBuf>) -> Self {
        let mut config = match path {
            None => {
                let config = ron::from_str(include_str!("../../../../configs/default.ron"))
                    .expect("default_config.ron is not correct");
                log::info!("EcosystemConfig loaded from configs/default.ron");
                config
            }
            Some(path) => {
                if let Ok(ron_string) = std::fs::read_to_string(path.as_path()) {
                    match ron::from_str::<EcosystemConfig>(&ron_string) {
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
        for organism_config in config.organisms.iter() {
            config
                .organisms_per_name
                .insert(organism_config.name.clone(), organism_config.clone());
        }
        config.update();
        config
    }
    // Update configuration.
    // Allow to set 'undefinied' values based on other configuration values.
    // Also check for configuration incoherencies.
    fn update(&mut self) {
        let mut min_generation_length = u32::MAX;
        for organism in self.organisms.iter() {
            // Check that each mouth.edible must reference defined organism name
            if let Some(mouth_config) = &organism.mouth {
                for name in mouth_config.edible.iter() {
                    if !self.organisms_per_name.contains_key(name) {
                        panic!("Undefined organism '{name}' referenced in a mouth.edible");
                    }
                }
            }
            if let ReproductionConfig::GenerationEvolution {
                generation_length,
                min_population: _,
                fertility_rate: _,
                mutation_chance: _,
                mutation_amplitude: _,
            } = organism.reproduction
            {
                min_generation_length = min_generation_length.min(generation_length);
            }
        }
        if self.statistics.aggregation_rate.is_none() {
            if min_generation_length != u32::MAX {
                self.statistics.aggregation_rate = Some((min_generation_length / 10).max(1));
            } else {
                self.statistics.aggregation_rate = Some(1000);
            }
        }
        println!("Aggreation: {:?}", self.statistics.aggregation_rate);
    }
    pub fn get_egui_color(&self, organism_name: &str, saturation: f32, lightness: f32) -> Color32 {
        let hue = self.organisms_per_name[organism_name].visualization.hue;
        let [r, g, b, _] = Color::hsl(hue, saturation, lightness).as_rgba_f32();
        Color32::from_rgb((r * 256.0) as u8, (g * 256.0) as u8, (b * 256.0) as u8)
    }
}
