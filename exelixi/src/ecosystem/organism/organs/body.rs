use std::collections::HashMap;

use crate::ecosystem::*;

#[derive(Component)]
pub struct Body {
    cur_energy: f32,
    max_energy: f32,
    // Body energy consumption per step
    body_cost: f32,
}
impl Body {
    pub fn new(config: &BodyConfig) -> Self {
        Self {
            cur_energy: config.starting_energy,
            max_energy: config.maximum_energy,
            body_cost: config.body_cost,
        }
    }
    pub fn energy_cost(&self) -> f32 {
        self.body_cost
    }
    pub fn energy(&self) -> f32 {
        self.cur_energy
    }
    // Return current energy percentage in regard to maximum energy
    pub fn energy_pct(&self) -> f32 {
        self.cur_energy / self.max_energy
    }
    pub fn spend_energy(&mut self, energy: f32) -> bool {
        self.cur_energy -= energy;
        self.cur_energy > 0.0
    }
    pub fn add_energy(&mut self, energy: f32) {
        self.cur_energy = (self.cur_energy + energy).min(self.max_energy);
    }
    pub fn get_sensors(&self) -> Vec<f32> {
        vec![self.cur_energy / self.max_energy]
    }
    pub fn n_sensors(&self) -> usize {
        1
    }
}

#[allow(clippy::too_many_arguments)]
pub fn body_energy_consumption(
    mut commands: Commands,
    mut bodies: Query<(Entity, &mut Body, &Organism)>,
    q0: Query<&Brain>,
    q1: Query<&Eye>,
    q2: Query<&Locomotion>,
    q3: Query<&Leaf>,
    mut simulation: ResMut<Simulation>,
) {
    let mut deaths = HashMap::new();
    for (entity, mut body, organism) in bodies.iter_mut() {
        let mut total = body.energy_cost();
        if let Ok(organ) = q0.get(entity) {
            total += organ.energy_cost();
        }
        if let Ok(organ) = q1.get(entity) {
            total += organ.energy_cost();
        }
        if let Ok(organ) = q2.get(entity) {
            total += organ.energy_cost();
        }
        if let Ok(organ) = q3.get(entity) {
            total += organ.energy_cost();
        }
        if !body.spend_energy(total) {
            commands.entity(entity).despawn_recursive();
            *deaths.entry(organism.name.clone()).or_insert(0) += 1;
        }
    }
    if !deaths.is_empty() {
        simulation.statistics.update_deaths(deaths);
    }
}
