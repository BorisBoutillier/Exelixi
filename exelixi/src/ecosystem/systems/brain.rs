use crate::*;

pub fn process_brain(
    mut organisms: Query<(Entity, &Position, &mut Locomotion, &Eye, &Brain)>,
    food_positions: Query<&Position, With<Food>>,
    organism_positions: Query<(Entity, &Position), With<Organism>>,
    config: Res<SimulationConfig>,
) {
    let food_positions = food_positions.iter().collect::<Vec<_>>();
    organisms.for_each_mut(
        |(
            organism_entity,
            organism_position,
            mut organism_locomotion,
            organism_eye,
            organism_brain,
        )| {
            let organism_positions = organism_positions
                .iter()
                .filter_map(|(e, t)| if e != organism_entity { Some(t) } else { None })
                .collect::<Vec<_>>();
            let vision = organism_eye.process_vision(
                organism_position,
                &food_positions,
                &organism_positions,
                &config,
            );
            let response = organism_brain.nn.propagate(&vision);
            organism_locomotion.actuates(response);
        },
    );
}
