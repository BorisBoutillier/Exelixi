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
                commands.entity(entity).despawn_recursive();
                AnimalIndividual::from_stomach_and_brain(s, b)
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
                let eye = Eye {
                    see_walls: config.environment.wall && config.animals.see_walls,
                    see_foods: config.animals.see_foods,
                    see_animals: config.animals.see_animals,
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
        let missing_population = config.min_population - new_population.len() as i32;
        for _ in 0..missing_population {
            let eye = Eye {
                see_walls: config.environment.wall && config.animals.see_walls,
                see_foods: config.animals.see_foods,
                see_animals: config.animals.see_animals,
                ..Default::default()
            };
            let brain = Brain::random(&mut rng, &eye);
            spawn_animal(&mut commands, &*asset_server, &*config, eye, brain, false);
        }
        // Remove all food
        for entity in foods.iter() {
            commands.entity(entity).despawn_recursive();
        }
        simulation.statistics.update(population_stat);
        println!("{}", simulation.sprint_state(&config));
    }
}
