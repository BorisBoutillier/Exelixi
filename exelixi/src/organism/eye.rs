use crate::prelude::*;

use std::f32::consts::PI;

#[derive(Debug, Component)]
pub struct Eye {
    pub fov_range: i32,
    pub fov_angle_crad: i32,
    pub n_sectors: usize,
    pub n_cells: usize,
    pub see_foods: bool,
    pub see_walls: bool,
    pub see_organisms: bool,
    energy_cost: f32,
}

impl Eye {
    pub fn random(rng: &mut dyn RngCore, config: &SimulationConfig) -> Self {
        let (_n_sectors, n_cells) = match config.organisms.n_eye_cells {
            ConfigValue::Fixed(v) => (v, v),
            ConfigValue::Gene { min, max } => (rng.gen_range(min..=max), max),
            _ => panic!(),
        };
        let fov_angle_crad = match config.organisms.eye_fov_angle_crad {
            ConfigValue::Fixed(v) => v,
            ConfigValue::Gene { min, max } => rng.gen_range(min..=max),
            _ => panic!(),
        };
        let fov_range = match config.organisms.eye_fov_range {
            ConfigValue::Fixed(v) => v,
            ConfigValue::Gene { min, max } => rng.gen_range(min..=max),
            _ => panic!(),
        };
        Self {
            see_walls: config.environment.wall && config.organisms.see_walls,
            see_foods: config.organisms.see_foods,
            see_organisms: config.organisms.see_organisms,
            fov_range,
            fov_angle_crad,
            n_sectors: n_cells as usize,
            n_cells: n_cells as usize,
            energy_cost: compute_energy_cost(
                fov_range,
                fov_angle_crad,
                config.organisms.eye_energy_cost,
            ),
        }
    }
    pub fn from_genes(genes: impl IntoIterator<Item = f32>, config: &SimulationConfig) -> Self {
        let mut genes = genes.into_iter();
        let fov_angle_crad = match config.organisms.eye_fov_angle_crad {
            ConfigValue::Fixed(v) => v,
            ConfigValue::Gene { min, max } => {
                let gene = genes.next().expect("Missing gene for the fov_angle");
                (gene as i32).clamp(min, max)
            }
            _ => panic!(),
        };
        let fov_range = match config.organisms.eye_fov_range {
            ConfigValue::Fixed(v) => v,
            ConfigValue::Gene { min, max } => {
                let gene = genes.next().expect("Missing gene for the fov_range");
                (gene as i32).clamp(min, max)
            }
            _ => panic!(),
        };
        let (_n_sectors, n_cells) = match config.organisms.n_eye_cells {
            ConfigValue::Fixed(v) => (v, v),
            ConfigValue::Gene { min, max } => {
                let gene = genes.next().expect("Missing gene for the n_eye_cells");
                ((gene as u8).clamp(min, max), max)
            }
            _ => panic!(),
        };
        Self {
            see_walls: config.environment.wall && config.organisms.see_walls,
            see_foods: config.organisms.see_foods,
            see_organisms: config.organisms.see_organisms,
            fov_range,
            fov_angle_crad,
            n_sectors: n_cells as usize,
            n_cells: n_cells as usize,
            energy_cost: compute_energy_cost(
                fov_range,
                fov_angle_crad,
                config.organisms.eye_energy_cost,
            ),
        }
    }
    pub fn as_chromosome(&self, config: &SimulationConfig) -> ga::Chromosome {
        let mut genes = vec![];
        match config.organisms.eye_fov_angle_crad {
            ConfigValue::Fixed(_) => (),
            ConfigValue::Gene { min: _, max: _ } => genes.push(self.fov_angle_crad as f32),
            _ => panic!(),
        }
        match config.organisms.eye_fov_range {
            ConfigValue::Fixed(_) => (),
            ConfigValue::Gene { min: _, max: _ } => genes.push(self.fov_range as f32),
            _ => panic!(),
        }
        match config.organisms.n_eye_cells {
            ConfigValue::Fixed(_) => (),
            ConfigValue::Gene { min: _, max: _ } => genes.push(self.n_sectors as f32),
            _ => panic!(),
        }
        genes.into_iter().collect()
    }
    pub fn process_vision(
        &self,
        position: &Position,
        food_positions: &[&Position],
        organism_positions: &[&Position],
        config: &SimulationConfig,
    ) -> Vec<f32> {
        let mut sensors = vec![];
        if self.see_foods {
            sensors.extend(self.sense_objects(position, food_positions));
        }
        if self.see_walls {
            sensors.extend(self.sense_walls(position, config));
        }
        if self.see_organisms {
            sensors.extend(self.sense_objects(position, organism_positions));
        }
        assert_eq!(sensors.len(), self.n_sensors());
        sensors
    }
    // process the sensors value for each eye cell associated to the given
    // transforms
    pub fn sense_objects(&self, position: &Position, object_positions: &[&Position]) -> Vec<f32> {
        let mut cells = vec![0.0; self.n_cells];
        //println!("SENSE for {position:?}");
        for object_position in object_positions {
            let dist_pow2 =
                (position.x - object_position.x).pow(2) + (position.y - object_position.y).pow(2);
            if dist_pow2 > self.fov_range.pow(2) {
                continue;
            }
            let view_angle_crad = (f32::atan2(
                (object_position.y - position.y) as f32,
                (object_position.x - position.x) as f32,
            ) * 100.0) as i32;
            //println!("    FOOD {:?} -> {}", object_position, view_angle_crad);
            if view_angle_crad < -self.fov_angle_crad / 2
                || view_angle_crad > self.fov_angle_crad / 2
            {
                continue;
            }

            let sector_angle_crad = self.fov_angle_crad / self.n_sectors as i32;
            let sector = (view_angle_crad + self.fov_angle_crad / 2) / sector_angle_crad;
            let sector = (sector as usize).min(self.n_sectors - 1);

            let energy =
                (self.fov_range as f32 - (dist_pow2 as f32).sqrt()) / self.fov_range as f32;

            cells[sector] += energy;
        }
        //println!("  -> Cells: {cells:?}");
        cells
    }
    pub fn sense_walls(&self, position: &Position, config: &SimulationConfig) -> Vec<f32> {
        let half_width = config.environment.width / 2;
        let half_height = config.environment.height / 2;
        let angle_incr_crad = self.fov_angle_crad / self.n_cells as i32;
        // Starting from the lowest fov line we evaluate the distance of the closest wall intersect on this line.
        // and compute an energy
        // Doing it for each cell boundary so cells.length()+1 lines.
        let start_angle_crad =
            position.angle_crad() - self.fov_angle_crad / 2 + angle_incr_crad / 2;
        (0..self.n_cells as i32)
            .map(|i| {
                let angle = (start_angle_crad + (i * angle_incr_crad)) as f32 / 100.0;
                let mut dist = f32::INFINITY;
                let dist_right = (half_width - position.x) as f32 / angle.cos();
                if dist_right > 0.0 {
                    dist = dist.min(dist_right);
                }
                let dist_left = (-half_width - position.x) as f32 / angle.cos();
                if dist_left > 0.0 {
                    dist = dist.min(dist_left);
                }
                let dist_top = (half_height - position.y) as f32 / angle.sin();
                if dist_top > 0.0 {
                    dist = dist.min(dist_top);
                }
                let dist_bottom = (-half_height - position.y) as f32 / angle.sin();
                if dist_bottom > 0.0 {
                    dist = dist.min(dist_bottom);
                }
                ((self.fov_range as f32 - dist) / self.fov_range as f32).max(0.0)
            })
            .collect::<Vec<_>>()
    }
    pub fn energy_cost(&self) -> f32 {
        self.energy_cost
    }
    // Return the number of sensors associated with this eye configuration
    pub fn n_sensors(&self) -> usize {
        let mut n_sensors = 0;
        if self.see_foods {
            n_sensors += self.n_cells;
        }
        if self.see_walls {
            n_sensors += self.n_cells;
        }
        if self.see_organisms {
            n_sensors += self.n_cells;
        }
        n_sensors
    }
}

fn compute_energy_cost(fov_range: i32, fov_angle_crad: i32, energy_per_area: f32) -> f32 {
    (PI * fov_range.pow(2) as f32 * 2.0 * PI / (fov_angle_crad as f32 / 100.0)) * energy_per_area
        / (PI * 150.0 * 150.0)
}
