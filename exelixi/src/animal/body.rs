use crate::prelude::*;

#[derive(Component)]
pub struct Body {
    pub energy: f32,
    pub max_energy: f32,
}
impl Body {
    pub fn new(config: &SimulationConfig) -> Self {
        Self {
            energy: config.animals.starting_energy,
            max_energy: config.animals.maximum_energy,
        }
    }
}
