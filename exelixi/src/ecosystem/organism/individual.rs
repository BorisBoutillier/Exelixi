use crate::ecosystem::*;
use lib_genetic_algorithm as ga;

pub struct OrganismIndividual {
    pub energy: f32,
    chromosome: ga::Chromosome,
}

impl ga::Individual for OrganismIndividual {
    fn chromosome(&self) -> &ga::Chromosome {
        &self.chromosome
    }
    fn create(chromosome: ga::Chromosome) -> Self {
        Self {
            energy: 0.0,
            chromosome,
        }
    }
    fn fitness(&self) -> f32 {
        self.energy
    }
}
impl OrganismIndividual {
    pub fn from_components(
        config: &EcosystemConfig,
        body: &Body,
        eye: &Eye,
        brain: &Brain,
    ) -> Self {
        let mut eye_chromosome = eye.as_chromosome(config);
        let brain_chromosome = brain.as_chromosome();
        //println!("Size {} {}", eye_chromosome.len(), brain_chromosome.len());
        eye_chromosome.extend(brain_chromosome);
        //println!("  -> {}", eye_chromosome.len());
        Self {
            energy: body.energy() as f32,
            chromosome: eye_chromosome,
        }
    }
    pub fn into_components(self, config: &EcosystemConfig) -> (Body, Eye, Brain) {
        let mut genes = self.chromosome.into_iter();
        let body = Body::new(
            config.organisms.starting_energy,
            config.organisms.maximum_energy,
            config.organisms.body_cost,
        );
        let locomotion = Locomotion::new(config);
        let eye = Eye::from_genes(&mut genes, config);
        let brain = Brain::from_genes(&mut genes, &body, &eye, &locomotion);
        (body, eye, brain)
    }
    pub fn random(mut rng: &mut dyn RngCore, config: &EcosystemConfig) -> Self {
        let body = Body::new(
            config.organisms.starting_energy,
            config.organisms.maximum_energy,
            config.organisms.body_cost,
        );
        let locomotion = Locomotion::new(config);
        let eye = Eye::random(&mut rng, config);
        let brain = Brain::random(&mut rng, &body, &eye, &locomotion);
        Self::from_components(config, &body, &eye, &brain)
    }
}
