use std::f32::consts::PI;

use crate::*;

pub fn spawn_animals(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    let mut rng = thread_rng();
    for i in 0..N_ANIMAL {
        let selected = i == 0;
        let color = if selected {
            Color::YELLOW
        } else {
            Color::WHITE
        };
        let mut command = commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(25.0, 25.0)),
                color,
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
        });
        let eye = Eye::default();
        let brain = Brain::random(&mut rng, &eye);
        command
            .insert(Velocity {
                linear: rng.gen_range(V_LINEAR_MIN..V_LINEAR_MAX),
                angular: 0.0,
            })
            .insert(Stomach::default())
            .insert(eye)
            .insert(brain);
        if selected {
            command.insert(Selected {});
        }
    }
}

pub fn spawn_foods(mut commands: Commands, asset_server: Res<AssetServer>, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let mut rng = thread_rng();
    for _ in 0..N_FOOD {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(
                        rng.gen_range((-window.width() / 2.0)..(window.width() / 2.0)),
                        rng.gen_range((-window.height() / 2.0)..(window.height() / 2.0)),
                        1.0,
                    ),
                    ..Default::default()
                },
                texture: asset_server.load("food.png"),
                ..Default::default()
            })
            .insert(Food {});
    }
}
