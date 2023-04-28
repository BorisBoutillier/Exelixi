use std::collections::BTreeMap;

use crate::ecosystem::*;

#[derive(Default, Debug, Clone)]
pub struct SpeciesStatistic {
    // Current generation for this organism.
    // Applicable only for organism with GenerationEvolution reproduction
    pub generation: Option<u32>,
    // Current organism population size
    pub size: u32,
    // Total energy of all organism in this species.
    pub energy_total: f32,
    // Number of dead organism by out_of_energy since last Step
    pub out_of_energy: u32,
    // Number of eaten organism since last Step
    pub eaten: u32,
}
impl SpeciesStatistic {
    pub fn inline_sprint(&self) -> String {
        format!(
            "Size: {:4}, Energy: {:6.0}, Deaths:{:4}, Eaten:{:4}, Generation:{}",
            self.size,
            self.energy_total,
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
pub struct SpeciesStatistics {
    pub name: String,
    pub current: SpeciesStatistic,
    pub accumulation: Vec<(u32, SpeciesStatistic)>,
}
impl SpeciesStatistics {
    pub fn new(name: String) -> Self {
        Self {
            name,
            current: SpeciesStatistic::default(),
            accumulation: vec![],
        }
    }
    pub fn set_current(&mut self, size: u32, energy_total: f32) {
        self.current.size = size;
        self.current.energy_total = energy_total;
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
        format!("{:19} - {}", self.name, self.current.inline_sprint())
    }
    pub fn store_at_step(&mut self, step: u32) {
        self.accumulation.push((step, self.current.clone()));
        self.current = SpeciesStatistic::default();
    }
    pub fn last_stored(&self) -> Option<&SpeciesStatistic> {
        self.accumulation.last().map(|(_, stat)| stat)
    }
}

#[derive(Resource, Debug, Default)]
pub struct EcosystemStatistics {
    pub organisms: BTreeMap<SpeciesId, SpeciesStatistics>,
}

impl EcosystemStatistics {
    pub fn new(config: &EcosystemConfig) -> Self {
        let mut organisms = BTreeMap::new();
        for (species_id, species_config) in config.species.iter() {
            organisms.insert(
                *species_id,
                SpeciesStatistics::new(species_config.name.clone()),
            );
        }
        Self { organisms }
    }
    pub fn update_eaten(&mut self, eaten: BTreeMap<SpeciesId, u32>) {
        for (species_id, count) in eaten {
            if let Some(stat) = self.organisms.get_mut(&species_id) {
                stat.add_eaten(count)
            }
        }
    }
    pub fn update_out_of_energy(&mut self, out_of_energy: BTreeMap<SpeciesId, u32>) {
        for (name, count) in out_of_energy.into_iter() {
            if let Some(stat) = self.organisms.get_mut(&name) {
                stat.add_out_of_energy(count)
            }
        }
    }
    pub fn sprint(&self, cur_step: u32) -> String {
        let mut s = String::new();
        s.push_str(&format!("Steps: {:6}\n", cur_step));
        for stat in self.organisms.values() {
            s.push_str(&format!("    {}\n", stat.inline_sprint()));
        }
        s
    }
}

pub fn statistics_accumulation(
    ecosystem: Res<EcosystemRuntime>,
    config: Res<EcosystemConfig>,
    mut ecosystem_statistics: ResMut<EcosystemStatistics>,
    organisms: Query<(&Organism, &Body)>,
) {
    if ecosystem.steps % config.statistics_aggregation_rate == 0 {
        // Update current statistics
        let mut size = BTreeMap::new();
        let mut energy = BTreeMap::new();
        for (organism, body) in organisms.iter() {
            *size.entry(organism.species()).or_insert(0) += 1;
            *energy.entry(organism.species()).or_insert(0.) += body.energy();
        }
        for (name, size) in size.into_iter() {
            if let Some(stat) = ecosystem_statistics.organisms.get_mut(&name) {
                stat.set_current(size, energy[&name]);
            }
        }
        for (species, generation) in ecosystem.generation.iter() {
            if let Some(stat) = ecosystem_statistics.organisms.get_mut(species) {
                stat.set_generation(*generation);
            }
        }
        // Print in console
        println!("{}", ecosystem_statistics.sprint(ecosystem.steps));
        // Store current statistics, and prepare new one
        for stat in ecosystem_statistics.organisms.values_mut() {
            stat.store_at_step(ecosystem.steps);
        }
    }
}
