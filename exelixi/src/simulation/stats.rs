use crate::prelude::*;

#[derive(Default, Debug, Clone, Copy)]
pub struct GenerationStatistics {
    pub start_size: usize,
    pub end_size: usize,
    pub avg_energy: f32,
    pub food_decay: usize,
}

#[derive(Default, Debug)]
pub struct SimulationStatistics {
    pub generations: Vec<GenerationStatistics>,
    pub cur_generation: GenerationStatistics,
}

impl SimulationStatistics {
    pub fn latest_dead(&self) -> usize {
        self.generations
            .last()
            .map_or(0, |s| (s.start_size - s.end_size))
    }
    pub fn latest_start_size(&self) -> usize {
        self.generations.last().map_or(0, |s| s.start_size)
    }
    pub fn latest_end_size(&self) -> usize {
        self.generations.last().map_or(0, |s| s.end_size)
    }
    pub fn latest_avg_energy(&self) -> f32 {
        self.generations.last().map_or(0.0, |s| s.avg_energy)
    }
    pub fn latest_food_decay(&self) -> usize {
        self.generations.last().map_or(0, |s| s.food_decay)
    }
    pub fn add_food_decay(&mut self, food_decay: usize) {
        self.cur_generation.food_decay += food_decay;
    }
    pub fn start_of_new_generation(&mut self, population: &[AnimalIndividual]) {
        self.cur_generation.start_size = population.len();
    }
    pub fn end_of_generation(&mut self, population: &[AnimalIndividual]) {
        let mut cur = self.cur_generation;
        cur.end_size = population.len();
        cur.avg_energy = population.iter().map(|i| i.energy).sum::<f32>() / (cur.end_size as f32);
        self.generations.push(cur);
        self.cur_generation = GenerationStatistics::default();
        //println!("---------------------------");
        //self.generations
        //    .iter()
        //    .enumerate()
        //    .for_each(|(i, s)| println!("{:03}: {:?}", i, s));
    }
}
