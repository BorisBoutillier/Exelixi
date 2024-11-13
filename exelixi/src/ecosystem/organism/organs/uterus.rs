use lib_genetic_algorithm::{Chromosome, Individual};

use crate::ecosystem::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Uterus {
    // Defines the furthest distance an organism can be
    // to be able to mate with it.
    mating_distance: f32,
    // The Chromosome of the latest individual this organism
    // has mated with.
    // None while we have not yet mated, or we have given birth.
    chromosome: Option<Chromosome>,
}
impl Uterus {
    pub fn new(config: &UterusConfig) -> Self {
        Self {
            mating_distance: config.mating_distance,
            chromosome: None,
        }
    }
}
impl EnergyActor for Uterus {
    fn energy_consumed(&self) -> f32 {
        0.0
    }
}

// Each organism uterus will try to catch the chromosome of the closest
// organism of the same species within mating_distance.
// This mating is currently instantaneous and does not impact in any way the other organism
pub fn uterus_processing(
    config: Res<EcosystemConfig>,
    mut uteruses: Query<(Entity, &Position, &Organism, &mut Uterus)>,
    organisms: Query<(&Body, &Brain, Option<&Eye>)>,
    kdtree: Res<OrganismKdTree>,
) {
    for (entity, position, organism, mut uterus) in uteruses.iter_mut() {
        let species = organism.species();
        // Look for two nearest to position, as we will get ourself.
        let nearests =
            kdtree.per_species[&species].nearests(&KdTreeEntry::new(position, entity), 2);
        if let Some(nearest) = nearests.iter().find(|n| {
            n.item.entity != entity && n.squared_distance <= uterus.mating_distance.powi(2)
        }) {
            let other = nearest.item.entity;
            let (other_body, other_brain, other_eye) =
                organisms.get(other).expect("Mating organism without Body");
            uterus.chromosome = Some(
                OrganismIndividual::from_components(
                    &config.species[&species],
                    other_body,
                    &other_eye,
                    other_brain,
                )
                .chromosome()
                .clone(),
            );
        }
    }
}
