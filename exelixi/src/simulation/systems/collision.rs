use crate::prelude::*;

pub fn collision(
    mut commands: Commands,
    mut animals: Query<(&mut Stomach, &Transform)>,
    mut foods: Query<(Entity, &Transform, &mut Food)>,
) {
    for (mut animal_stomach, animal_transform) in animals.iter_mut() {
        for (entity, food_transform, mut food) in foods.iter_mut() {
            if !food.eaten {
                let distance = (animal_transform.translation - food_transform.translation).length();
                if distance <= 10.0 {
                    animal_stomach.satiation += 1.0;
                    // Storing the eaten state is currently necessary, because despawn will not
                    // happen when we do multiple steps per run_criteria
                    food.eaten = true;
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}
