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
    // Mean pos X
    pub total_position_x: f32,
    pub total_position_y: f32,
}
impl SpeciesStatistic {
    pub fn inline_sprint(&self) -> String {
        format!(
            "Size:{:5} Energy:{:9.0} Generation:{:-4} Mean_Pos:({:9.3},{:9.3})",
            self.size,
            self.energy_total,
            if let Some(generation) = self.generation {
                generation.to_string()
            } else {
                "N/A".to_string()
            },
            self.total_position_x / (self.size as f32),
            self.total_position_y / (self.size as f32),
        )
    }
}
#[derive(Reflect, Serialize, Deserialize, Debug)]
pub struct SpeciesStatistics {
    pub name: String,
    pub accumulation: Vec<(u32, SpeciesStatistic)>,
}
impl SpeciesStatistics {
    pub fn new(name: String) -> Self {
        Self {
            name,
            accumulation: vec![],
        }
    }
    pub fn inline_sprint(&self) -> String {
        if let Some(stat) = self.last() {
            format!("{:10} - {}", self.name, stat.inline_sprint())
        } else {
            String::new()
        }
    }
    pub fn add(&mut self, step: u32, stat: SpeciesStatistic) {
        self.accumulation.push((step, stat));
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
    pub fn sprint(&self, cur_step: u32) -> String {
        let mut s = String::new();
        s.push_str(&format!("Steps: {:6}\n", cur_step));
        for stat in self.organisms.values() {
            s.push_str(&format!("  {}\n", stat.inline_sprint()));
        }
        s
    }
}

pub fn statistics_accumulation(
    ecosystem_statistics: ResMut<EcosystemStatistics>,
    ecosystem_runtime: Res<EcosystemRuntime>,
    config: Res<EcosystemConfig>,
    organisms: Query<(&Organism, &Body, &Position)>,
) {
    if ecosystem_runtime.steps >= config.statistics_aggregation_start
        && ecosystem_runtime.steps % config.statistics_aggregation_rate == 0
    {
        // Update current statistics
        accumulate_statistics(ecosystem_statistics, ecosystem_runtime, organisms);
    }
}

pub fn accumulate_statistics(
    mut ecosystem_statistics: ResMut<EcosystemStatistics>,
    ecosystem_runtime: Res<EcosystemRuntime>,
    organisms: Query<(&Organism, &Body, &Position)>,
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
    for (organism, body, position) in organisms.iter() {
        let stat = current_stats
            .get_mut(&organism.species())
            .expect("Found an Organism with a SpeciesId not in EcosystemRuntime.generation");
        stat.size += 1;
        stat.energy_total += body.energy();
        stat.total_position_x += position.x;
        stat.total_position_y += position.y;
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
