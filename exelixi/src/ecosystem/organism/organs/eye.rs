use crate::ecosystem::*;
use lib_genetic_algorithm as ga;

use std::f32::consts::PI;

#[derive(Debug, Component)]
pub struct Eye {
    pub fov_range: f32,
    pub fov_angle: f32,
    pub n_sectors: usize,
    pub n_cells: usize,
    visible: Vec<String>,
    energy_cost: f32,
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
            visible: config.visible.clone(),
            energy_cost: compute_energy_cost(fov_range, fov_angle, config.energy_cost),
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
            visible: config.visible.clone(),
            energy_cost: compute_energy_cost(fov_range, fov_angle, config.energy_cost),
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
    pub fn process_vision(
        &self,
        position: &Position,
        positions: &[(&Position, &Organism)],
    ) -> Vec<f32> {
        let mut sensors = vec![];
        let visible_positions = positions
            .iter()
            .filter(|(_, o)| self.visible.contains(&o.name))
            .map(|(p, _)| *p)
            .collect::<Vec<_>>();
        sensors.extend(self.sense_objects(position, &visible_positions));
        assert_eq!(sensors.len(), self.n_sensors());
        sensors
    }
    // process the sensors value for each eye cell associated to the given
    // transforms
    pub fn sense_objects(&self, position: &Position, object_positions: &[&Position]) -> Vec<f32> {
        let mut cells = vec![0.0; self.n_cells];
        //println!("SENSE for {position:?}");
        for object_position in object_positions {
            let distance_squared = position.distance_squared(object_position);
            if distance_squared > self.fov_range.powi(2) {
                continue;
            }
            let view_angle = position.angle_between(object_position);
            //println!("    FOOD {:?} -> {}", object_position, view_angle);
            if view_angle < -self.fov_angle / 2.0 || view_angle > self.fov_angle / 2.0 {
                continue;
            }

            let sector_angle = self.fov_angle / self.n_sectors as f32;
            let sector = (view_angle + self.fov_angle / 2.0) / sector_angle;
            let sector = (sector as usize).min(self.n_sectors - 1);

            let energy = (self.fov_range - distance_squared.sqrt()) / self.fov_range;

            cells[sector] += energy;
        }
        //println!("  -> Cells: {cells:?}");
        cells
    }
    pub fn energy_cost(&self) -> f32 {
        self.energy_cost
    }
    // Return the number of sensors associated with this eye configuration
    pub fn n_sensors(&self) -> usize {
        self.n_cells
    }
}

fn compute_energy_cost(fov_range: f32, fov_angle: f32, energy_per_area: f32) -> f32 {
    (PI * fov_range.powi(2) * 2.0 * PI / fov_angle) * energy_per_area / (PI * 150.0 * 150.0)
}
