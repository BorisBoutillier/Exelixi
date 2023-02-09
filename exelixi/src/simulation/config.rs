use std::path::PathBuf;

use crate::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ConfigValue<T> {
    Fixed(T),
    Gene { min: T, max: T },
    Neuron { min: T, max: T },
}

#[derive(Serialize, Deserialize)]
pub struct OrganismsConfig {
    // Fov angle of the eye
    pub eye_fov_angle: ConfigValue<f32>,
    // Fov angle of the eye
    pub eye_fov_range: ConfigValue<f32>,
    // Cost for a full circle of 150.0 radius,
    pub eye_energy_cost: f32,
    // Number of eye cells.
    // The eye fov angle is seperated in n_eye_cells sectors.
    // Each cells accumulate information of content in its sector
    pub n_eye_cells: ConfigValue<u8>,
    // Does the eyes senses the foods.
    // will add n_eyes inputs to the neural networks
    pub see_foods: bool,
    // Does the eyes senses the walls.
    // will add n_eye_cells inputs to the neural networks
    pub see_walls: bool,
    // Does the eyes senses other organisms.
    // Will add n_eye_cells inputs to the neural networks for each organism type
    pub see_organisms: bool,
    pub starting_energy: f32,
    pub maximum_energy: f32,
    pub linear_locomotion: ConfigValue<f32>,
    // Cost for will be linear_cost*linear^2
    pub linear_cost: f32,
    // Cost for will be angular_cost*angular^2
    pub angular_cost: f32,
    // Cost to run the body.
    // This defines a minimum energy consumption per step
    pub body_cost: f32,
}
#[derive(Serialize, Deserialize)]
pub struct EnvironmentConfig {
    // Width of the floor
    pub width: f32,
    // Height of the floor
    pub height: f32,
    // Presence of wall on the boundary.
    // Without walls the world is a torus
    pub wall: bool,
    // Average number of food that spawns per step
    pub food_spawn_rate: f64,
    // Number of steps after appearance that a food disappear
    pub food_decay_time: u32,
    pub food_energy: f32,
}

//
// Resources
//
#[derive(Serialize, Deserialize, Resource)]
pub struct SimulationConfig {
    pub generation_length: u32,
    // Minimum number of organisms in each generation. Randomized if 'missing'
    pub min_population: usize,
    // Number of child one surviving organism spawn in next generation
    pub fertility_rate: f32,
    // Defines if the simulation must be stopped after reaching a given generation.
    pub exit_at_generation: Option<u32>,
    // Defines how the simulation behaves at the start. One of Paused, Run or Fastest
    pub start_state: SimulationControlState,
    // Configuration information regarding the environment
    pub environment: EnvironmentConfig,
    // Configuration information regarding the organisms
    pub organisms: OrganismsConfig,
}
impl SimulationConfig {
    pub fn from_path(path: Option<PathBuf>) -> Self {
        match path {
            None => {
                let config = ron::from_str(include_str!("../../../configs/default.ron"))
                    .expect("default_config.ron is not correct");
                log::info!("SimulationConfig loaded from default_config.ron");
                config
            }
            Some(path) => {
                if let Ok(ron_string) = std::fs::read_to_string(path.as_path()) {
                    if let Ok(config) = ron::from_str::<SimulationConfig>(&ron_string) {
                        log::info!("SimulationConfig loaded from {:?}", path.as_os_str());
                        config
                    } else {
                        log::error!("SimulationConfig could not be loaded from {:?}, invalid content in the file.",path.as_os_str());
                        panic!();
                    }
                } else {
                    log::error!(
                        "SimulationConfig could not be loaded from {:?}, file does not exists.",
                        path.as_os_str()
                    );
                    panic!();
                }
            }
        }
    }
}
