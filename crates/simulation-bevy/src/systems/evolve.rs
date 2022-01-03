use crate::prelude::*;

pub fn evolve(
    mut commands: Commands,
    mut simulation: ResMut<Simulation>,
    mut animals: Query<(Entity, &mut Stomach, &Brain, &Eye)>,
    mut transforms: Query<&mut Transform, Without<Camera>>,
    windows: Res<Windows>,
) {
    simulation.age += 1;
    if simulation.age == GENERATION_LENGTH {
        let mut rng = thread_rng();
        let window = windows.get_primary().unwrap();
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
            transform.translation.x =
                rng.gen_range((-window.width() / 2.0)..(window.width() / 2.0));
            transform.translation.y =
                rng.gen_range((-window.height() / 2.0)..(window.height() / 2.0));
            transform.rotation = Quat::from_axis_angle(Vec3::Z, rng.gen_range(-PI..PI));
        }
    }
}
