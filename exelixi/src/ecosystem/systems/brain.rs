use crate::*;

pub fn process_brain(
    mut organisms: Query<(Entity, &Body, &Position, &mut Locomotion, &Eye, &Brain)>,
    food_positions: Query<&Position, With<Food>>,
    organism_positions: Query<(Entity, &Position), With<Organism>>,
    config: Res<SimulationConfig>,
) {
    let food_positions = food_positions.iter().collect::<Vec<_>>();
    organisms.for_each_mut(|(entity, body, position, mut locomotion, eye, brain)| {
        let organism_positions = organism_positions
            .iter()
            .filter_map(|(e, t)| if e != entity { Some(t) } else { None })
            .collect::<Vec<_>>();
        let mut inputs =
            eye.process_vision(position, &food_positions, &organism_positions, &config);
        inputs.extend(body.get_sensors().iter());
        let response = brain.nn.propagate(&inputs);
        locomotion.actuates(response);
    });
}
