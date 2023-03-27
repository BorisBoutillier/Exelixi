use super::*;

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
