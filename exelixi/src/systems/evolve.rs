use crate::prelude::*;

pub fn evolve(
    mut commands: Commands,
    mut simulation: ResMut<Simulation>,
    config: Res<SimulationConfig>,
    animals: Query<(Entity, &Stomach, &Brain, &Eye)>,
    foods: Query<Entity, With<Food>>,
    asset_server: Res<AssetServer>,
) {
    simulation.age += 1;
    if simulation.age == config.generation_length {
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
        // Remove all food
        for entity in foods.iter() {
            commands.entity(entity).despawn();
        }
        simulation.statistics = stats;
        println!("{}", simulation.sprint_state(&config));
    }
}
