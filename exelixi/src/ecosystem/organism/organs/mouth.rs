use std::collections::{HashMap, HashSet};

use crate::ecosystem::*;

#[derive(Component)]
pub struct Mouth {
    // Maximum distance the mouth can catch something
    pub reach: f32,
    // Vec of type of organism kind that are edible for this one
    pub edible: Vec<OrganismKind>,
}

// Each organism mouth will eat the closest reachable other organisms.
pub fn mouth_eating(
    mut commands: Commands,
    mut organisms: Query<(Entity, &Organism, &Position, Option<&Mouth>), With<Body>>,
    mut bodies: Query<&mut Body>,
) {
    let mut per_kind = HashMap::new();
    for (e, o, p, _) in organisms.iter() {
        per_kind.entry(o.kind).or_insert(vec![]).push((e, *p));
    }
    // Store for each eatable organisms, the list of each organism that want to eat it
    // with the distance it is at.
    // Only the closest will be able to eat it.
    let mut want_to_eat = HashMap::new();
    for (entity, _, position, mouth) in organisms.iter_mut() {
        if let Some(mouth) = mouth {
            let mouth_reach_squared = mouth.reach.powi(2);
            for kind in mouth.edible.iter() {
                if !per_kind.contains_key(kind) {
                    continue;
                }
                for (other_entity, other_position) in per_kind[kind].iter() {
                    let dist_squared = position.distance_squared(other_position);
                    if *other_entity != entity && dist_squared <= mouth_reach_squared {
                        want_to_eat
                            .entry(*other_entity)
                            .or_insert(vec![])
                            .push((dist_squared, entity));
                    }
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
            commands.entity(eaten_entity).despawn();
        }
    }
}
