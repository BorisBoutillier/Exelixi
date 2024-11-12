use std::collections::{BTreeMap, HashMap};

use crate::ecosystem::*;

#[allow(clippy::enum_variant_names)]
#[derive(Default, Serialize, Deserialize, Reflect, Debug, Clone, Copy)]
pub enum CellSensors {
    // 2 sensors per cell, one for distance percentage, one for energy level pct of the
    // closest visible object in this cell.
    #[default]
    DistanceEnergy,
    // 3 sensors per cell, one for distance percentage, one for energy level pct ,
    // one for the hue pct of the closest visible object in this cell.
    DistanceEnergyHue,
    // 2 * nVisibleSpecies per cell.
    // distance, energy per visible species, per cell
    DistanceEnergyPerSpecies,
}
impl CellSensors {
    pub fn n_sensors(&self, n_visible_species: usize) -> usize {
        match self {
            CellSensors::DistanceEnergy => 2,
            CellSensors::DistanceEnergyHue => 3,
            CellSensors::DistanceEnergyPerSpecies => 2 * n_visible_species,
        }
    }
    // For DistanceEnergyPerSpecies, this method must be called for each species.
    pub fn sensors(&self, per_species: &BTreeMap<SpeciesId, (f32, f32, f32)>) -> Vec<f32> {
        match self {
            CellSensors::DistanceEnergy => {
                let &(distance_pct, energy_pct, _) = per_species
                    .values()
                    .min_by(|(d1, _, _), (d2, _, _)| d1.partial_cmp(d2).unwrap())
                    .unwrap();
                vec![distance_pct, energy_pct]
            }
            CellSensors::DistanceEnergyHue => {
                let &(distance_pct, energy_pct, hue) = per_species
                    .values()
                    .min_by(|(d1, _, _), (d2, _, _)| d1.partial_cmp(d2).unwrap())
                    .unwrap();
                vec![distance_pct, energy_pct, hue / 360.0]
            }
            CellSensors::DistanceEnergyPerSpecies => per_species
                .values()
                .flat_map(|&(distance_pct, energy_pct, _)| vec![distance_pct, energy_pct])
                .collect(),
        }
    }
}
// Configuration for the eye organ.
#[derive(Reflect, Serialize, Deserialize, Debug, Clone)]
pub struct EyeConfig {
    // Total fov angle of all eyes cells in radians
    pub fov_angle: ConfigValue<f32>,
    // Distance each eye cell can see
    pub fov_range: ConfigValue<f32>,
    // Energy cost, per step of a full circle of 150.0 radius,
    pub energy_cost: f32,
    // Number of eye cells.
    // The eye fov angle is separated in n_cells sectors.
    // Each cells accumulate information of content in its sector
    pub n_cells: ConfigValue<u8>,
    // Type of sensors per cell.
    #[serde(default)]
    pub cell_sensors: CellSensors,
    // Name of oganism that are visible by this eye.
    // Each eye cell will see the closest 'visible' organism
    pub visible: Vec<String>,
    #[serde(skip)]
    pub visible_species: Vec<SpeciesId>,
}
impl EyeConfig {
    pub fn update(&mut self, species_name_to_id: &HashMap<String, SpeciesId>) {
        self.visible_species = self
            .visible
            .iter()
            .map(|name| {
                *species_name_to_id
                    .get(name)
                    .expect("Configuration of an Eye.visible contains an invalid species.")
            })
            .collect();
    }
}

// Configuration for the body
#[derive(Reflect, Serialize, Deserialize, Debug, Clone)]
pub struct BodyConfig {
    pub starting_energy: f32,
    pub maximum_energy: f32,
    pub body_cost: f32,
}

// Configuration for locomotion organ
#[derive(Reflect, Serialize, Deserialize, Debug, Clone)]
pub struct LocomotionConfig {
    pub linear: ConfigValue<f32>,
    // Cost for will be linear_cost*linear^2
    pub linear_cost: f32,
    // Cost for will be angular_cost*angular^2
    pub angular_cost: f32,
}

// Configuration for the eye organ.
#[derive(Reflect, Serialize, Deserialize, Debug, Clone)]
pub struct LeafConfig {
    pub energy_production: f32,
    pub lifetime: u32,
}

// Configuration for the mouth organ.
#[derive(Reflect, Serialize, Deserialize, Debug, Clone)]
pub struct MouthConfig {
    pub reach: f32,
    pub edible: Vec<String>,
    #[serde(skip)]
    pub edible_species: Vec<SpeciesId>,
}
impl MouthConfig {
    pub fn update(&mut self, species_name_to_id: &HashMap<String, SpeciesId>) {
        self.edible_species = self
            .edible
            .iter()
            .map(|name| species_name_to_id[name])
            .collect();
    }
}

// Configuration for the uterus organ.
#[derive(Reflect, Serialize, Deserialize, Debug, Clone)]
pub struct UterusConfig {
    pub mating_distance: f32,
}
