use crate::*;

pub fn energy(
    mut commands: Commands,
    mut energy_query: Query<(Entity, &mut Body, &Brain, &Eye, &Locomotion)>,
) {
    let mut deads = vec![];
    for (entity, mut body, brain, eye, locomotion) in energy_query.iter_mut() {
        let total_cost = body.energy_cost() as f32
            + brain.energy_cost()
            + eye.energy_cost()
            + locomotion.energy_cost();
        if !body.spend_energy(total_cost as i32) {
            deads.push(entity);
        }
    }
    for dead in deads.iter() {
        commands.entity(*dead).despawn_recursive();
    }
}
