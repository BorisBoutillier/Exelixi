use crate::prelude::*;

pub fn evolve(
    mut commands: Commands,
    mut simulation: ResMut<Simulation>,
    config: Res<SimulationConfig>,
    animals: Query<(Entity, &Stomach, &Brain, &Eye)>,
    mut transforms: Query<&mut Transform, Or<(With<Food>, With<Animal>)>>,
    asset_server: Res<AssetServer>,
) {
    simulation.age += 1;
    if simulation.age == config.generation_length {
        let half_width = config.environment.size.width / 2.0;
        let half_height = config.environment.size.height / 2.0;
        let mut rng = thread_rng();
        simulation.age = 0;
        simulation.generation += 1;

        let current_population = animals
            .iter()
            .map(|(entity, s, b, _)| {
                commands.entity(entity).despawn();
                AnimalIndividual::from_stomach_and_brain(s, b)
            })
            .collect::<Vec<_>>();
        let (new_population, stats) = simulation.ga.evolve(
            &mut rng,
            &current_population,
            config.fitness_die_threshold,
            config.fitness_reproduce_threshold,
        );
        new_population
            .iter()
            .enumerate()
            .for_each(|(i, individual)| {
                let selected = i == 0;
                let eye = Eye {
                    see_walls: config.environment.wall,
                    ..Default::default()
                };
                let brain = individual.clone().into_brain(&eye);
                spawn_animal(
                    &mut commands,
                    &*asset_server,
                    &*config,
                    eye,
                    brain,
                    selected,
                );
            });
        // If not enough survived, add random animals
        if new_population.len() < config.min_population as usize {
            for _ in 0..(config.min_population as usize - new_population.len()) {
                let eye = Eye {
                    see_walls: config.environment.wall,
                    ..Default::default()
                };
                let brain = Brain::random(&mut rng, &eye);
                spawn_animal(&mut commands, &*asset_server, &*config, eye, brain, false);
            }
        }
        simulation.statistics = stats;
        // Reset all transform
        for mut transform in transforms.iter_mut() {
            transform.translation.x = rng.gen_range(-half_width..half_width);
            transform.translation.y = rng.gen_range(-half_height..half_height);
            transform.rotation = Quat::from_axis_angle(Vec3::Z, rng.gen_range(-PI..PI));
        }
        println!("{}", simulation.sprint_state(&config));
    }
}
