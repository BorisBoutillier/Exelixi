use crate::prelude::*;

pub fn evolve(
    mut commands: Commands,
    mut simulation: ResMut<Simulation>,
    config: Res<SimulationConfig>,
    animals: Query<(Entity, &Body, &Brain, &Eye)>,
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
            .map(|(entity, body, brain, eye)| {
                commands.entity(entity).despawn_recursive();
                fov_angles.push(eye.fov_angle);
                AnimalIndividual::from_components(&config, body, eye, brain)
            })
            .collect::<Vec<_>>();
        simulation.statistics.end_of_generation(&current_population);
        //simulation.statistics.mean_fov_angle = mean(&fov_angles);
        //simulation.statistics.std_dev_fov_angle = std_deviation(&fov_angles);
        println!("{}", simulation.sprint_state(&config));

        let mut new_population = simulation.ga.evolve(
            &mut rng,
            &current_population,
            config.fertility_rate,
            (config.min_population as f32 * 0.9) as usize, // If survivors and fertility are not enough keep 10% random
        );
        // If not enough survived, add random animals
        let missing_population = config.min_population as i32 - new_population.len() as i32;
        for _ in 0..missing_population {
            new_population.push(AnimalIndividual::random(&mut rng, &config));
        }
        simulation
            .statistics
            .start_of_new_generation(&new_population, &config);

        simulation.new_generation();
        // Remove all remaining food
        {
            let mut food_decay = 0;
            for entity in foods.iter() {
                commands.entity(entity).despawn_recursive();
                food_decay += 1;
            }
            simulation.statistics.add_food_decay(food_decay);
        }
        // Spawn new Animals
        new_population
            .into_iter()
            .enumerate()
            .for_each(|(i, individual)| {
                let selected = i == 0;
                let (eye, brain) = individual.into_components(&config);
                simulation.statistics.population.add_entry(&eye);
                spawn_animal(
                    &mut commands,
                    &*asset_server,
                    &*config,
                    eye,
                    brain,
                    selected,
                );
            });
    }
}
