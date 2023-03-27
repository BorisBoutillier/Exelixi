use std::f32::consts::PI;

use crate::ecosystem::*;

pub fn spawn_organism(
    commands: &mut Commands,
    config: &EcosystemConfig,
    body: Body,
    eye: Eye,
    brain: Brain,
    rng: &mut EcosystemRng,
) {
    let half_width = config.environment.width / 2;
    let half_height = config.environment.height / 2;
    let angle = rng.0.gen_range(-PI..PI);
    let x = rng.0.gen_range(-half_width..half_width);
    let y = rng.0.gen_range(-half_height..half_height);
    commands.spawn((
        Organism {
            kind: OrganismKind::Herbivore,
        },
        Position::new(x as f32, y as f32, angle),
        Locomotion::new(config),
        Mouth {
            reach: 10.0,
            edible: vec![OrganismKind::Plant],
        },
        body,
        eye,
        brain,
    ));
}
pub fn spawn_starting_organisms(
    mut commands: Commands,
    mut simulation: ResMut<Simulation>,
    config: Res<EcosystemConfig>,
    mut rng: ResMut<EcosystemRng>,
) {
    if config.is_changed() {
        // Create a new random population
        let new_population = (0..config.min_population)
            .map(|_| OrganismIndividual::random(&mut rng.0, &config))
            .collect::<Vec<_>>();
        simulation
            .statistics
            .start_of_new_generation(&new_population, &config);
        simulation.new_generation();
        // Spawn the organisms
        new_population.into_iter().for_each(|individual| {
            let (body, eye, brain) = individual.into_components(&config);
            simulation.statistics.population.add_entry(&eye);
            spawn_organism(&mut commands, &config, body, eye, brain, &mut rng);
        });
    }
}
