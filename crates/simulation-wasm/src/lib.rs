use lib_simulation as sim;
use rand::prelude::*;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Simulation {
    generation: usize,
    rng: ThreadRng,
    sim: sim::Simulation,
}

#[allow(clippy::new_without_default)]
#[wasm_bindgen]
impl Simulation {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let sim = sim::Simulation::random(&mut rng);
        println!("CREATED");
        Self {
            generation: 0,
            rng,
            sim,
        }
    }
    pub fn world(&self) -> JsValue {
        let world = World::from(self.sim.world());
        JsValue::from_serde(&world).unwrap()
    }
    pub fn step(&mut self) -> String {
        if let Some(stats) = self.sim.step(&mut self.rng) {
            self.generation += 1;
            format!(
                "{:03}: min={:.2}, max={:.2}, avg={:.2}",
                self.generation,
                stats.min_fitness(),
                stats.max_fitness(),
                stats.avg_fitness()
            )
        } else {
            "".to_string()
        }
    }
    pub fn train(&mut self) -> String {
        let stats = self.sim.train(&mut self.rng);
        self.generation += 1;

        format!(
            "{:03}: min={:.2}, max={:.2}, avg={:.2}",
            self.generation,
            stats.min_fitness(),
            stats.max_fitness(),
            stats.avg_fitness()
        )
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct World {
    pub animals: Vec<Animal>,
    pub foods: Vec<Food>,
}

impl From<&sim::World> for World {
    fn from(world: &sim::World) -> Self {
        let animals = world.animals().iter().map(Animal::from).collect();
        let foods = world.foods().iter().map(Food::from).collect();
        Self { animals, foods }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Animal {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
}

impl From<&sim::Animal> for Animal {
    fn from(animal: &sim::Animal) -> Self {
        Self {
            x: animal.position().x,
            y: animal.position().y,
            rotation: animal.rotation().angle(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Food {
    pub x: f32,
    pub y: f32,
}

impl From<&sim::Food> for Food {
    fn from(food: &sim::Food) -> Self {
        Self {
            x: food.position().x,
            y: food.position().y,
        }
    }
}
