use crate::prelude::*;

#[derive(Component)]
pub struct Organism {}
#[derive(Component)]
pub struct Food {
    pub eaten: bool,
    pub energy: i32,
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
pub struct Decay {
    // Number of steps after which this entity will be despawned
    pub time: i32,
}
