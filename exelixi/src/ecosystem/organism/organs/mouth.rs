use std::collections::HashSet;

use crate::ecosystem::*;

#[derive(Component)]
pub struct Mouth {
    // Maximum distance the mouth can catch something
    pub reach: f32,
}

pub fn mouth_eating(
    mut commands: Commands,
    mut organisms: Query<(Entity, &Organism, &mut Body, &Position, Option<&Mouth>)>,
) {
    let foods = organisms
        .iter()
        .filter_map(|(e, o, b, p, _)| {
            if o.kind == OrganismKind::Plant {
                Some((e, *p, b.energy()))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let mut eaten_foods = HashSet::new();
    for (_, _, mut body, position, mouth) in organisms.iter_mut() {
        if let Some(mouth) = mouth {
            let mouth_reach_squared = mouth.reach.powf(2.);
            for (food_entity, food_position, food_energy) in foods.iter() {
                if !eaten_foods.contains(food_entity)
                    && position.distance_squared(food_position) <= mouth_reach_squared
                {
                    body.add_energy(*food_energy);
                    // Storing the eaten state is currently necessary, because despawn will not
                    // happen when we do multiple steps per run_criteria
                    eaten_foods.insert(food_entity);
                    commands.entity(*food_entity).despawn();
                }
            }
        }
    }
}
