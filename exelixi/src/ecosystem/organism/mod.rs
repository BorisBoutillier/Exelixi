mod lifecycle;
mod organs;
mod reproduction;

use crate::ecosystem::*;
pub use lifecycle::*;
pub use organs::*;
pub use reproduction::*;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Organism {
    // Identifier of the species of this organism
    species: SpeciesId,
    // Hue of this organism.
    // Must be coherent with name as per Ecosystem config.
    hue: f32,
    // Age in ticks since spawning of this organism
    age: u32,
}
impl Organism {
    pub fn new(config: &SpeciesConfig) -> Self {
        Self {
            species: config.id,
            hue: config.visualization.hue,
            age: 0,
        }
    }
    pub fn hue(&self) -> f32 {
        self.hue
    }
    pub fn species(&self) -> SpeciesId {
        self.species
    }
    pub fn age(&self) -> u32 {
        self.age
    }
    pub fn tick(&mut self) {
        self.age += 1;
    }
}

pub fn organism_aging(mut organisms: Query<&mut Organism>) {
    for mut organism in organisms.iter_mut() {
        organism.tick();
    }
}
