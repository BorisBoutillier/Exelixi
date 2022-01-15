use crate::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EnvironmentConfig {
    // Width of the floor
    pub width: f32,
    // Height of the floor
    pub height: f32,
    // Presence of wall on the boundary.
    // Without walls the world is a torus
    pub wall: bool,
}

//
// Resources
//
#[derive(Serialize, Deserialize)]
pub struct SimulationConfig {
    pub generation_length: u32,
    // Number of random animals to spawn in first generation
    pub start_population: i32,
    // Minimum number of animals in each generation. Randomized if 'missing'
    pub min_population: i32,
    // Number of child one surviving animal spawn in next generation
    pub fertility_rate: f32,
    // Minimum fitness required at end of generation to survive
    pub death_threshold: f32,
    // Average number of food that spawns per step
    pub food_spawn_rate: f64,
    pub environment: EnvironmentConfig,
}
impl SimulationConfig {
    pub fn get_default_config() -> Self {
        match Self::load_from_default_file() {
            Some(config) => config,
            None => {
                let config = ron::from_str(include_str!("default_config.ron"))
                    .expect("default_config.ron is not correct");
                log::info!("SimulationConfig loaded from default_config.ron");
                config
            }
        }
    }
    pub fn save_as_default_file(&self) {
        if let Some(mut path) = dirs::config_dir() {
            path.push("exelixi");
            if std::fs::create_dir_all(path.as_path()).is_err() {
                println!(
                    "Could not create configuration directory {:?}",
                    path.as_os_str()
                );
                return;
            }

            path.push("default_simulation_configuration");
            path.set_extension("ron");
            let ron_string = ron::to_string(&self).expect("Cannot Ronify the SimulationConfig");
            if std::fs::write(path.as_path(), ron_string).is_err() {
                println!(
                    "Could not write SimulationConfig to file {:?}",
                    path.as_os_str()
                );
            }
        }
    }
    pub fn load_from_default_file() -> Option<Self> {
        if let Some(mut path) = dirs::config_dir() {
            path.push("exelixi");
            path.push("default_simulation_configuration");
            path.set_extension("ron");
            if let Ok(ron_string) = std::fs::read_to_string(path.as_path()) {
                if let Ok(config) = ron::from_str::<SimulationConfig>(&ron_string) {
                    log::info!("SimulationConfig loaded from {:?}", path.as_os_str());
                    return Some(config);
                } else {
                    log::error!("SimulationConfig could not be loaded from {:?}, invalid content in the file.",path.as_os_str());
                }
            }
        }
        None
    }
}
