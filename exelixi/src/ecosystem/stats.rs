use std::collections::HashMap;

use crate::ecosystem::*;

#[derive(Default, Debug, Clone)]
pub struct GenerationStatistics {
    pub start_size: usize,
    pub end_size: usize,
    pub avg_energy: f32,
    deaths: HashMap<String, usize>,
}

impl GenerationStatistics {
    pub fn get_deaths(&self, name: &str) -> usize {
        *self.deaths.get(name).unwrap_or(&0)
    }
}
#[derive(Default, Debug)]
// FIXME
pub struct PopulationStatistics {
    //pub count: usize,
    //pub fov_range: HashMap<Range<u32>, usize>,
    //pub fov_angle: HashMap<Range<u32>, usize>,
}

impl PopulationStatistics {
    pub fn new(_config: &EcosystemConfig) -> Self {
        //let mut s = Self::default();
        //if let ConfigValue::Gene { min, max } = config.organisms.eye_fov_range {
        //    let min = min as u32;
        //    let max = max as u32;
        //    let step = (max - min) / 20;
        //    for i in (min..max).step_by(step as usize) {
        //        s.fov_range.insert(i..(i + step), 0);
        //    }
        //}
        //if let ConfigValue::Gene { min, max } = config.organisms.eye_fov_angle {
        //    let min = (min * 100.0) as u32;
        //    let max = (max * 100.0) as u32;
        //    let step = (max - min) / 20;
        //    for i in (min..max).step_by(step as usize) {
        //        s.fov_angle.insert(i..(i + step), 0);
        //    }
        //}
        //s
        Self {}
    }
    //pub fn add_entry(&mut self, eye: &Eye) {
    //    self.count += 1;
    //    if !self.fov_range.is_empty() {
    //        let fov_range = eye.fov_range as u32;
    //        for (range, count) in self.fov_range.iter_mut() {
    //            if range.contains(&fov_range) {
    //                *count += 1;
    //            }
    //        }
    //    }
    //    if !self.fov_angle.is_empty() {
    //        let fov_angle = (eye.fov_angle * 100.0) as u32;
    //        for (range, count) in self.fov_angle.iter_mut() {
    //            if range.contains(&fov_angle) {
    //                *count += 1;
    //            }
    //        }
    //    }
    //}
}

#[derive(Default, Debug)]
pub struct SimulationStatistics {
    pub generations: Vec<GenerationStatistics>,
    pub cur_generation: GenerationStatistics,
    pub population: PopulationStatistics,
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
        self.generations.last().map_or(0, |s| s.get_deaths("Plant"))
    }
    pub fn update_deaths(&mut self, deaths: HashMap<String, usize>) {
        for (name, count) in deaths.into_iter() {
            *self.cur_generation.deaths.entry(name).or_insert(0) += count;
        }
    }
    pub fn start_of_new_generation(
        &mut self,
        population: &[OrganismIndividual],
        config: &EcosystemConfig,
    ) {
        self.cur_generation.start_size = population.len();
        self.population = PopulationStatistics::new(config);
    }
    pub fn end_of_generation(&mut self, population: &[OrganismIndividual]) {
        let mut cur = self.cur_generation.clone();
        cur.end_size = population.len();
        cur.avg_energy = population.iter().map(|i| i.energy).sum::<f32>() / (cur.end_size as f32);
        self.generations.push(cur);
        self.cur_generation = GenerationStatistics::default();
    }
}
