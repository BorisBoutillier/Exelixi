use crate::prelude::*;
//
/// Maximum angalur velocity in radians/step
pub const V_ANGULAR_MAX: f32 = PI / 30.0;

#[derive(Component)]
pub struct Locomotion {
    pub linear: f32,
    pub angular: f32,
    pub linear_actuator: bool,
    pub linear_max: f32,
}

impl Locomotion {
    pub fn new(config: &SimulationConfig) -> Self {
        match config.animals.linear_locomotion {
            ConfigValue::Fixed(v) => Self {
                linear: v,
                angular: 0.0,
                linear_actuator: false,
                linear_max: v,
            },
            ConfigValue::Neuron { min: _, max } => Self {
                linear: 0.0,
                angular: 0.0,
                linear_actuator: true,
                linear_max: max,
            },
            _ => panic!(),
        }
    }
    pub fn n_actuators(&self) -> usize {
        1 + if self.linear_actuator { 1 } else { 0 }
    }
    pub fn actuates(&mut self, outputs: impl IntoIterator<Item = f32>) {
        let mut outputs = outputs.into_iter();
        // Linear
        if self.linear_actuator {
            let output = outputs
                .next()
                .expect("Not enough otuput neurons")
                .clamp(0.0, 1.0);
            self.linear = output * self.linear_max;
        }
        // Angular
        {
            let output = outputs.next().expect("Not enough otuput neurons");
            let angular = (output.clamp(0.0, 1.0) - 0.5) * V_ANGULAR_MAX;
            self.angular = angular;
        }
    }
}
