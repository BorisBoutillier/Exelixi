use std::f32::consts::PI;

use rand::thread_rng;

use crate::*;

pub fn spawn_animals(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    let mut rng = thread_rng();
    for _ in 0..40 {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(15.0, 15.0)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(
                        rng.gen_range((-window.width() / 2.0)..(window.width() / 2.0)),
                        rng.gen_range((-window.height() / 2.0)..(window.height() / 2.0)),
                        1.0,
                    ),
                    rotation: Quat::from_axis_angle(Vec3::Z, rng.gen_range(-PI..PI)),
                    ..Default::default()
                },
                texture: asset_server.load("bird.png"),
                ..Default::default()
            })
            .insert(Velocity {
                linear: rng.gen_range(V_LINEAR_MIN..V_LINEAR_MAX),
                angular: rng.gen_range(-V_ANGULAR_MAX..V_ANGULAR_MAX),
            });
    }
}
