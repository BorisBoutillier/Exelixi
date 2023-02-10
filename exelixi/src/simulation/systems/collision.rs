use crate::prelude::*;

pub fn collision(
    mut commands: Commands,
    mut organisms: Query<(&mut Body, &Position)>,
    mut foods: Query<(Entity, &Position, &mut Food)>,
) {
    for (mut organism_body, organism_position) in organisms.iter_mut() {
        for (entity, food_position, mut food) in foods.iter_mut() {
            if !food.eaten {
                let distance = (organism_position.x - food_position.x).pow(2)
                    + (organism_position.y - food_position.y).pow(2);
                if distance <= 100 {
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
