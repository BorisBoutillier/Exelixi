use crate::ecosystem::{organism::reproduction::individual::OrganismIndividual, *};

pub struct NewGenerationEvent {
    pub generation: u32,
}

#[allow(clippy::too_many_arguments)]
pub fn evolve(
    mut commands: Commands,
    mut simulation: ResMut<Simulation>,
    config: Res<EcosystemConfig>,
    organisms: Query<(Entity, &Body, &Brain, Option<&Eye>)>,
    mut rng: ResMut<EcosystemRng>,
    mut new_generation_events: EventWriter<NewGenerationEvent>,
) {
    simulation.steps += 1;
    for organism_config in config.organisms.iter() {
        if let ReproductionConfig::EndOfGenerationEvolution {
            generation_length,
            min_population,
            fertility_rate,
            mutation_chance: _,
            mutation_amplitude: _,
        } = organism_config.reproduction
        {
            if simulation.steps == generation_length {
                simulation.steps = 0;

                let current_population = organisms
                    .iter()
                    .map(|(entity, body, brain, eye)| {
                        commands.entity(entity).despawn_recursive();
                        OrganismIndividual::from_components(organism_config, body, &eye, brain)
                    })
                    .collect::<Vec<_>>();
                simulation.statistics.end_of_generation(&current_population);
                println!("{}", simulation.sprint_state(&config));

                let mut new_population = simulation.ga.evolve(
                    &mut rng.0,
                    &current_population,
                    fertility_rate,
                    (min_population as f32 * 0.9) as usize, // If survivors and fertility are not enough keep 10% random
                );
                // If not enough survived, add random organisms
                let missing_population = min_population as i32 - new_population.len() as i32;
                for _ in 0..missing_population {
                    new_population.push(OrganismIndividual::random(&mut rng.0, organism_config));
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
                    let (body, eye, locomotion, brain) =
                        individual.into_components(organism_config);
                    spawn_organism(
                        &mut commands,
                        &config,
                        organism_config,
                        body,
                        eye,
                        locomotion,
                        brain,
                        &mut rng,
                    );
                });
            }
        }
    }
}

use std::f32::consts::PI;

#[allow(clippy::too_many_arguments)]
pub fn spawn_organism(
    commands: &mut Commands,
    config: &EcosystemConfig,
    organism_config: &OrganismConfig,
    body: Body,
    eye: Option<Eye>,
    locomotion: Option<Locomotion>,
    brain: Brain,
    rng: &mut EcosystemRng,
) {
    let half_width = config.environment.width / 2;
    let half_height = config.environment.height / 2;
    let angle = rng.0.gen_range(-PI..PI);
    let x = rng.0.gen_range(-half_width..half_width);
    let y = rng.0.gen_range(-half_height..half_height);
    let mut command = commands.spawn((
        Organism {
            kind: OrganismKind::Herbivore,
        },
        Position::new(x as f32, y as f32, angle),
        Mouth {
            reach: 10.0,
            edible: vec![OrganismKind::Plant],
        },
        body,
        brain,
    ));
    if let Some(locomotion) = locomotion {
        command.insert(locomotion);
    }
    if let Some(eye) = eye {
        command.insert(eye);
    }
    if let Some(leaf_config) = &organism_config.leaf {
        command.insert(Leaf::new(leaf_config));
    }
}
pub fn spawn_starting_organisms(
    mut commands: Commands,
    mut simulation: ResMut<Simulation>,
    config: Res<EcosystemConfig>,
    mut rng: ResMut<EcosystemRng>,
) {
    if config.is_changed() {
        for organism_config in config.organisms.iter() {
            if let ReproductionConfig::EndOfGenerationEvolution {
                generation_length: _,
                min_population,
                fertility_rate: _,
                mutation_chance: _,
                mutation_amplitude: _,
            } = organism_config.reproduction
            {
                // Create a new random population
                let new_population = (0..min_population)
                    .map(|_| OrganismIndividual::random(&mut rng.0, organism_config))
                    .collect::<Vec<_>>();
                simulation
                    .statistics
                    .start_of_new_generation(&new_population, &config);
                simulation.new_generation();
                // Spawn the organisms
                new_population.into_iter().for_each(|individual| {
                    let (body, eye, locomotion, brain) =
                        individual.into_components(organism_config);
                    //simulation.statistics.population.add_entry(&eye);
                    spawn_organism(
                        &mut commands,
                        &config,
                        organism_config,
                        body,
                        eye,
                        locomotion,
                        brain,
                        &mut rng,
                    );
                });
            }
        }
    }
}
