use crate::ecosystem::*;

#[derive(Component)]
pub struct Body {
    cur_energy: i32,
    max_energy: i32,
    pub consumed_energy: i32,
    // Body energy consumption per step
    body_cost: i32,
}
impl Body {
    pub fn new(config: &EcosystemConfig) -> Self {
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
    pub fn get_sensors(&self) -> Vec<f32> {
        vec![self.cur_energy as f32 / self.max_energy as f32]
    }
    pub fn n_sensors(&self) -> usize {
        1
    }
}

pub fn body_energy_consumption(
    mut commands: Commands,
    mut energy_query: Query<(Entity, &mut Body, &Brain, &Eye, &Locomotion)>,
) {
    let mut deads = vec![];
    for (entity, mut body, brain, eye, locomotion) in energy_query.iter_mut() {
        let total_cost = body.energy_cost() as f32
            + brain.energy_cost()
            + eye.energy_cost()
            + locomotion.energy_cost();
        if !body.spend_energy(total_cost as i32) {
            deads.push(entity);
        }
    }
    for dead in deads.iter() {
        commands.entity(*dead).despawn_recursive();
    }
}
