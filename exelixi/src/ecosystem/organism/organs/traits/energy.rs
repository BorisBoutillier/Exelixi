// Defines that an organ can consume energy each tick.
pub trait EnergyConsumer {
    // Return the energy consumed in this tick.
    fn energy_consumed(&self) -> f32;
}
//
// Defines that an organ can produce energy each tick.
pub trait EnergyProducer {
    // Return the energy produced in this tick.
    fn energy_produced(&self) -> f32;
}
