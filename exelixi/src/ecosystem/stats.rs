use std::collections::BTreeMap;

use crate::ecosystem::*;

#[derive(Reflect, Serialize, Deserialize, Default, Debug, Clone)]
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
}
impl SpeciesStatistic {
    pub fn inline_sprint(&self) -> String {
        format!(
            "Size: {:4}, Energy: {:6.0}, Deaths:{:4}, Generation:{}",
            self.size,
            self.energy_total,
            self.out_of_energy,
            if let Some(generation) = self.generation {
                generation.to_string()
            } else {
                "N/A".to_string()
            }
        )
    }
}
#[derive(Reflect, Serialize, Deserialize, Debug)]
pub struct SpeciesStatistics {
    pub name: String,
    pub out_of_energy_count: u32,
    pub accumulation: Vec<(u32, SpeciesStatistic)>,
}
impl SpeciesStatistics {
    pub fn new(name: String) -> Self {
        Self {
            name,
            out_of_energy_count: 0,
            accumulation: vec![],
        }
    }
    pub fn add_out_of_energy(&mut self, count: u32) {
        self.out_of_energy_count += count;
    }
    pub fn inline_sprint(&self) -> String {
        if let Some(stat) = self.last() {
            format!("{:19} - {}", self.name, stat.inline_sprint())
        } else {
            String::new()
        }
    }
    pub fn add(&mut self, step: u32, stat: SpeciesStatistic) {
        self.accumulation.push((step, stat));
        self.out_of_energy_count = 0;
    }
    pub fn last(&self) -> Option<&SpeciesStatistic> {
        self.accumulation.last().map(|(_, stat)| stat)
    }
}

#[derive(Resource, Reflect, Serialize, Deserialize, Debug, Default)]
#[reflect(Resource)]
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
    ecosystem_statistics: ResMut<EcosystemStatistics>,
    ecosystem_runtime: Res<EcosystemRuntime>,
    config: Res<EcosystemConfig>,
    organisms: Query<(&Organism, &Body)>,
) {
    if ecosystem_runtime.steps % config.statistics_aggregation_rate == 0 {
        // Update current statistics
        accumulate_statistics(ecosystem_statistics, ecosystem_runtime, organisms);
    }
}

pub fn accumulate_statistics(
    mut ecosystem_statistics: ResMut<EcosystemStatistics>,
    ecosystem_runtime: Res<EcosystemRuntime>,
    organisms: Query<(&Organism, &Body)>,
) {
    let mut current_stats = BTreeMap::new();
    for (species, generation) in ecosystem_runtime.generation.iter() {
        current_stats.insert(
            species,
            SpeciesStatistic {
                generation: Some(*generation),
                ..Default::default()
            },
        );
    }
    for (organism, body) in organisms.iter() {
        let stat = current_stats
            .get_mut(&organism.species())
            .expect("Found an Organism with a SpeciesId not in EcosystemRuntime.generation");
        stat.energy_total += body.energy();
        stat.size += 1;
    }
    for (species, stats) in ecosystem_statistics.organisms.iter_mut() {
        stats.add(
            ecosystem_runtime.steps,
            current_stats
                .remove(species)
                .expect("EcosystemStatistics has a SpeciesId unknown to EcosystemRuntime"),
        );
    }
    // Print in console
    println!("{}", ecosystem_statistics.sprint(ecosystem_runtime.steps));
}
