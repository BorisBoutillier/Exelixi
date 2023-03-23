use crate::ecosystem::*;

pub struct NewGenerationEvent {
    pub generation: u32,
}

#[allow(clippy::too_many_arguments)]
pub fn evolve(
    mut commands: Commands,
    mut simulation: ResMut<Simulation>,
    config: Res<EcosystemConfig>,
    organisms: Query<(Entity, &Body, &Brain, &Eye)>,
    mut rng: ResMut<EcosystemRng>,
    mut new_generation_events: EventWriter<NewGenerationEvent>,
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
            config.organisms.fertility_rate,
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
        new_generation_events.send(NewGenerationEvent {
            generation: simulation.generation,
        });
        // Spawn new organisms
        new_population.into_iter().for_each(|individual| {
            let (body, eye, brain) = individual.into_components(&config);
            simulation.statistics.population.add_entry(&eye);
            spawn_organism(&mut commands, &config, body, eye, brain, &mut rng);
        });
    }
}
