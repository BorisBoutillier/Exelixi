// Defines that an organ provides inputs (sensors) for the brain neural network.
pub trait Sensor {
    // Defines the number of sensor that this organ will create in the brain, as inputs to the neuron network.
    fn n_sensors(&self) -> usize;
    // Returns the values for the inputs in the brain neural network.
    // The length of the Vec must be equal to n_sensors
    fn sensors(&self) -> Vec<f32>;
}

// Defines that an organ needs outputs of the brain neural network.
pub trait Actuator {
    // Defines the number of output needed in the neural network
    fn n_actuators(&self) -> usize;
    // Should consumes n_actuators items in the output iterator to behave
    fn actuates(&mut self, outputs: &mut impl Iterator<Item = f32>);
}
