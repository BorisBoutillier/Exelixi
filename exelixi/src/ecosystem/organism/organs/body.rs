use crate::ecosystem::*;

#[derive(Component)]
pub struct Body {
    cur_energy: i32,
    max_energy: i32,
    // Body energy consumption per step
    body_cost: i32,
}
impl Body {
    pub fn new(starting_energy: i32, max_energy: i32, body_cost: i32) -> Self {
        Self {
            cur_energy: starting_energy,
            max_energy,
            body_cost,
        }
    }
    pub fn energy_cost(&self) -> i32 {
        self.body_cost
    }
    pub fn energy(&self) -> i32 {
        self.cur_energy
    }
    // Return current energy percentage in regard to maximum energy
    pub fn energy_pct(&self) -> f32 {
        self.cur_energy as f32 / self.max_energy as f32
    }
    pub fn spend_energy(&mut self, energy: i32) -> bool {
        self.cur_energy -= energy;
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
    let mut food_decays = 0;
    for (entity, mut body, organism) in bodies.iter_mut() {
        let mut total = body.energy_cost() as f32;
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
        if !body.spend_energy(total as i32) {
            commands.entity(entity).despawn_recursive();
            if organism.kind == OrganismKind::Plant {
                food_decays += 1;
            }
        }
    }
    if food_decays > 0 {
        simulation.statistics.add_food_decay(food_decays);
    }
}
