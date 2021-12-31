use crate::*;

pub fn movement(
    time: Res<Time>,
    windows: Res<Windows>,
    mut movables: Query<(&mut Transform, &Velocity)>,
) {
    let window = windows.get_primary().unwrap();
    for (mut transform, velocity) in movables.iter_mut() {
        let delta =
            transform.rotation * Vec3::new(velocity.linear * time.delta_seconds(), 0.0, 0.0);
        transform.translation += delta;
        transform.rotation *=
            Quat::from_axis_angle(Vec3::Z, velocity.angular * time.delta_seconds());
    }
}
