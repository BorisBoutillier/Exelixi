use crate::ecosystem::{organism::reproduction::individual::OrganismIndividual, *};

#[derive(Debug)]
pub struct NewGenerationEvent {
    pub species: SpeciesId,
    pub generation: u32,
}

#[allow(clippy::too_many_arguments)]
pub fn evolve(
    mut commands: Commands,
    mut simulation: ResMut<Simulation>,
    config: Res<EcosystemConfig>,
    organisms: Query<(Entity, &Organism, &Body, &Brain, Option<&Eye>)>,
    mut rng: ResMut<EcosystemRng>,
    mut new_generation_events: EventWriter<NewGenerationEvent>,
    mut generation_evolutions: ResMut<GenerationEvolutions>,
) {
    simulation.steps += 1;
    for (species, state) in generation_evolutions.per_species.iter_mut() {
        if simulation.steps % state.generation_length == 0 {
            let current_population = organisms
                .iter()
                .filter(|(_, organism, _, _, _)| &organism.species() == species)
                .map(|(entity, _, body, brain, eye)| {
                    commands.entity(entity).despawn_recursive();
                    OrganismIndividual::from_components(&state.config, body, &eye, brain)
                })
                .collect::<Vec<_>>();
            state.current_generation += 1;
            let total_energy = organisms
                .iter()
                .filter(|(_, organism, _, _, _)| &organism.species() == species)
                .map(|(_, _, b, _, _)| b.energy())
                .sum::<f32>();

            let mut new_population = state.genetic_algorithm.evolve(
                &mut rng.0,
                &current_population,
                state.fertility_rate,
                (state.minimum_population as f32 * 0.9) as usize,
            );
            let n_evolve = new_population.len();
            let evolve_energy = total_energy / n_evolve as f32;
            // If not enough survived, add random organisms
            let missing_population = state.minimum_population as i32 - new_population.len() as i32;
            for _ in 0..missing_population {
                new_population.push(OrganismIndividual::random(&mut rng.0, &state.config));
            }

            new_generation_events.send(NewGenerationEvent {
                species: *species,
                generation: state.current_generation,
            });
            // Spawn new organisms
            new_population
                .into_iter()
                .enumerate()
                .for_each(|(i, individual)| {
                    let (mut body, eye, locomotion, brain) =
                        individual.into_components(&state.config);
                    if i < n_evolve {
                        body.set_energy(evolve_energy);
                    }
                    spawn_organism(
                        &mut commands,
                        &config,
                        &state.config,
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

use std::f32::consts::PI;

#[allow(clippy::too_many_arguments)]
pub fn spawn_organism(
    commands: &mut Commands,
    config: &EcosystemConfig,
    organism_config: &SpeciesConfig,
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
        Organism::new(organism_config),
        Position::new(x as f32, y as f32, angle),
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
    if let Some(mouth_config) = &organism_config.mouth {
        command.insert(Mouth::new(mouth_config));
    }
}
pub fn spawn_starting_organisms(
    mut commands: Commands,
    config: Res<EcosystemConfig>,
    mut rng: ResMut<EcosystemRng>,
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
            } = organism_config.reproduction
            {
                generation_evolutions
                    .per_species
                    .insert(*species_id, GenerationEvolution::new(organism_config));
                // Create a new random population
                let new_population = (0..min_population)
                    .map(|_| OrganismIndividual::random(&mut rng.0, organism_config))
                    .collect::<Vec<_>>();
                // Spawn the organisms
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
