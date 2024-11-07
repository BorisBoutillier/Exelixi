use crate::ecosystem::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
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
    pub fn energy(&self) -> f32 {
        self.cur_energy
    }
    // Return current energy percentage in regard to maximum energy
    pub fn energy_pct(&self) -> f32 {
        self.cur_energy / self.max_energy
    }
    pub fn add_energy(&mut self, energy: f32) {
        self.cur_energy = (self.cur_energy + energy).min(self.max_energy);
    }
    pub fn set_energy(&mut self, energy: f32) {
        self.cur_energy = energy.min(self.max_energy);
    }
    pub fn is_dead(&self) -> bool {
        self.cur_energy <= 0.0
    }
}
impl Sensor for Body {
    fn n_sensors(&self) -> usize {
        1
    }

    fn sensors(&self) -> Vec<f32> {
        vec![self.cur_energy / self.max_energy]
    }
}
impl EnergyActor for Body {
    fn energy_consumed(&self) -> f32 {
        self.body_cost
    }
}

pub fn body_energy_consumption(
    mut commands: Commands,
    mut actors: Query<(
        Entity,
        &mut Body,
        Option<&Leaf>,
        Option<&Mouth>,
        Option<&Brain>,
        Option<&Eye>,
        Option<&Locomotion>,
    )>,
) {
    //let mut energy_updates = HashMap::new();
    for (entity, mut body, a0, a1, a2, a3, a4) in actors.iter_mut() {
        // Aggregate all energy produced and consumed this tick by this entity
        // Accumulate in a variable before updating the body energy only once,
        // so that clamping only happen once.
        let mut tick_energy = body.energy_produced() - body.energy_consumed();
        if let Some(actor) = a0 {
            tick_energy += actor.energy_produced() - actor.energy_consumed();
        }
        if let Some(actor) = a1 {
            tick_energy += actor.energy_produced() - actor.energy_consumed();
        }
        if let Some(actor) = a2 {
            tick_energy += actor.energy_produced() - actor.energy_consumed();
        }
        if let Some(actor) = a3 {
            tick_energy += actor.energy_produced() - actor.energy_consumed();
        }
        if let Some(actor) = a4 {
            tick_energy += actor.energy_produced() - actor.energy_consumed();
        }
        body.add_energy(tick_energy);
        if body.is_dead() {
            // We have consume more energy than we had in stock, we are dead.
            // Death consists simply in fully despawning ourself.
            commands.entity(entity).despawn_recursive();
        }
    }
}
