use crate::*;

pub fn movement(
    time: Res<Time>,
    mut movables: Query<(&mut Transform, &Velocity)>,
    config: Res<SimulationConfig>,
    simulation: Res<Simulation>,
) {
    if !simulation.running {
        return;
    }
    let half_width = config.environment_size.width / 2.0;
    let half_height = config.environment_size.height / 2.0;
    movables.for_each_mut(|(mut transform, velocity)| {
        // Update transform based on linear and angular velocity
        let delta =
            transform.rotation * Vec3::new(velocity.linear * time.delta_seconds(), 0.0, 0.0);
        transform.translation += delta;
        transform.rotation *=
            Quat::from_axis_angle(Vec3::Z, velocity.angular * time.delta_seconds());
        // Handle arena limit collision by 'mirroring' the rotation angle
        let (angle_vec, mut angle) = transform.rotation.to_axis_angle();
        if angle_vec.z < 0.0 {
            angle = -angle;
        }
        if (transform.translation.x < -half_width && delta.x < 0.0)
            || (transform.translation.x > half_width && delta.x > 0.0)
        {
            let new_angle = -angle + PI;
            transform.rotation = Quat::from_axis_angle(Vec3::Z, new_angle);
        }
        if (transform.translation.y < -half_height && delta.y < 0.0)
            || (transform.translation.y > half_height && delta.y > 0.0)
        {
            let new_angle = -angle;
            transform.rotation = Quat::from_axis_angle(Vec3::Z, new_angle);
        }
    });
}
