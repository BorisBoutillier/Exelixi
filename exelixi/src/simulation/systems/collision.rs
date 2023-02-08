use crate::prelude::*;

pub fn collision(
    mut commands: Commands,
    mut organisms: Query<(&mut Body, &Transform)>,
    mut foods: Query<(Entity, &Transform, &mut Food)>,
) {
    for (mut organism_body, organism_transform) in organisms.iter_mut() {
        for (entity, food_transform, mut food) in foods.iter_mut() {
            if !food.eaten {
                let distance =
                    (organism_transform.translation - food_transform.translation).length();
                if distance <= 10.0 {
                    organism_body.add_energy(food.energy);
                    // Storing the eaten state is currently necessary, because despawn will not
                    // happen when we do multiple steps per run_criteria
                    food.eaten = true;
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}
