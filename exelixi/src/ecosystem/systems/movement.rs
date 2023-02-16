use crate::*;

pub fn movement(mut movables: Query<(&mut Position, &Locomotion)>, config: Res<SimulationConfig>) {
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
