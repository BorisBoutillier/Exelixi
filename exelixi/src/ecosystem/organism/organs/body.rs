use bevy_trait_query::All;

use crate::ecosystem::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Body {
    /// Config copy
    // Local copy of BodyConfig maximum_energy
    config_maximum_energy: f32,
    // Local copy of BodyConfig body_cost
    config_body_cost: f32,
    /// Current data
    // Energy of the body at the current time.
    pub energy: f32,
}
impl Body {
    pub fn new(config: &BodyConfig) -> Self {
        Self {
            config_maximum_energy: config.maximum_energy,
            config_body_cost: config.body_cost,
            energy: config.starting_energy,
        }
    }
    // Return current energy percentage in regard to maximum energy
    pub fn energy_pct(&self) -> f32 {
        self.energy / self.config_maximum_energy
    }
}
impl Sensor for Body {
    fn n_sensors(&self) -> usize {
        1
    }

    fn sensors(&self) -> Vec<f32> {
        vec![self.energy / self.config_maximum_energy]
    }
}

pub fn body_processing(
    mut bodies: Query<(Entity, &mut Body)>,
    mut organisms_lifecycle: ResMut<OrganismsLifecycle>,
    actors: Query<All<&dyn EnergyActor>>,
) {
    for (entity, mut body) in bodies.iter_mut() {
        // Update Body energy, based on consumers and producers
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
        // Body cannot itself implement the EnergyActor trait without issue with Queries
        // So don't forget to remove body_cost
        body.energy =
            (body.energy + tick_energy - body.config_body_cost).min(body.config_maximum_energy);

        // We die if we have no more energy.
        if body.energy <= 0.0 {
            organisms_lifecycle.add_death(entity);
        }
    }
}
