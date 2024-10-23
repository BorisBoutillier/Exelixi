use crate::ecosystem::*;
use lib_genetic_algorithm as ga;

use std::{collections::BTreeMap, f32::consts::PI};

use super::traits::Sensor;

#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
pub struct Eye {
    pub fov_range: f32,
    pub fov_angle: f32,
    pub n_sectors: usize,
    pub n_cells: usize,
    visible: Vec<SpeciesId>,
    energy_cost: f32,
    pub cell_sensors: CellSensors,
    sensors: Vec<f32>,
}

impl Eye {
    pub fn random(rng: &mut dyn RngCore, config: &EyeConfig) -> Self {
        let (_n_sectors, n_cells) = match config.n_cells {
            ConfigValue::Fixed(v) => (v, v),
            ConfigValue::Gene { min, max } => (rng.gen_range(min..=max), max),
            _ => panic!(),
        };
        let fov_angle = match config.fov_angle {
            ConfigValue::Fixed(v) => v,
            ConfigValue::Gene { min, max } => rng.gen_range(min..=max),
            _ => panic!(),
        };
        let fov_range = match config.fov_range {
            ConfigValue::Fixed(v) => v,
            ConfigValue::Gene { min, max } => rng.gen_range(min..=max),
            _ => panic!(),
        };
        Self {
            fov_range,
            fov_angle,
            n_sectors: n_cells as usize,
            n_cells: n_cells as usize,
            visible: config.visible_species.clone(),
            energy_cost: Eye::compute_energy_cost(fov_range, fov_angle, config.energy_cost),
            cell_sensors: config.cell_sensors,
            sensors: vec![
                0.0;
                n_cells as usize
                    * config.cell_sensors.n_sensors(config.visible_species.len())
            ],
        }
    }
    pub fn from_genes(genes: impl IntoIterator<Item = f32>, config: &EyeConfig) -> Self {
        let mut genes = genes.into_iter();
        let fov_angle = match config.fov_angle {
            ConfigValue::Fixed(v) => v,
            ConfigValue::Gene { min, max } => {
                let gene = genes.next().expect("Missing gene for the fov_angle");
                gene.clamp(min, max)
            }
            _ => panic!(),
        };
        let fov_range = match config.fov_range {
            ConfigValue::Fixed(v) => v,
            ConfigValue::Gene { min, max } => {
                let gene = genes.next().expect("Missing gene for the fov_range");
                gene.clamp(min, max)
            }
            _ => panic!(),
        };
        let (_n_sectors, n_cells) = match config.n_cells {
            ConfigValue::Fixed(v) => (v, v),
            ConfigValue::Gene { min, max } => {
                let gene = genes.next().expect("Missing gene for the n_eye_cells");
                ((gene as u8).clamp(min, max), max)
            }
            _ => panic!(),
        };
        Self {
            fov_range,
            fov_angle,
            n_sectors: n_cells as usize,
            n_cells: n_cells as usize,
            visible: config.visible_species.clone(),
            energy_cost: Eye::compute_energy_cost(fov_range, fov_angle, config.energy_cost),
            cell_sensors: config.cell_sensors,
            sensors: vec![
                0.0;
                n_cells as usize
                    * config.cell_sensors.n_sensors(config.visible_species.len())
            ],
        }
    }
    pub fn as_chromosome(&self, config: &EyeConfig) -> ga::Chromosome {
        let mut genes = vec![];
        match config.fov_angle {
            ConfigValue::Fixed(_) => (),
            ConfigValue::Gene { min: _, max: _ } => genes.push(self.fov_angle),
            _ => panic!(),
        }
        match config.fov_range {
            ConfigValue::Fixed(_) => (),
            ConfigValue::Gene { min: _, max: _ } => genes.push(self.fov_range),
            _ => panic!(),
        }
        match config.n_cells {
            ConfigValue::Fixed(_) => (),
            ConfigValue::Gene { min: _, max: _ } => genes.push(self.n_sectors as f32),
            _ => panic!(),
        }
        genes.into_iter().collect()
    }
    // process the sensors value for each eye cell associated to the given transforms
    // Each eye sector only seen the closest organism.
    // The sensor value for each sector is (1-distance_pct)*energy_pct of the closest organism that this sector can see.
    // Meaning the closer and the more energy this organism has, the higher the value, range [0..1]
    pub fn sense_objects(
        &mut self,
        position: &Position,
        organisms: &BTreeMap<SpeciesId, Vec<(&Position, /*energy_pct*/ f32, /*hue*/ f32)>>,
    ) {
        let default = organisms
            .keys()
            .map(|species| (*species, (0.0, 0.0, 0.0)))
            .collect::<BTreeMap<_, _>>();
        let mut closest_per_cell = vec![default; self.n_cells];
        for (species, details) in organisms.iter() {
            for &(organism_position, organism_energy_pct, organism_hue) in details {
                let distance_squared = position.distance_squared(organism_position);
                assert!(
                    distance_squared <= self.fov_range.powi(2),
                    "Positions should already have been filtered by distance"
                );
                let view_angle = (position.angle() - position.angle_between(organism_position)
                    + PI)
                    .rem_euclid(2. * PI)
                    - PI;
                if view_angle < -self.fov_angle / 2.0 || view_angle > self.fov_angle / 2.0 {
                    continue;
                }

                let sector_angle = self.fov_angle / self.n_sectors as f32;
                let sector = (view_angle + self.fov_angle / 2.0) / sector_angle;
                let sector = (sector as usize).min(self.n_sectors - 1);

                let distance_pct = (self.fov_range - distance_squared.sqrt()) / self.fov_range;
                let &(other_distance_pct, _, _) = closest_per_cell[sector].get(species).unwrap();
                if distance_pct > other_distance_pct {
                    closest_per_cell[sector]
                        .insert(*species, (distance_pct, organism_energy_pct, organism_hue));
                }
            }
        }
        self.sensors = closest_per_cell
            .iter()
            .flat_map(|per_species| self.cell_sensors.sensors(per_species))
            .collect::<Vec<_>>();
    }

    // Return the number of sensors per cell of this eye
    pub fn n_cell_sensors(&self) -> usize {
        self.cell_sensors.n_sensors(self.visible.len())
    }
    fn compute_energy_cost(fov_range: f32, fov_angle: f32, energy_per_area: f32) -> f32 {
        (PI * fov_range.powi(2) * 2.0 * PI / fov_angle) * energy_per_area / (PI * 150.0 * 150.0)
    }
}
impl EnergyConsumer for Eye {
    fn energy_consumed(&self) -> f32 {
        self.energy_cost
    }
}

impl Sensor for Eye {
    fn n_sensors(&self) -> usize {
        self.n_cells * self.n_cell_sensors()
    }

    fn sensors(&self) -> Vec<f32> {
        self.sensors.clone()
    }
}

pub fn eye_processing(
    mut organisms_with_eye: Query<(&mut Eye, &Position)>,
    kdtree: Res<OrganismKdTree>,
    organisms: Query<(&Organism, &Body)>,
) {
    for (mut eye, position) in organisms_with_eye.iter_mut() {
        let mut visible = BTreeMap::new();
        for species in eye.visible.iter() {
            let mut details = vec![];
            for entry in kdtree.per_species[species].within_radius(
                &KdTreeEntry::new(position, Entity::PLACEHOLDER),
                eye.fov_range,
            ) {
                // Organism in the KdTree can have been eaten within this step
                if let Ok((organism, body)) = organisms.get(entry.entity) {
                    details.push((&entry.position, body.energy_pct(), organism.hue()))
                }
            }
            visible.insert(*species, details);
        }
        eye.sense_objects(position, &visible);
        assert_eq!(eye.sensors.len(), eye.n_sensors());
    }
}
