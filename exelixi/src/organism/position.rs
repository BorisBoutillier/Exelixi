use crate::prelude::*;

#[derive(Clone, Copy, Component, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    // Orientation in centi-radian
    angle_crad: i32,
}

impl Position {
    pub fn new(x: i32, y: i32, angle: f32) -> Self {
        Self {
            x,
            y,
            angle_crad: (angle * 100.0) as i32,
        }
    }
    pub fn angle(&self) -> f32 {
        self.angle_crad as f32 / 100.0
    }
    pub fn angle_crad(&self) -> i32 {
        self.angle_crad
    }
    pub fn set_angle_crad(&mut self, angle_crad: i32) {
        // Keep within  -PI..PI
        self.angle_crad = (angle_crad + 314) % (2 * 314) - 314;
    }
}
