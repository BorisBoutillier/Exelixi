use super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ReproductionConfig {
    AutoSpawn {
        // Average number of organism that spawns per step
        spawn_rate: f64,
    },
    EndOfGenerationEvolution {
        // Number of child one surviving organism spawn in next generation
        generation_length: u32,
        // Minimum number of organisms in each generation. Randomized if 'missing'
        min_population: usize,
        // Number of child one surviving organism spawn in next generation
        fertility_rate: f32,
        mutation_chance: f32,
        mutation_amplitude: f32,
    },
}
