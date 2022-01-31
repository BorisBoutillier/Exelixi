use crate::prelude::*;

#[derive(Component)]
pub struct Animal {}
#[derive(Component)]
pub struct Food {
    pub eaten: bool,
    pub energy: f32,
}
impl Food {
    pub fn new(config: &SimulationConfig) -> Self {
        Self {
            eaten: false,
            energy: config.environment.food_energy,
        }
    }
}

#[derive(Component)]
pub struct Floor {}

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct Decay {
    // Number of steps after which this entity will be despawned
    pub time: i32,
}
