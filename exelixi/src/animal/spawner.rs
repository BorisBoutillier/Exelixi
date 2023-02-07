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
    let mut command = commands.spawn(SpriteBundle {
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
        .insert(Locomotion::new(config))
        .insert(Body::new(config))
        .insert(eye)
        .insert(brain);
    if selected {
        command.insert(Selected);
    }
}
pub fn spawn_starting_animals(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut simulation: ResMut<Simulation>,
    config: Res<SimulationConfig>,
) {
    if config.is_changed() {
        let mut rng = thread_rng();
        // Create a new random population
        let new_population = (0..config.min_population)
            .map(|_| AnimalIndividual::random(&mut rng, &config))
            .collect::<Vec<_>>();
        simulation
            .statistics
            .start_of_new_generation(&new_population, &config);
        simulation.new_generation();
        // Spawn the Animals
        new_population
            .into_iter()
            .enumerate()
            .for_each(|(i, individual)| {
                let selected = i == 0;
                let (eye, brain) = individual.into_components(&config);
                simulation.statistics.population.add_entry(&eye);
                spawn_animal(&mut commands, &asset_server, &config, eye, brain, selected);
            });
    }
}
