use crate::*;

pub fn process_brain(
    mut organisms: Query<(Entity, &Position, &Transform, &mut Locomotion, &Eye, &Brain)>,
    food_transforms: Query<&Transform, With<Food>>,
    organism_transforms: Query<(Entity, &Transform), With<Organism>>,
    config: Res<SimulationConfig>,
) {
    let food_transforms = food_transforms.iter().collect::<Vec<_>>();
    organisms.for_each_mut(
        |(
            organism_entity,
            organism_position,
            organism_transform,
            mut organism_locomotion,
            organism_eye,
            organism_brain,
        )| {
            let organism_transforms = organism_transforms
                .iter()
                .filter_map(|(e, t)| if e != organism_entity { Some(t) } else { None })
                .collect::<Vec<_>>();
            let vision = organism_eye.process_vision(
                organism_position,
                organism_transform,
                &food_transforms,
                &organism_transforms,
                &config,
            );
            let response = organism_brain.nn.propagate(&vision);
            organism_locomotion.actuates(response);
        },
    );
}
