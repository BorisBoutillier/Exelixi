use crate::*;

pub fn decay(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Decay)>,
    mut simulation: ResMut<Simulation>,
) {
    let mut food_decay = 0;
    for (entity, mut decay) in query.iter_mut() {
        decay.time -= 1;
        if decay.time <= 0 {
            commands.entity(entity).despawn();
            food_decay += 1;
        }
    }
    simulation.statistics.add_food_decay(food_decay);
}
