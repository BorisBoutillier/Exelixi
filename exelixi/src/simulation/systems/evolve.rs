use crate::prelude::*;

pub fn evolve(
    mut commands: Commands,
    mut simulation: ResMut<Simulation>,
    config: Res<SimulationConfig>,
    animals: Query<(Entity, &Stomach, &Brain, &Eye)>,
    foods: Query<Entity, With<Food>>,
    asset_server: Res<AssetServer>,
) {
    simulation.steps += 1;
    if simulation.steps == config.generation_length {
        let mut rng = thread_rng();
        simulation.steps = 0;

        let mut fov_angles = vec![];
        let current_population = animals
            .iter()
            .map(|(entity, s, b, e)| {
                commands.entity(entity).despawn_recursive();
                fov_angles.push(e.fov_angle);
                AnimalIndividual::from_components(&config, s, e, b)
            })
            .collect::<Vec<_>>();
        let (new_population, population_stat) = simulation.ga.evolve(
            &mut rng,
            &current_population,
            config.death_threshold,
            config.fertility_rate,
        );
        new_population
            .iter()
            .enumerate()
            .for_each(|(i, individual)| {
                let selected = i == 0;
                let (eye, brain) = individual.clone().into_components(&config);
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
        let missing_population = config.min_population - new_population.len() as i32;
        for _ in 0..missing_population {
            let eye = Eye::random(&mut rng, &config);
            let brain = Brain::random(&mut rng, &eye);
            spawn_animal(&mut commands, &*asset_server, &*config, eye, brain, false);
        }
        // Remove all remaining food
        for entity in foods.iter() {
            simulation.statistics.food_decay += 1;
            commands.entity(entity).despawn_recursive();
        }
        simulation.statistics.update(population_stat);
        simulation.statistics.mean_fov_angle = mean(&fov_angles);
        simulation.statistics.std_dev_fov_angle = std_deviation(&fov_angles);
        println!("{}", simulation.sprint_state(&config));
        simulation.new_generation();
    }
}
