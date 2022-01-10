use crate::*;

pub fn movement(mut movables: Query<(&mut Transform, &Velocity)>, config: Res<SimulationConfig>) {
    let half_width = config.environment.size.width / 2.0;
    let half_height = config.environment.size.height / 2.0;
    movables.for_each_mut(|(mut transform, velocity)| {
        // Update transform based on linear and angular velocity
        let delta = transform.rotation * Vec3::new(velocity.linear, 0.0, 0.0);
        transform.translation += delta;
        transform.rotation *= Quat::from_axis_angle(Vec3::Z, velocity.angular);
        if config.environment.wall {
            // Detects wall collision and mirror the rotation angle
            let (angle_vec, mut angle) = transform.rotation.to_axis_angle();
            if angle_vec.z < 0.0 {
                angle = -angle;
            }
            if (transform.translation.x < -half_width && delta.x < 0.0)
                || (transform.translation.x > half_width && delta.x > 0.0)
            {
                let new_angle = -angle + PI;
                transform.rotation = Quat::from_axis_angle(Vec3::Z, new_angle);
                transform.translation.x =
                    f32::clamp(transform.translation.x, -half_width, half_width);
            }
            if (transform.translation.y < -half_height && delta.y < 0.0)
                || (transform.translation.y > half_height && delta.y > 0.0)
            {
                let new_angle = -angle;
                transform.rotation = Quat::from_axis_angle(Vec3::Z, new_angle);
                transform.translation.y =
                    f32::clamp(transform.translation.y, -half_height, half_height);
            }
        } else {
            // Detects border interaction and wrap around
            if transform.translation.x < -half_width {
                transform.translation.x = half_width;
            }
            if transform.translation.x > half_width {
                transform.translation.x = -half_width;
            }
            if transform.translation.y < -half_height {
                transform.translation.y = half_height;
            }
            if transform.translation.y > half_height {
                transform.translation.y = -half_height;
            }
        }
    });
}
