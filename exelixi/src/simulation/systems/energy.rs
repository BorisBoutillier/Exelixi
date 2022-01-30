use crate::*;

pub fn energy(
    mut commands: Commands,
    mut energy_query: Query<(Entity, &mut Body, &Brain, &Eye, &Locomotion)>,
) {
    let mut deads = vec![];
    for (entity, mut body, brain, eye, locomotion) in energy_query.iter_mut() {
        if !body.spend_energy(brain.energy_cost() + eye.energy_cost() + locomotion.energy_cost()) {
            deads.push(entity);
        }
    }
    for dead in deads.iter() {
        commands.entity(*dead).despawn_recursive();
    }
}
