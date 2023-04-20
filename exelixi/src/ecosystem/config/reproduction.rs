use super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ReproductionConfig {
    AutoSpawn {
        // Average number of organism that spawns per step
        spawn_rate: f64,
        // Mininum distance from an organism that can eat me
        #[serde(default)]
        minimum_distance_from_eater: f32,
    },
    GenerationEvolution {
        // Number of child one surviving organism spawn in next generation
        generation_length: u32,
        // Minimum number of organisms in each generation. Randomized if 'missing'
        min_population: usize,
        // Number of child one surviving organism spawn in next generation
        fertility_rate: f32,
        mutation_chance: f32,
        mutation_amplitude: f32,
        // Maximum distance child are spawning from survivors position
        // When None, child spawn randomly in the environment.
        #[serde(default)]
        child_spawn_distance: Option<f32>,
    },
}
