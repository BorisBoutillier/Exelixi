use crate::prelude::*;

use std::f32::consts::{FRAC_PI_4, PI};

const FOV_RANGE: f32 = 300.0;
const FOV_ANGLE: f32 = PI + FRAC_PI_4;
const CELLS: usize = 9;

#[derive(Debug, Component)]
pub struct Eye {
    pub fov_range: f32,
    pub fov_angle: f32,
    pub cells: usize,
}

impl Eye {
    pub fn new(fov_range: f32, fov_angle: f32, cells: usize) -> Self {
        assert!(fov_range > 0.0);
        assert!(fov_angle > 0.0);
        assert!(cells > 0);
        Self {
            fov_range,
            fov_angle,
            cells,
        }
    }
    pub fn process_vision(
        &self,
        transform: &Transform,
        foods_transform: &[&Transform],
    ) -> Vec<f32> {
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
        cells
    }
}

impl Default for Eye {
    fn default() -> Self {
        Self::new(FOV_RANGE, FOV_ANGLE, CELLS)
    }
}
