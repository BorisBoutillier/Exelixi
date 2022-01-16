use crate::prelude::*;

use std::f32::consts::{FRAC_PI_4, PI};

const FOV_RANGE: f32 = 150.0;
const FOV_ANGLE: f32 = PI + FRAC_PI_4;
const CELLS: usize = 9;

#[derive(Debug, Component)]
pub struct Eye {
    pub fov_range: f32,
    pub fov_angle: f32,
    pub cells: usize,
    pub see_foods: bool,
    pub see_walls: bool,
}

impl Eye {
    pub fn new(
        fov_range: f32,
        fov_angle: f32,
        cells: usize,
        see_foods: bool,
        see_walls: bool,
    ) -> Self {
        assert!(fov_range > 0.0);
        assert!(fov_angle > 0.0);
        assert!(cells > 0);
        Self {
            fov_range,
            fov_angle,
            cells,
            see_foods,
            see_walls,
        }
    }
    pub fn process_vision(
        &self,
        transform: &Transform,
        foods_transform: &[&Transform],
        config: &SimulationConfig,
    ) -> Vec<f32> {
        let mut sensors = vec![];
        if self.see_foods {
            let mut cells = vec![0.0; self.cells];
            for food_transform in foods_transform {
                let vec = transform.translation - food_transform.translation;
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
                let cell = angle / self.fov_angle;
                let cell = cell * (self.cells as f32);
                let cell = (cell as usize).min(self.cells - 1);

                let energy = (self.fov_range - dist) / self.fov_range;

                cells[cell] += energy;
            }
            sensors.extend(cells);
        }
        sensors.extend(self.process_vision_walls(transform, config));
        assert_eq!(sensors.len(), self.sensors());
        sensors
    }
    pub fn process_vision_walls(
        &self,
        transform: &Transform,
        config: &SimulationConfig,
    ) -> Vec<f32> {
        if self.see_walls {
            let half_width = config.environment.width / 2.0;
            let half_height = config.environment.height / 2.0;
            let (axis, mut rotation_angle) = transform.rotation.to_axis_angle();
            if axis.z < 0.0 {
                rotation_angle = -rotation_angle;
            }
            let angle_incr = self.fov_angle / (self.cells as f32);
            // Starting from the lowest fov line we evaluate the distance of the closest wall intersect on this line.
            // and compute an energy
            // Doing it for each cell boundary so cells.length()+1 lines.
            let start_angle = rotation_angle - self.fov_angle / 2.0 + angle_incr / 2.0;
            (0..self.cells)
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
        } else {
            vec![]
        }
    }
    // Return the number of sensors associated with this eye configuration
    pub fn sensors(&self) -> usize {
        let mut sensors = 0;
        if self.see_foods {
            sensors += self.cells;
        }
        if self.see_walls {
            sensors += self.cells;
        }
        sensors
    }
}

impl Default for Eye {
    fn default() -> Self {
        Self::new(FOV_RANGE, FOV_ANGLE, CELLS, true, true)
    }
}
