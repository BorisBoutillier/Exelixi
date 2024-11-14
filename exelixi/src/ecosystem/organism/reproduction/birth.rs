use lib_genetic_algorithm::{CrossoverMethod, Individual, MutationMethod};

use crate::ecosystem::*;

pub fn reproduction_birth(
    mut organisms: Query<(
        &Organism,
        &Position,
        &mut Body,
        &mut Uterus,
        Option<&Eye>,
        &Brain,
    )>,
    ecosystem_config: Res<EcosystemConfig>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    mut organisms_lifecycle: ResMut<OrganismsLifecycle>,
) {
    for (organism, position, mut body, mut uterus, eye, brain) in organisms.iter_mut() {
        let config = &ecosystem_config.species[&organism.species];
        if let ReproductionConfig::Birth {
            minimum_age,
            minimum_energy_pct,
            child_energy_pct,
            mutation_chance,
            mutation_amplitude,
        } = config.reproduction
        {
            if organism.age >= minimum_age && body.energy_pct() >= minimum_energy_pct {
                if let Some(other_chromosome) = uterus.chromosome.take() {
                    let individual =
                        OrganismIndividual::from_components(config, &body, &eye, brain);
                    let crossover_method = ga::UniformCrossover;
                    let mutation_method =
                        ga::GaussianMutation::new(mutation_chance, mutation_amplitude);
                    let mut child_chromosome = crossover_method.crossover(
                        &mut *rng,
                        individual.chromosome(),
                        &other_chromosome,
                    );
                    mutation_method.mutate(&mut *rng, &mut child_chromosome);
                    let child_energy = body.energy() * child_energy_pct;
                    let child_position = (*position).with_random_angle(&mut *rng);
                    let parent_energy = body.energy() - child_energy;
                    body.set_energy(parent_energy);
                    organisms_lifecycle.births.push(OrganismBirth {
                        species: organism.species,
                        position: Some(child_position),
                        energy: Some(child_energy),
                        chromosome: Some(child_chromosome),
                    });
                }
            }
        }
    }
}
