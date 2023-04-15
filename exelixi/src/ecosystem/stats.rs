use std::collections::{BTreeMap, HashMap};

use crate::ecosystem::*;

#[derive(Default, Debug, Clone)]
pub struct OrganismStatistic {
    // Current generation for this organism.
    // Applicable only for organism with GenerationEvolution reproduction
    pub generation: Option<u32>,
    // Current organism population size
    pub size: u32,
    // Current organism energy average
    pub energy_avg: f32,
    // Number of dead organism by out_of_energy since last Step
    pub out_of_energy: u32,
    // Number of eaten organism since last Step
    pub eaten: u32,
}
impl OrganismStatistic {
    pub fn inline_sprint(&self) -> String {
        format!(
            "Size: {:4}, Energy: {:6.0}, Deaths:{:4}, Eaten:{:4}, Generation:{}",
            self.size,
            self.energy_avg,
            self.out_of_energy,
            self.eaten,
            if let Some(generation) = self.generation {
                generation.to_string()
            } else {
                "N/A".to_string()
            }
        )
    }
}
#[derive(Debug)]
pub struct OrganismStatistics {
    pub current: OrganismStatistic,
    pub accumulation: Vec<(u32, OrganismStatistic)>,
}
impl OrganismStatistics {
    pub fn new() -> Self {
        Self {
            current: OrganismStatistic::default(),
            accumulation: vec![],
        }
    }
    pub fn set_current(&mut self, size: u32, energy_avg: f32) {
        self.current.size = size;
        self.current.energy_avg = energy_avg;
    }
    pub fn set_generation(&mut self, generation: u32) {
        self.current.generation = Some(generation);
    }
    pub fn add_eaten(&mut self, count: u32) {
        self.current.eaten += count;
    }
    pub fn add_out_of_energy(&mut self, count: u32) {
        self.current.out_of_energy += count;
    }
    pub fn inline_sprint(&self) -> String {
        self.current.inline_sprint()
    }
    pub fn store_at_step(&mut self, step: u32) {
        self.accumulation.push((step, self.current.clone()));
        self.current = OrganismStatistic::default();
    }
    pub fn last_stored(&self) -> Option<&OrganismStatistic> {
        self.accumulation.last().map(|(_, stat)| stat)
    }
}

#[derive(Resource, Debug, Default)]
pub struct EcosystemStatistics {
    pub organisms: BTreeMap<String, OrganismStatistics>,
}

impl EcosystemStatistics {
    pub fn new(config: &EcosystemConfig) -> Self {
        let mut organisms = BTreeMap::new();
        for organism_config in config.organisms.iter() {
            organisms.insert(organism_config.name.clone(), OrganismStatistics::new());
        }
        Self { organisms }
    }
    pub fn update_eaten(&mut self, eaten: HashMap<String, u32>) {
        for (name, count) in eaten.into_iter() {
            if let Some(stat) = self.organisms.get_mut(&name) {
                stat.add_eaten(count)
            }
        }
    }
    pub fn update_out_of_energy(&mut self, out_of_energy: HashMap<String, u32>) {
        for (name, count) in out_of_energy.into_iter() {
            if let Some(stat) = self.organisms.get_mut(&name) {
                stat.add_out_of_energy(count)
            }
        }
    }
    pub fn sprint(&self, cur_step: u32) -> String {
        let mut s = String::new();
        s.push_str(&format!("Steps: {:6}\n", cur_step));
        for (name, stat) in self.organisms.iter() {
            s.push_str(&format!("    {name:10} - {}\n", stat.inline_sprint()));
        }
        s
    }
}

pub fn statistics_accumulation(
    simulation: Res<Simulation>,
    config: Res<EcosystemConfig>,
    mut ecosystem_statistics: ResMut<EcosystemStatistics>,
    organisms: Query<(&Organism, &Body)>,
    generation_evolutions: Res<GenerationEvolutions>,
) {
    if simulation.steps % config.statistics.aggregation_rate.unwrap() == 0 {
        // Update current statistics
        let mut size = HashMap::new();
        let mut energy = HashMap::new();
        for (organism, body) in organisms.iter() {
            *size.entry(organism.name().to_string()).or_insert(0) += 1;
            *energy.entry(organism.name().to_string()).or_insert(0.) += body.energy();
        }
        for (name, size) in size.into_iter() {
            if let Some(stat) = ecosystem_statistics.organisms.get_mut(&name) {
                stat.set_current(size, energy[&name] / size as f32);
            }
        }
        for (name, generation_evolution) in generation_evolutions.per_name.iter() {
            if let Some(stat) = ecosystem_statistics.organisms.get_mut(name) {
                stat.set_generation(generation_evolution.current_generation);
            }
        }
        // Print in console
        println!("{}", ecosystem_statistics.sprint(simulation.steps));
        // Store current statistics, and prepare new one
        for stat in ecosystem_statistics.organisms.values_mut() {
            stat.store_at_step(simulation.steps);
        }
    }
}
