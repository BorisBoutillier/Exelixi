use crate::prelude::*;

pub fn collision(
    mut commands: Commands,
    mut animals: Query<(&mut Body, &Transform)>,
    mut foods: Query<(Entity, &Transform, &mut Food)>,
) {
    for (mut animal_body, animal_transform) in animals.iter_mut() {
        for (entity, food_transform, mut food) in foods.iter_mut() {
            if !food.eaten {
                let distance = (animal_transform.translation - food_transform.translation).length();
                if distance <= 10.0 {
                    animal_body.energy =
                        (animal_body.energy + food.energy).min(animal_body.max_energy);
                    // Storing the eaten state is currently necessary, because despawn will not
                    // happen when we do multiple steps per run_criteria
                    food.eaten = true;
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}
