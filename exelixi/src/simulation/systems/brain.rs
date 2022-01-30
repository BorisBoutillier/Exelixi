use crate::*;

pub fn process_brain(
    mut animals: Query<(Entity, &Transform, &mut Locomotion, &Eye, &Brain)>,
    food_transforms: Query<&Transform, With<Food>>,
    animal_transforms: Query<(Entity, &Transform), With<Animal>>,
    config: Res<SimulationConfig>,
) {
    let food_transforms = food_transforms.iter().collect::<Vec<_>>();
    animals.for_each_mut(
        |(animal_entity, animal_transform, mut animal_locomotion, animal_eye, animal_brain)| {
            let animal_transforms = animal_transforms
                .iter()
                .filter_map(|(e, t)| if e != animal_entity { Some(t) } else { None })
                .collect::<Vec<_>>();
            let vision = animal_eye.process_vision(
                animal_transform,
                &food_transforms,
                &animal_transforms,
                &config,
            );
            let response = animal_brain.nn.propagate(&vision);
            animal_locomotion.actuates(response);
        },
    );
}
