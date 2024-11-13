mod organs;
mod reproduction;
mod spawn;

use crate::ecosystem::*;
pub use organs::*;
pub use reproduction::*;
pub use spawn::*;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
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
