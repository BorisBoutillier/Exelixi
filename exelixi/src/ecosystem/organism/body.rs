use crate::prelude::*;

#[derive(Component)]
pub struct Body {
    cur_energy: i32,
    max_energy: i32,
    pub consumed_energy: i32,
    // Body energy consumption per step
    body_cost: i32,
}
impl Body {
    pub fn new(config: &SimulationConfig) -> Self {
        Self {
            cur_energy: config.organisms.starting_energy,
            max_energy: config.organisms.maximum_energy,
            consumed_energy: 0,
            body_cost: config.organisms.body_cost,
        }
    }
    pub fn energy_cost(&self) -> i32 {
        self.body_cost
    }
    pub fn energy(&self) -> i32 {
        self.cur_energy
    }
    pub fn spend_energy(&mut self, energy: i32) -> bool {
        self.cur_energy -= energy;
        self.consumed_energy += energy;
        self.cur_energy > 0
    }
    pub fn add_energy(&mut self, energy: i32) {
        self.cur_energy = (self.cur_energy + energy).min(self.max_energy);
    }
}
