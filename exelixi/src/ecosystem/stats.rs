use std::collections::HashMap;

use crate::ecosystem::*;

#[derive(Default, Debug, Clone)]
pub struct OrganismStatistic {
    // Current generation for this organism.
    // Applicable only for organism with GenerationEvolution reproduction
    pub generation: Option<u32>,
    // Current organism population size
    pub size: usize,
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

#[derive(Resource, Debug, Default)]
pub struct EcosystemStatistics {
    pub accumulation: Vec<(u32, HashMap<String, OrganismStatistic>)>,
    pub current: HashMap<String, OrganismStatistic>,
}

impl EcosystemStatistics {
    pub fn new(config: &EcosystemConfig) -> Self {
        let mut s = Self::default();
        s.reset_current(config);
        s
    }
    pub fn reset_current(&mut self, config: &EcosystemConfig) {
        self.current.clear();
        for organism_config in config.organisms.iter() {
            self.current
                .insert(organism_config.name.clone(), OrganismStatistic::default());
        }
    }
    pub fn update_eaten(&mut self, eaten: HashMap<String, u32>) {
        for (name, count) in eaten.into_iter() {
            self.current
                .entry(name)
                .and_modify(|stat| stat.eaten += count);
        }
    }
    pub fn update_out_of_energy(&mut self, out_of_energy: HashMap<String, u32>) {
        for (name, count) in out_of_energy.into_iter() {
            self.current
                .entry(name)
                .and_modify(|stat| stat.out_of_energy += count);
        }
    }
    pub fn sprint(&self, cur_step: u32) -> String {
        let mut s = String::new();
        s.push_str(&format!("Steps: {:6}\n", cur_step));
        for (name, stat) in self.current.iter() {
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
) {
    if simulation.steps % config.statistics.aggregation_rate.unwrap() == 0 {
        // Update current statistics
        let mut size = HashMap::new();
        let mut energy = HashMap::new();
        for (organism, body) in organisms.iter() {
            *size.entry(organism.name.clone()).or_insert(0) += 1;
            *energy.entry(organism.name.clone()).or_insert(0.) += body.energy();
        }
        for (name, size) in size.into_iter() {
            if let Some(stat) = ecosystem_statistics.current.get_mut(&name) {
                stat.size = size;
                stat.energy_avg = energy[&name] / size as f32;
            }
        }
        // Print in console
        println!("{}", ecosystem_statistics.sprint(simulation.steps));
        // Store current statistics, and prepare new one
        let current = ecosystem_statistics.current.clone();
        ecosystem_statistics
            .accumulation
            .push((simulation.steps, current));
        ecosystem_statistics.reset_current(&config);
    }
}
