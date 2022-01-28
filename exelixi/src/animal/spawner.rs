use crate::prelude::*;

pub fn spawn_animal(
    commands: &mut Commands,
    asset_server: &AssetServer,
    config: &SimulationConfig,
    eye: Eye,
    brain: Brain,
    selected: bool,
) {
    let half_width = config.environment.width / 2.0;
    let half_height = config.environment.height / 2.0;
    let mut rng = thread_rng();
    let color = if selected {
        Color::rgb(0.2, 0.9, 0.9)
    } else {
        Color::rgb(0.8, 0.3, 0.8)
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
pub fn spawn_starting_animals(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<SimulationConfig>,
) {
    if config.is_changed() {
        let mut rng = thread_rng();
        for i in 0..config.start_population {
            let selected = i == 0;
            let eye = Eye::random(&mut rng, &config);
            let brain = Brain::random(&mut rng, &eye);
            spawn_animal(
                &mut commands,
                &*asset_server,
                &*config,
                eye,
                brain,
                selected,
            );
        }
    }
}
