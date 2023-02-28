use crate::prelude::*;

pub fn spawn_organism(
    commands: &mut Commands,
    config: &SimulationConfig,
    body: Body,
    eye: Eye,
    brain: Brain,
    selected: bool,
    rng: &mut MyRng,
) {
    let half_width = config.environment.width / 2;
    let half_height = config.environment.height / 2;
    let angle = rng.0.gen_range(-PI..PI);
    let x = rng.0.gen_range(-half_width..half_width);
    let y = rng.0.gen_range(-half_height..half_height);
    let mut command = commands.spawn((
        Organism {},
        Position::new(x as f32, y as f32, angle),
        Locomotion::new(config),
        body,
        eye,
        brain,
    ));
    if selected {
        command.insert(Selected);
    }
}
pub fn spawn_starting_organisms(
    mut commands: Commands,
    mut simulation: ResMut<Simulation>,
    config: Res<SimulationConfig>,
    mut rng: ResMut<MyRng>,
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
