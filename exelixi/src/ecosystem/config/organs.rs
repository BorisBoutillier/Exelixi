use super::*;

#[derive(Default, Serialize, Deserialize, Debug, Clone, Copy)]
pub enum CellSensors {
    // 2 sensors per cell, one for distance percentage, one for energy level pct of the
    // closest visible object in this cell.
    #[default]
    DistanceEnergy,
    // 3 sensors per cell, one for distance percentage, one for energy level pct ,
    // one for the hue pct of the closest visible object in this cell.
    DistanceEnergyHue,
}
impl CellSensors {
    pub fn n_sensors(&self) -> usize {
        match self {
            CellSensors::DistanceEnergy => 2,
            CellSensors::DistanceEnergyHue => 3,
        }
    }
    pub fn sensors(&self, distance_pct: f32, energy_pct: f32, hue: f32) -> Vec<f32> {
        match self {
            CellSensors::DistanceEnergy => vec![distance_pct, energy_pct],
            CellSensors::DistanceEnergyHue => vec![distance_pct, energy_pct, hue / 360.0],
        }
    }
}
// Configuration for the eye organ.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EyeConfig {
    // Total fov angle of all eyes cells in radians
    pub fov_angle: ConfigValue<f32>,
    // Distance each eye cell can see
    pub fov_range: ConfigValue<f32>,
    // Energy cost, per step of a full circle of 150.0 radius,
    pub energy_cost: f32,
    // Number of eye cells.
    // The eye fov angle is seperated in n_cells sectors.
    // Each cells accumulate information of content in its sector
    pub n_cells: ConfigValue<u8>,
    // Type of sensors per cell.
    #[serde(default)]
    pub cell_sensors: CellSensors,
    // Name of oganism that are visible by this eye.
    // Each eye cell will see the closest 'visible' organism
    pub visible: Vec<String>,
}

// Configuration for the body
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BodyConfig {
    pub starting_energy: f32,
    pub maximum_energy: f32,
    pub body_cost: f32,
}

// Configuration for locomotion organ
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocomotionConfig {
    pub linear: ConfigValue<f32>,
    // Cost for will be linear_cost*linear^2
    pub linear_cost: f32,
    // Cost for will be angular_cost*angular^2
    pub angular_cost: f32,
}

// Configuration for the eye organ.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LeafConfig {
    pub energy_production: f32,
    pub lifetime: u32,
}

// Configuration for the eye organ.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MouthConfig {
    pub reach: f32,
    pub edible: Vec<String>,
}
