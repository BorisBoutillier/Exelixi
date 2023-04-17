mod organs;
mod reproduction;

pub use organs::*;
pub use reproduction::*;

use crate::ecosystem::*;
#[derive(Component)]
pub struct Organism {
    // Identifier of the species of this organism
    species: SpeciesId,
    // Hue of this organism.
    // Must be coherent with name as per Ecosystem config.
    hue: f32,
}
impl Organism {
    pub fn new(config: &SpeciesConfig) -> Self {
        Self {
            species: config.id,
            hue: config.visualization.hue,
        }
    }
    pub fn hue(&self) -> f32 {
        self.hue
    }
    pub fn species(&self) -> SpeciesId {
        self.species
    }
}
