// Defines that an organ provides inputs (sensors) for the brain neural network.
pub trait Sensor {
    // Defines the number of sensor that this organ will create in the brain, as inputs to the neuron network.
    fn n_sensors(&self) -> usize;
    // Returns the values for the inputs in the brain neural network.
    // The length of the Vec must be equal to n_sensors
    fn sensors(&self) -> Vec<f32>;
}
