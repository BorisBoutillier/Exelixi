use std::collections::{BTreeMap, HashMap, HashSet};

use crate::ecosystem::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Mouth {
    // Maximum distance the mouth can catch something
    pub reach: f32,
    // Vec of name of organisms that are edible for this one
    pub edible: Vec<SpeciesId>,
}

impl Mouth {
    pub fn new(config: &MouthConfig) -> Self {
        Self {
            reach: config.reach,
            edible: config.edible_species.clone(),
        }
    }
}

// Each organism mouth will try to eat the closest reachable other organisms.
// When multiple organism when to eat the same target, only the closest one will eat it.
pub fn mouth_eating(
    mut commands: Commands,
    mut eaters: Query<(Entity, &Position, &Mouth)>,
    kdtree: Res<OrganismKdTree>,
    mut bodies: Query<&mut Body>,
) {
    // Store for each eatable organisms, the list of each organism that want to eat it
    // with the distance it is at.
    // Only the closest will be able to eat it.
    let mut foods = BTreeMap::new();
    for (entity, position, mouth) in eaters.iter_mut() {
        for species in mouth.edible.iter() {
            let mut food = None;
            for other in kdtree.per_species[species]
                .within_radius(&KdTreeEntry::new(position, entity), mouth.reach)
            {
                let distance = position.distance_squared(&other.position);
                if let Some((_, food_distance)) = food {
                    if distance < food_distance {
                        food = Some((other.entity, distance));
                    }
                } else {
                    food = Some((other.entity, distance))
                }
            }
            if let Some((food_entity, food_distance)) = food {
                foods
                    .entry(food_entity)
                    .or_insert(vec![])
                    .push((food_distance, entity));
            }
        }
    }
    let mut has_eaten = HashSet::new();
    // We store the energy of each eaten organism before applying any mouth eating
    // so that eaten energy is independent of order of mouth eating.
    let food_energy = HashMap::<Entity, f32>::from_iter(
        foods.keys().map(|e| (*e, bodies.get(*e).unwrap().energy())),
    );
    for (food_entity, mut eaters) in foods.into_iter() {
        eaters.sort_by(|(d1, _), (d2, _)| d1.partial_cmp(d2).unwrap());
        if let Some((_, e)) = eaters.into_iter().find(|(_, e)| !has_eaten.contains(e)) {
            bodies
                .get_mut(e)
                .unwrap()
                .add_energy(food_energy[&food_entity]);
            has_eaten.insert(e);
            commands.entity(food_entity).despawn_recursive();
        }
    }
}
