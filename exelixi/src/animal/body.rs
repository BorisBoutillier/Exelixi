use crate::prelude::*;

#[derive(Component)]
pub struct Body {
    cur_energy: f32,
    max_energy: f32,
    pub consumed_energy: f32,
}
impl Body {
    pub fn new(config: &SimulationConfig) -> Self {
        Self {
            cur_energy: config.animals.starting_energy,
            max_energy: config.animals.maximum_energy,
            consumed_energy: 0.0,
        }
    }
    pub fn energy(&self) -> f32 {
        self.cur_energy
    }
    pub fn spend_energy(&mut self, energy: f32) -> bool {
        self.cur_energy -= energy;
        self.consumed_energy += energy;
        self.cur_energy > 0.0
    }
    pub fn add_energy(&mut self, energy: f32) {
        self.cur_energy = (self.cur_energy + energy).min(self.max_energy);
    }
}
