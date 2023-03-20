use crate::prelude::*;

#[derive(Component)]
pub struct Mouth {
    // Maximum distance the mouth can catch something
    pub reach: f32,
}

pub fn mouth_eating(
    mut commands: Commands,
    mut organisms: Query<(&mut Body, &Position, &Mouth)>,
    mut foods: Query<(Entity, &Position, &mut Food)>,
) {
    for (mut body, position, mouth) in organisms.iter_mut() {
        let mouth_reach_pow_2 = mouth.reach.powf(2.);
        for (food_entity, food_position, mut food) in foods.iter_mut() {
            if !food.eaten {
                let dist_pow2 =
                    (position.x - food_position.x).powi(2) + (position.y - food_position.y).powi(2);
                if dist_pow2 <= mouth_reach_pow_2 {
                    body.add_energy(food.energy);
                    // Storing the eaten state is currently necessary, because despawn will not
                    // happen when we do multiple steps per run_criteria
                    food.eaten = true;
                    commands.entity(food_entity).despawn();
                }
            }
        }
    }
}
