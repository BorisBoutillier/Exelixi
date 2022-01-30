use crate::*;

pub fn energy(
    mut commands: Commands,
    mut energy_query: Query<(Entity, &mut Body, Option<&Brain>, Option<&Eye>)>,
) {
    let mut deads = vec![];
    for (entity, mut body, _brain, _eye) in energy_query.iter_mut() {
        body.energy -= 0.1;
        if body.energy < 0.0 {
            deads.push(entity);
        }
    }
    for dead in deads.iter() {
        commands.entity(*dead).despawn_recursive();
    }
}
