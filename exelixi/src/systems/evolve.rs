use crate::prelude::*;

pub fn evolve(
    mut commands: Commands,
    mut simulation: ResMut<Simulation>,
    mut animals: Query<(Entity, &mut Stomach, &Brain, &Eye)>,
    mut transforms: Query<&mut Transform, Or<(With<Food>, With<Animal>)>>,
    config: Res<SimulationConfig>,
) {
    simulation.age += 1;
    if simulation.age == config.generation_length {
        let half_width = config.environment_size.width / 2.0;
        let half_height = config.environment_size.height / 2.0;
        let mut rng = thread_rng();
        simulation.age = 0;
        simulation.generation += 1;

        let current_population = animals
            .iter()
            .map(|(_, s, b, _)| AnimalIndividual::from_stomach_and_brain(s, b))
            .collect::<Vec<_>>();
        let (new_population, stats) = simulation.ga.evolve(&mut rng, &current_population);
        for (i, (entity, _, _, eye)) in animals.iter_mut().enumerate() {
            let brain = new_population[i].clone().into_brain(eye);
            commands
                .entity(entity)
                .insert(brain)
                .insert(Stomach::default());
        }
        simulation.statistics = stats;
        // Reset all transform
        for mut transform in transforms.iter_mut() {
            transform.translation.x = rng.gen_range(-half_width..half_width);
            transform.translation.y = rng.gen_range(-half_height..half_height);
            transform.rotation = Quat::from_axis_angle(Vec3::Z, rng.gen_range(-PI..PI));
        }
        println!(
            "Gen: {:03} , Avg: {:.1}",
            simulation.generation,
            simulation.statistics.avg_fitness()
        );
    }
}
