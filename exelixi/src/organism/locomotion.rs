use crate::prelude::*;
//
/// Maximum angalur velocity in radians/step
pub const V_ANGULAR_MAX: f32 = PI / 30.0;

#[derive(Component)]
pub struct Locomotion {
    // Linear  velocity in unit per step
    pub linear: i32,
    // Angular velocity in centi radians per step
    pub angular: i32,
    pub linear_actuator: bool,
    pub linear_max: i32,
    linear_cost: f32,
    angular_cost: f32,
}

impl Locomotion {
    pub fn new(config: &SimulationConfig) -> Self {
        match config.organisms.linear_locomotion {
            ConfigValue::Fixed(v) => Self {
                linear: v,
                angular: 0,
                linear_actuator: false,
                linear_max: v,
                linear_cost: config.organisms.linear_cost,
                angular_cost: config.organisms.angular_cost,
            },
            ConfigValue::Neuron { min: _, max } => Self {
                linear: 0,
                angular: 0,
                linear_actuator: true,
                linear_max: max,
                linear_cost: config.organisms.linear_cost,
                angular_cost: config.organisms.angular_cost,
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
            self.linear = output as i32 * self.linear_max;
        }
        // Angular
        {
            let output = outputs.next().expect("Not enough otuput neurons");
            let angular = (output.clamp(0.0, 1.0) - 0.5) * V_ANGULAR_MAX;
            self.angular = angular as i32;
        }
    }
    pub fn energy_cost(&self) -> f32 {
        self.linear_cost * self.linear.pow(2) as f32
            + self.angular_cost * self.angular.pow(2) as f32
    }
}
