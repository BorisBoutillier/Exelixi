mod organs;
mod reproduction;

pub use organs::*;
pub use reproduction::*;

use crate::ecosystem::*;
#[derive(Component)]
pub struct Organism {
    // Name of this organism.
    // Must be one of Ecosystem config organism names.
    name: String,
    // Hue of this organism.
    // Must be coherent with name as per Ecosystem config.
    hue: f32,
}
impl Organism {
    pub fn new(config: &OrganismConfig) -> Self {
        Self {
            name: config.name.clone(),
            hue: config.visualization.hue,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn hue(&self) -> f32 {
        self.hue
    }
}
