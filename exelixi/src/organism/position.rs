use crate::prelude::*;

#[derive(Clone, Copy, Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    // Orientation in centi-radian
    pub angle_crad: i32,
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
}
