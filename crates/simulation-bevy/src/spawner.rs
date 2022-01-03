use std::f32::consts::PI;

use crate::*;

pub fn spawn_animals(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<SimulationConfig>,
) {
    let half_width = config.environment_size.width / 2.0;
    let half_height = config.environment_size.height / 2.0;
    let mut rng = thread_rng();
    for i in 0..config.starting_animals {
        let selected = i == 0;
        let color = if selected {
            Color::rgb(0.5, 0.5, 0.7)
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
                    rng.gen_range(-half_width..half_width),
                    rng.gen_range(-half_height..half_height),
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
            .insert(Animal {})
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

pub fn spawn_foods(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<SimulationConfig>,
) {
    let half_width = config.environment_size.width / 2.0;
    let half_height = config.environment_size.height / 2.0;
    let mut rng = thread_rng();
    for _ in 0..config.starting_foods {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(
                        rng.gen_range(-half_width..half_width),
                        rng.gen_range(-half_height..half_height),
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

pub fn spawn_floor(mut commands: Commands, config: Res<SimulationConfig>) {
    println!("SPAWN {}", config.environment_size.width as f32);
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(
                    config.environment_size.width as f32,
                    config.environment_size.height as f32,
                )),
                color: Color::rgb(0.05, 0.0, 0.2),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Floor {});
}
