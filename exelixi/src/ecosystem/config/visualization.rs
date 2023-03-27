use super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrganismVisualizationConfig {
    pub hue: f32,
    pub sprite_file: String,
    pub sprite_size: (f32, f32),
}
