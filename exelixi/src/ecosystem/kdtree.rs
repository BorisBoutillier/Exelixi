use std::collections::BTreeMap;

use kd_tree::{KdPoint, KdTree};

use crate::ecosystem::*;

pub struct KdTreeEntry {
    pub position: Position,
    pub entity: Entity,
}
impl KdTreeEntry {
    pub fn new(position: &Position, entity: Entity) -> Self {
        Self {
            position: *position,
            entity,
        }
    }
}
impl KdPoint for KdTreeEntry {
    type Scalar = f32;

    type Dim = typenum::U2;

    fn at(&self, i: usize) -> Self::Scalar {
        match i {
            0 => self.position.x,
            1 => self.position.y,
            _ => panic!(),
        }
    }
}

#[derive(Resource, Default)]
pub struct OrganismKdTree {
    pub per_species: BTreeMap<SpeciesId, KdTree<KdTreeEntry>>,
}

pub fn build_organism_kdtree(
    mut organism_kdtree: ResMut<OrganismKdTree>,
    organisms: Query<(Entity, &Organism, &Position)>,
    ecosystem_config: Res<EcosystemConfig>,
) {
    let mut per_species = ecosystem_config
        .species
        .keys()
        .map(|k| (*k, vec![]))
        .collect::<BTreeMap<_, _>>();
    for (entity, organism, position) in organisms.iter() {
        per_species
            .entry(organism.species)
            .and_modify(|v| v.push(KdTreeEntry::new(position, entity)));
    }
    organism_kdtree.per_species.clear();
    for (id, entries) in per_species {
        organism_kdtree
            .per_species
            .insert(id, KdTree::build_by_ordered_float(entries));
    }
}
