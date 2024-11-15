mod lifecycle;
mod organs;
mod reproduction;

use lib_genetic_algorithm::Chromosome;
pub use lifecycle::*;
pub use organs::*;
pub use reproduction::*;

use crate::ecosystem::*;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Organism {
    // Identifier of the species of this organism
    pub species: SpeciesId,
    // Chromosome used to create this organism
    // This can be an empty Chromosome for species without reproduction.
    pub chromosome: Chromosome,
    // Time in ticks since spawning of this organism
    pub age: u32,
}
impl Organism {
    pub fn new(species: SpeciesId, chromosome: Chromosome) -> Self {
        Self {
            species,
            chromosome,
            age: 0,
        }
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
