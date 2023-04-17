use std::collections::HashMap;

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
    pub per_name: HashMap<String, KdTree<KdTreeEntry>>,
}

pub fn build_organism_kdtree(
    mut organism_kdtree: ResMut<OrganismKdTree>,
    organisms: Query<(Entity, &Organism, &Position)>,
    ecosystem_config: Res<EcosystemConfig>,
) {
    let mut per_name = HashMap::new();
    for (entity, organism, position) in organisms.iter() {
        per_name
            .entry(organism.name().to_string())
            .or_insert(vec![])
            .push(KdTreeEntry::new(position, entity));
    }
    organism_kdtree.per_name.clear();
    for name in ecosystem_config.organisms_per_name.keys() {
        let entries = per_name.remove(name).unwrap_or(vec![]);
        organism_kdtree
            .per_name
            .insert(name.to_string(), KdTree::build_by_ordered_float(entries));
    }
}
