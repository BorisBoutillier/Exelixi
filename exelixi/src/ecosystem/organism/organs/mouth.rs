use std::collections::{HashMap, HashSet};

use crate::ecosystem::*;

#[derive(Component)]
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

// Each organism mouth will eat the closest reachable other organisms.
pub fn mouth_eating(
    mut commands: Commands,
    mut eaters: Query<(Entity, &Position, &Mouth)>,
    kdtree: Res<OrganismKdTree>,
    mut bodies: Query<&mut Body>,
) {
    // Store for each eatable organisms, the list of each organism that want to eat it
    // with the distance it is at.
    // Only the closest will be able to eat it.
    let mut want_to_eat = HashMap::new();
    for (entity, position, mouth) in eaters.iter_mut() {
        for species in mouth.edible.iter() {
            for other in kdtree.per_species[species]
                .within_radius(&KdTreeEntry::new(position, entity), mouth.reach)
            {
                if other.entity != entity {
                    want_to_eat
                        .entry(other.entity)
                        .or_insert(vec![])
                        .push((position.distance_squared(&other.position), entity));
                }
            }
        }
    }
    let mut has_eaten = HashSet::new();
    // We store the energy of each eaten organism before applying any mouth eating
    // so that eaten energy is independant of order of mouth eating.
    let eaten_energy = HashMap::<Entity, f32>::from_iter(
        want_to_eat
            .keys()
            .map(|e| (*e, bodies.get(*e).unwrap().energy())),
    );
    for (eaten_entity, mut eaters) in want_to_eat.into_iter() {
        eaters.sort_by(|(d1, _), (d2, _)| d1.partial_cmp(d2).unwrap());
        if let Some((_, e)) = eaters.into_iter().find(|(_, e)| !has_eaten.contains(e)) {
            bodies
                .get_mut(e)
                .unwrap()
                .add_energy(eaten_energy[&eaten_entity]);
            has_eaten.insert(e);
            commands.entity(eaten_entity).despawn_recursive();
        }
    }
}
