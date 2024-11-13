use lib_genetic_algorithm::Individual;

use crate::ecosystem::{organism::reproduction::individual::OrganismIndividual, *};

#[allow(clippy::too_many_arguments)]
pub fn evolve(
    mut commands: Commands,
    config: Res<EcosystemConfig>,
    organisms: Query<(Entity, &Organism, &Position, &Body, &Brain, Option<&Eye>)>,
    mut ecosystem: ResMut<EcosystemRuntime>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    mut generation_evolutions: ResMut<GenerationEvolutions>,
    mut spawn_events: EventWriter<SpawnOrganismEvent>,
) {
    for (species, state) in generation_evolutions.per_species.iter_mut() {
        if ecosystem.steps % state.generation_length == 0 {
            let minimum_population = config.species[species].minimum_population;
            let mut organisms = organisms.iter().collect::<Vec<_>>();
            organisms.sort_unstable_by(|(_, _, p1, _, _, _), (_, _, p2, _, _, _)| {
                p1.partial_cmp(p2).unwrap()
            });
            let current_population = organisms
                .iter()
                .filter(|(_, organism, _, _, _, _)| &organism.species() == species)
                .map(|(entity, _, _, body, brain, eye)| {
                    commands.entity(*entity).despawn_recursive();
                    ecosystem.decrease_population(species);
                    OrganismIndividual::from_components(&state.config, body, eye, brain)
                })
                .collect::<Vec<_>>();
            ecosystem.increment_generation(species);
            let total_energy = organisms
                .iter()
                .filter(|(_, organism, _, _, _, _)| &organism.species() == species)
                .map(|(_, _, _, b, _, _)| b.energy())
                .sum::<f32>();

            let new_population = state.genetic_algorithm.evolve(
                &mut *rng,
                &current_population,
                state.fertility_rate,
                // We want 90% of the minimum population to be created from evolving previous generation creature
                // This is let at most 10% of minimum population be fully random.
                (minimum_population as f32 * 0.9) as usize,
            );
            let n_evolve = new_population.len();
            let evolve_energy = total_energy / n_evolve as f32;
            //
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
                    let angle = rng.gen_range(-PI..PI);
                    let (x, y) = match (current_positions.is_empty(), state.child_spawn_distance) {
                        (false, Some(distance)) => {
                            let dx = rng.gen_range(-distance..distance);
                            let dy = rng.gen_range(-distance..distance);
                            (
                                (current_positions[i % current_positions.len()].x + dx)
                                    .clamp(-half_width, half_width),
                                (current_positions[i % current_positions.len()].y + dy)
                                    .clamp(-half_height, half_height),
                            )
                        }
                        _ => (
                            rng.gen_range(-half_width..half_width),
                            rng.gen_range(-half_height..half_height),
                        ),
                    };
                    let position = Position::new(x, y, angle);
                    spawn_events.send(SpawnOrganismEvent {
                        species: state.config.id,
                        position: Some(position),
                        energy: Some(evolve_energy),
                        chromosome: Some(individual.chromosome().clone()),
                    });
                });
        }
    }
    // Increase steps at the end so that we can have a first evolution at step = 0
    ecosystem.steps += 1;
}

use std::f32::consts::PI;
