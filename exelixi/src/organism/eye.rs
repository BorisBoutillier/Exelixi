use crate::prelude::*;

use std::f32::consts::PI;

#[derive(Debug, Component)]
pub struct Eye {
    pub fov_range: f32,
    pub fov_angle: f32,
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
        let fov_angle = match config.organisms.eye_fov_angle {
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
            fov_angle,
            n_sectors: n_cells as usize,
            n_cells: n_cells as usize,
            energy_cost: compure_energy_cost(
                fov_range,
                fov_angle,
                config.organisms.eye_energy_cost,
            ),
        }
    }
    pub fn from_genes(genes: impl IntoIterator<Item = f32>, config: &SimulationConfig) -> Self {
        let mut genes = genes.into_iter();
        let fov_angle = match config.organisms.eye_fov_angle {
            ConfigValue::Fixed(v) => v,
            ConfigValue::Gene { min, max } => {
                let gene = genes.next().expect("Missing gene for the fov_angle");
                gene.clamp(min, max)
            }
            _ => panic!(),
        };
        let fov_range = match config.organisms.eye_fov_range {
            ConfigValue::Fixed(v) => v,
            ConfigValue::Gene { min, max } => {
                let gene = genes.next().expect("Missing gene for the fov_range");
                gene.clamp(min, max)
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
            fov_angle,
            n_sectors: n_cells as usize,
            n_cells: n_cells as usize,
            energy_cost: compure_energy_cost(
                fov_range,
                fov_angle,
                config.organisms.eye_energy_cost,
            ),
        }
    }
    pub fn as_chromosome(&self, config: &SimulationConfig) -> ga::Chromosome {
        let mut genes = vec![];
        match config.organisms.eye_fov_angle {
            ConfigValue::Fixed(_) => (),
            ConfigValue::Gene { min: _, max: _ } => genes.push(self.fov_angle),
            _ => panic!(),
        }
        match config.organisms.eye_fov_range {
            ConfigValue::Fixed(_) => (),
            ConfigValue::Gene { min: _, max: _ } => genes.push(self.fov_range),
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
        transform: &Transform,
        food_transforms: &[&Transform],
        organism_transforms: &[&Transform],
        config: &SimulationConfig,
    ) -> Vec<f32> {
        let mut sensors = vec![];
        if self.see_foods {
            sensors.extend(self.sense_objects(transform, food_transforms));
        }
        if self.see_walls {
            sensors.extend(self.sense_walls(transform, config));
        }
        if self.see_organisms {
            sensors.extend(self.sense_objects(transform, organism_transforms));
        }
        assert_eq!(sensors.len(), self.n_sensors());
        sensors
    }
    // process the sensors value for each eye cell associated to the given
    // transforms
    pub fn sense_objects(
        &self,
        transform: &Transform,
        object_transforms: &[&Transform],
    ) -> Vec<f32> {
        let mut cells = vec![0.0; self.n_cells];
        for object_transform in object_transforms {
            let vec = transform.translation - object_transform.translation;
            let dist = vec.length();
            if dist > self.fov_range {
                continue;
            }
            let (axis, mut rotation_angle) = transform.rotation.to_axis_angle();
            if axis.z < 0.0 {
                rotation_angle = -rotation_angle;
            }
            let angle = Vec3::X.angle_between(vec) - rotation_angle;
            let angle = (angle % (2.0 * PI)) - PI; // = wrap(-PI,PI)
            if angle < -self.fov_angle / 2.0 || angle > self.fov_angle / 2.0 {
                continue;
            }

            let angle = angle + self.fov_angle / 2.0;
            let sector = angle / self.fov_angle;
            let sector = sector * (self.n_sectors as f32);
            let sector = (sector as usize).min(self.n_sectors - 1);

            let energy = (self.fov_range - dist) / self.fov_range;

            cells[sector] += energy;
        }
        cells
    }
    pub fn sense_walls(&self, transform: &Transform, config: &SimulationConfig) -> Vec<f32> {
        let half_width = config.environment.width / 2.0;
        let half_height = config.environment.height / 2.0;
        let (axis, mut rotation_angle) = transform.rotation.to_axis_angle();
        if axis.z < 0.0 {
            rotation_angle = -rotation_angle;
        }
        let angle_incr = self.fov_angle / (self.n_cells as f32);
        // Starting from the lowest fov line we evaluate the distance of the closest wall intersect on this line.
        // and compute an energy
        // Doing it for each cell boundary so cells.length()+1 lines.
        let start_angle = rotation_angle - self.fov_angle / 2.0 + angle_incr / 2.0;
        (0..self.n_cells)
            .map(|i| {
                let angle = start_angle + (i as f32 * angle_incr);
                let mut dist = f32::INFINITY;
                let dist_right = (half_width - transform.translation.x) / angle.cos();
                if dist_right > 0.0 {
                    dist = dist.min(dist_right);
                }
                let dist_left = (-half_width - transform.translation.x) / angle.cos();
                if dist_left > 0.0 {
                    dist = dist.min(dist_left);
                }
                let dist_top = (half_height - transform.translation.y) / angle.sin();
                if dist_top > 0.0 {
                    dist = dist.min(dist_top);
                }
                let dist_bottom = (-half_height - transform.translation.y) / angle.sin();
                if dist_bottom > 0.0 {
                    dist = dist.min(dist_bottom);
                }
                ((self.fov_range - dist) / self.fov_range).max(0.0)
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

fn compure_energy_cost(fov_range: f32, fov_angle: f32, energy_per_area: f32) -> f32 {
    (PI * fov_range.powi(2) * 2.0 * PI / fov_angle) * energy_per_area / (PI * 150.0 * 150.0)
}
