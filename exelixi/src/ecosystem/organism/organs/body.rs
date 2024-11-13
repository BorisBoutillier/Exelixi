use bevy_trait_query::All;

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
    // Update the body internal energy with the provided change.
    // positive energy_diff means energy has been produced by organs this tick ( negative mean consumed)
    // Must be called only once per tick, we also subtract the own body consumption
    // As Body cannot itself implement the EnergyActor trait without issue with Queries
    pub fn update_energy(&mut self, energy: f32) {
        self.cur_energy = (self.cur_energy + energy - self.body_cost).min(self.max_energy);
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

pub fn body_energy_consumption(
    mut commands: Commands,
    mut ecosystem: ResMut<EcosystemRuntime>,
    mut bodies: Query<(Entity, &Organism, &mut Body)>,
    actors: Query<All<&dyn EnergyActor>>,
) {
    for (entity, organism, mut body) in bodies.iter_mut() {
        let tick_energy = if let Ok(energy_actors) = actors.get(entity) {
            // Aggregate all energy produced and consumed this tick by this entity
            energy_actors
                .into_iter()
                .map(|actor| actor.energy_produced() - actor.energy_consumed())
                .sum()
        } else {
            // Some organism can only have a Body, like plants with all leaves lost.
            0.0
        };
        body.update_energy(tick_energy);
        if body.is_dead() {
            // We have consume more energy than we had in stock, we are dead.
            // Death consists simply in fully despawning ourself.
            commands.entity(entity).despawn_recursive();
            ecosystem.decrease_population(&organism.species());
        }
    }
}
