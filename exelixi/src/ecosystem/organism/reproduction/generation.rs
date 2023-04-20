use crate::ecosystem::{organism::reproduction::individual::OrganismIndividual, *};

#[derive(Debug)]
pub struct NewGenerationEvent {
    pub species: SpeciesId,
    pub generation: u32,
}

#[allow(clippy::too_many_arguments)]
pub fn evolve(
    mut commands: Commands,
    config: Res<EcosystemConfig>,
    organisms: Query<(Entity, &Organism, &Position, &Body, &Brain, Option<&Eye>)>,
    mut ecosystem: ResMut<Ecosystem>,
    mut new_generation_events: EventWriter<NewGenerationEvent>,
    mut generation_evolutions: ResMut<GenerationEvolutions>,
) {
    ecosystem.steps += 1;
    for (species, state) in generation_evolutions.per_species.iter_mut() {
        if ecosystem.steps % state.generation_length == 0 {
            let current_population = organisms
                .iter()
                .filter(|(_, organism, _, _, _, _)| &organism.species() == species)
                .map(|(entity, _, _, body, brain, eye)| {
                    commands.entity(entity).despawn_recursive();
                    OrganismIndividual::from_components(&state.config, body, &eye, brain)
                })
                .collect::<Vec<_>>();
            state.current_generation += 1;
            let total_energy = organisms
                .iter()
                .filter(|(_, organism, _, _, _, _)| &organism.species() == species)
                .map(|(_, _, _, b, _, _)| b.energy())
                .sum::<f32>();

            let mut new_population = state.genetic_algorithm.evolve(
                &mut ecosystem.rng,
                &current_population,
                state.fertility_rate,
                (state.minimum_population as f32 * 0.9) as usize,
            );
            let n_evolve = new_population.len();
            let evolve_energy = total_energy / n_evolve as f32;
            // If not enough survived, add random organisms
            let missing_population = state.minimum_population as i32 - new_population.len() as i32;
            for _ in 0..missing_population {
                new_population.push(OrganismIndividual::random(
                    &mut ecosystem.rng,
                    &state.config,
                ));
            }

            new_generation_events.send(NewGenerationEvent {
                species: *species,
                generation: state.current_generation,
            });
            // Spawn new organisms
            let current_positions = organisms
                .iter()
                .filter(|(_, organism, _, _, _, _)| &organism.species() == species)
                .map(|(_, _, p, _, _, _)| p)
                .collect::<Vec<_>>();
            let half_width = config.environment.width as f32 / 2.0;
            let half_height = config.environment.height as f32 / 2.0;
            new_population
                .into_iter()
                .enumerate()
                .for_each(|(i, individual)| {
                    let (mut body, eye, locomotion, brain) =
                        individual.into_components(&state.config);
                    if i < n_evolve {
                        body.set_energy(evolve_energy);
                    }
                    let angle = ecosystem.rng.gen_range(-PI..PI);
                    let (x, y) = match (current_positions.is_empty(), state.child_spawn_distance) {
                        (false, Some(distance)) => {
                            let dx = ecosystem.rng.gen_range(-distance..distance);
                            let dy = ecosystem.rng.gen_range(-distance..distance);
                            (
                                (current_positions[i % current_positions.len()].x + dx)
                                    .clamp(-half_width, half_width),
                                (current_positions[i % current_positions.len()].y + dy)
                                    .clamp(-half_height, half_height),
                            )
                        }
                        _ => (
                            ecosystem.rng.gen_range(-half_width..half_width),
                            ecosystem.rng.gen_range(-half_height..half_height),
                        ),
                    };
                    let position = Position::new(x, y, angle);
                    spawn_organism(
                        &mut commands,
                        &state.config,
                        body,
                        eye,
                        locomotion,
                        brain,
                        position,
                    );
                });
        }
    }
}

use std::f32::consts::PI;

#[allow(clippy::too_many_arguments)]
pub fn spawn_organism(
    commands: &mut Commands,
    organism_config: &SpeciesConfig,
    body: Body,
    eye: Option<Eye>,
    locomotion: Option<Locomotion>,
    brain: Brain,
    position: Position,
) {
    let mut command = commands.spawn((Organism::new(organism_config), position, body, brain));
    if let Some(locomotion) = locomotion {
        command.insert(locomotion);
    }
    if let Some(eye) = eye {
        command.insert(eye);
    }
    if let Some(leaf_config) = &organism_config.leaf {
        command.insert(Leaf::new(leaf_config));
    }
    if let Some(mouth_config) = &organism_config.mouth {
        command.insert(Mouth::new(mouth_config));
    }
}
pub fn spawn_starting_organisms(
    mut commands: Commands,
    config: Res<EcosystemConfig>,
    mut ecosystem: ResMut<Ecosystem>,
    mut generation_evolutions: ResMut<GenerationEvolutions>,
) {
    if config.is_changed() {
        commands.insert_resource(EcosystemStatistics::new(&config));
        generation_evolutions.per_species.clear();
        for (species_id, organism_config) in config.species.iter() {
            if let ReproductionConfig::GenerationEvolution {
                generation_length: _,
                min_population,
                fertility_rate: _,
                mutation_chance: _,
                mutation_amplitude: _,
                child_spawn_distance: _,
            } = organism_config.reproduction
            {
                generation_evolutions
                    .per_species
                    .insert(*species_id, GenerationEvolution::new(organism_config));
                // Create a new random population
                let new_population = (0..min_population)
                    .map(|_| OrganismIndividual::random(&mut ecosystem.rng, organism_config))
                    .collect::<Vec<_>>();
                // Spawn the organisms
                new_population.into_iter().for_each(|individual| {
                    let (body, eye, locomotion, brain) =
                        individual.into_components(organism_config);
                    let half_width = config.environment.width / 2;
                    let half_height = config.environment.height / 2;
                    let angle = ecosystem.rng.gen_range(-PI..PI);
                    let x = ecosystem.rng.gen_range(-half_width..half_width);
                    let y = ecosystem.rng.gen_range(-half_height..half_height);
                    let position = Position::new(x as f32, y as f32, angle);
                    spawn_organism(
                        &mut commands,
                        organism_config,
                        body,
                        eye,
                        locomotion,
                        brain,
                        position,
                    );
                });
            }
        }
    }
}
