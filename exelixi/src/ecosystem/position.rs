use std::f32::consts::PI;

use crate::ecosystem::*;

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
    pub fn distance_squared(&self, other: &Position) -> f32 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }
    pub fn angle_between(&self, other: &Position) -> f32 {
        f32::atan2(other.y - self.y, other.x - self.x)
    }
}