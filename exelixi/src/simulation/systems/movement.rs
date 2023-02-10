use crate::*;

pub fn movement(mut movables: Query<(&mut Position, &Locomotion)>, config: Res<SimulationConfig>) {
    let half_width = config.environment.width / 2;
    let half_height = config.environment.height / 2;
    movables.for_each_mut(|(mut position, locomotion)| {
        // Update transform based on linear and angular velocity
        let delta_x = (position.angle().cos() * locomotion.linear as f32) as i32;
        let delta_y = (position.angle().sin() * locomotion.linear as f32) as i32;
        position.x += delta_x;
        position.y += delta_y;
        position.angle_crad += locomotion.angular;
        if config.environment.wall {
            // Detects wall collision and mirror the rotation angle
            if (position.x < -half_width && delta_x < 0) || (position.x > half_width && delta_x > 0)
            {
                position.angle_crad = -position.angle_crad + (PI * 100.0) as i32;
                position.x = i32::clamp(position.x, -half_width, half_width);
            }
            if (position.y < -half_height && delta_y < 0)
                || (position.y > half_height && delta_y > 0)
            {
                position.angle_crad = -position.angle_crad;
                position.y = i32::clamp(position.y, -half_height, half_height);
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

pub fn transform_update(mut query: Query<(&mut Transform, &Position), Changed<Position>>) {
    for (mut transform, position) in query.iter_mut() {
        transform.translation.x = position.x as f32;
        transform.translation.y = position.y as f32;
        transform.rotation = Quat::from_axis_angle(Vec3::Z, position.angle());
    }
}
