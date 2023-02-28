use crate::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn evolve(
    mut commands: Commands,
    mut simulation: ResMut<Simulation>,
    config: Res<SimulationConfig>,
    organisms: Query<(Entity, &Body, &Brain, &Eye)>,
    foods: Query<Entity, With<Food>>,
    mut rng: ResMut<MyRng>,
) {
    simulation.steps += 1;
    if simulation.steps == config.generation_length {
        simulation.steps = 0;

        let mut fov_angles = vec![];
        let current_population = organisms
            .iter()
            .map(|(entity, body, brain, eye)| {
                commands.entity(entity).despawn_recursive();
                fov_angles.push(eye.fov_angle);
                OrganismIndividual::from_components(&config, body, eye, brain)
            })
            .collect::<Vec<_>>();
        simulation.statistics.end_of_generation(&current_population);
        //simulation.statistics.mean_fov_angle = mean(&fov_angles);
        //simulation.statistics.std_dev_fov_angle = std_deviation(&fov_angles);
        println!("{}", simulation.sprint_state(&config));

        let mut new_population = simulation.ga.evolve(
            &mut rng.0,
            &current_population,
            config.fertility_rate,
            (config.min_population as f32 * 0.9) as usize, // If survivors and fertility are not enough keep 10% random
        );
        // If not enough survived, add random organisms
        let missing_population = config.min_population as i32 - new_population.len() as i32;
        for _ in 0..missing_population {
            new_population.push(OrganismIndividual::random(&mut rng.0, &config));
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
        // Spawn new organisms
        new_population
            .into_iter()
            .enumerate()
            .for_each(|(i, individual)| {
                let selected = i == 0;
                let (body, eye, brain) = individual.into_components(&config);
                simulation.statistics.population.add_entry(&eye);
                spawn_organism(&mut commands, &config, body, eye, brain, selected, &mut rng);
            });
    }
}

pub fn dump_debug_info(
    simulation: Res<Simulation>,
    config: Res<SimulationConfig>,
    organisms_debug: Query<(&Position, &Locomotion, &Body), With<Organism>>,
    foods_debug: Query<&Position, With<Food>>,
) {
    if config.dump_debug_info {
        println!(
            "##### Generation:{} Step:{}",
            simulation.generation, simulation.steps
        );
        let mut organisms = organisms_debug.iter().collect::<Vec<_>>();
        organisms.sort_by(|o1, o2| (o1.0.x, o1.0.y).partial_cmp(&(o2.0.x, o2.0.y)).unwrap());
        for (position, locomotion, body) in organisms.iter() {
            println!(
                "O: ({:.2},{:.2})%{:.2} V:{} ; E:{:.2}",
                position.x,
                position.y,
                position.angle(),
                locomotion.linear,
                body.energy()
            );
        }
        let mut foods = foods_debug.iter().collect::<Vec<_>>();
        foods.sort_by(|o1, o2| (o1.x, o1.y).partial_cmp(&(o2.x, o2.y)).unwrap());
        for position in foods.iter() {
            println!("F: ({},{})", position.x, position.y,);
        }
    }
}
