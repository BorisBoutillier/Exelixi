use std::f32::consts::PI;

use crate::prelude::*;

#[derive(Clone, Copy, Component, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    // Orientation in radian
    angle: f32,
}

impl Position {
    pub fn new(x: f32, y: f32, angle: f32) -> Self {
        let mut pos = Self { x, y, angle: 0.0 };
        pos.set_angle(angle);
        pos
    }
    pub fn angle(&self) -> f32 {
        self.angle
    }
    pub fn set_angle(&mut self, angle: f32) {
        // Keep within  -PI..PI
        self.angle = (angle + PI) % (2.0 * PI) - PI;
    }
}
