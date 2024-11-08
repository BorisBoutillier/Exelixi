#[bevy_trait_query::queryable]
pub trait EnergyActor {
    // Return the energy consumed in this tick.
    fn energy_consumed(&self) -> f32 {
        0.0
    }
    // Return the energy produced in this tick.
    fn energy_produced(&self) -> f32 {
        0.0
    }
}
