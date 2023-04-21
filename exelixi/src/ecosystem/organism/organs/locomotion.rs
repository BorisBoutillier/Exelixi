use std::f32::consts::PI;

use crate::ecosystem::*;
//
/// Maximum angalur velocity in radians/step
pub const V_ANGULAR_MAX: f32 = 0.2;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Locomotion {
    // Linear  velocity in unit per step
    pub linear: f32,
    // Angular velocity in centi radians per step
    pub angular: f32,
    pub linear_actuator: bool,
    pub linear_max: f32,
    linear_cost: f32,
    angular_cost: f32,
}

impl Locomotion {
    pub fn new(config: &LocomotionConfig) -> Self {
        match config.linear {
            ConfigValue::Fixed(v) => Self {
                linear: v,
                angular: 0.0,
                linear_actuator: false,
                linear_max: v,
                linear_cost: config.linear_cost,
                angular_cost: config.angular_cost,
            },
            ConfigValue::Neuron { min: _, max } => Self {
                linear: 0.0,
                angular: 0.0,
                linear_actuator: true,
                linear_max: max,
                linear_cost: config.linear_cost,
                angular_cost: config.angular_cost,
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
                .expect("Not enough output neurons")
                .clamp(0.0, 1.0);
            self.linear = output * self.linear_max;
        }
        // Angular
        {
            let output = outputs.next().expect("Not enough otuput neurons");
            self.angular = (output.clamp(0.0, 1.0) - 0.5) * V_ANGULAR_MAX;
        }
    }
    pub fn energy_cost(&self) -> f32 {
        self.linear_cost * self.linear.powi(2) + self.angular_cost * self.angular.powi(2)
    }
}

pub fn locomotion_movement(
    mut movables: Query<(&mut Position, &Locomotion)>,
    config: Res<EcosystemConfig>,
) {
    let half_width = config.environment.width as f32 / 2.0;
    let half_height = config.environment.height as f32 / 2.0;
    movables.for_each_mut(|(mut position, locomotion)| {
        // Update transform based on linear and angular velocity
        let delta_x = position.angle().cos() * locomotion.linear;
        let delta_y = position.angle().sin() * locomotion.linear;
        position.x += delta_x;
        position.y += delta_y;
        let new_angle = position.angle() + locomotion.angular;
        position.set_angle(new_angle);
        if config.environment.wall {
            // Detects wall collision and mirror the rotation angle
            if (position.x < -half_width && delta_x.is_sign_negative())
                || (position.x > half_width && delta_x.is_sign_positive())
            {
                let new_angle = -position.angle() + PI;
                position.set_angle(new_angle);
                position.x = position.x.clamp(-half_width, half_width);
            }
            if (position.y < -half_height && delta_y.is_sign_negative())
                || (position.y > half_height && delta_y.is_sign_positive())
            {
                let new_angle = -position.angle();
                position.set_angle(new_angle);
                position.y = position.y.clamp(-half_height, half_height);
            }
        } else {
            // Detects border interaction and wrap around
            if position.x < -half_width {
                position.x = half_width;
            }
            if position.x > half_width {
                position.x = -half_width;
            }
            if position.y < -half_height {
                position.y = half_height;
            }
            if position.y > half_height {
                position.y = -half_height;
            }
        }
    });
}
