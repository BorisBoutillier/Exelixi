#![feature(crate_visibility_modifier)]
use std::f32::consts::FRAC_PI_2;

pub use self::{animal::*, animal_individual::*, brain::*, eye::*, food::*, world::*};

mod animal;
mod animal_individual;
mod brain;
mod eye;
mod food;
mod world;

use genetic_algorithm as ga;
use nalgebra as na;
use neural_network as nn;
use rand::{Rng, RngCore};

const SPEED_MIN: f32 = 0.001;
const SPEED_MAX: f32 = 0.005;
const SPEED_ACCEL: f32 = 0.001;
const ROTATION_ACCEL: f32 = FRAC_PI_2;
const GENERATION_LENGTH: usize = 2500;

pub struct Simulation {
    world: World,
    ga: ga::GeneticAlgorithm<ga::RouletteWheelSelection>,
    age: usize,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let world = World::random(rng);
        let ga = ga::GeneticAlgorithm::new(
            ga::RouletteWheelSelection::default(),
            ga::UniformCrossover::default(),
            ga::GaussianMutation::new(0.01, 0.3),
        );
        Self { world, ga, age: 0 }
    }
    pub fn world(&self) -> &World {
        &self.world
    }
    pub fn step(&mut self, rng: &mut dyn RngCore) -> Option<ga::Statistics> {
        self.process_collisions(rng);
        self.process_brains();
        self.process_movements();

        self.age += 1;

        if self.age > GENERATION_LENGTH {
            Some(self.evolve(rng))
        } else {
            None
        }
    }
    pub fn train(&mut self, rng: &mut dyn RngCore) -> ga::Statistics {
        loop {
            if let Some(stats) = self.step(rng) {
                return stats;
            }
        }
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for animal in &mut self.world.animals {
            for food in &mut self.world.foods {
                let distance = na::distance(&animal.position, &food.position);
                if distance <= 0.01 {
                    food.position = rng.gen();
                    animal.satiation += 1;
                }
            }
        }
    }
    fn process_movements(&mut self) {
        for animal in &mut self.world.animals {
            animal.position += animal.rotation * na::Vector2::new(animal.speed, 0.0);
            animal.position.x = na::wrap(animal.position.x, 0.0, 1.0);
            animal.position.y = na::wrap(animal.position.y, 0.0, 1.0);
        }
    }

    fn process_brains(&mut self) {
        for animal in &mut self.world.animals {
            let vision =
                animal
                    .eye
                    .process_vision(animal.position, animal.rotation, &self.world.foods);

            let response = animal.brain.nn.propagate(&vision);
            let speed = response[0].clamp(0.0, 2.0 * SPEED_ACCEL) - SPEED_ACCEL;

            let rotation = response[1].clamp(-ROTATION_ACCEL, ROTATION_ACCEL);

            animal.speed = (animal.speed + speed).clamp(SPEED_MIN, SPEED_MAX);

            animal.rotation = na::Rotation2::new(animal.rotation.angle() + rotation);
        }
    }
    fn evolve(&mut self, rng: &mut dyn RngCore) -> ga::Statistics {
        self.age = 0;
        let current_population = self
            .world
            .animals
            .iter()
            .map(AnimalIndividual::from_animal)
            .collect::<Vec<_>>();

        let (new_population, stats) = self.ga.evolve(rng, &current_population);

        self.world.animals = new_population
            .into_iter()
            .map(|a| a.into_animal(rng))
            .collect();

        // Reset the food position to visually see when evolve happens in the UI
        for food in &mut self.world.foods {
            food.position = rng.gen();
        }

        stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test() {
        // Because we always use the same seed, our `rng` in here will
        // always return the same set of values
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let mut simulation = Simulation::random(&mut rng);

        for _ in 0..5001 {
            simulation.step(&mut rng);
        }
    }
}
