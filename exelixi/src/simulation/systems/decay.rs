use crate::*;

pub fn decay(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Decay)>,
    mut simulation: ResMut<Simulation>,
) {
    for (entity, mut decay) in query.iter_mut() {
        decay.time -= 1;
        if decay.time <= 0 {
            commands.entity(entity).despawn();
            simulation.statistics.food_decay += 1;
        }
    }
}
