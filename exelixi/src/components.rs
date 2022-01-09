use crate::prelude::*;

#[derive(Component)]
pub struct Velocity {
    pub linear: f32,
    pub angular: f32,
}
#[derive(Component)]
pub struct Animal {}
#[derive(Component)]
pub struct Food {}

#[derive(Component)]
pub struct Floor {}

#[derive(Component)]
pub struct Stomach {
    pub satiation: f32,
}
impl Stomach {
    pub fn new() -> Self {
        Self { satiation: 0.0 }
    }
}

impl Default for Stomach {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Component)]
pub struct Selected {}