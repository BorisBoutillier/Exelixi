use crate::ecosystem::*;
use ga::Chromosome;
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
        config: &OrganismConfig,
        body: &Body,
        eye: &Option<&Eye>,
        brain: &Brain,
    ) -> Self {
        let mut chromosome = Chromosome::default();
        if let Some(eye_config) = &config.eye {
            chromosome.extend(eye.unwrap().as_chromosome(eye_config));
        }
        let brain_chromosome = brain.as_chromosome();
        //println!("Size {} {}", eye_chromosome.len(), brain_chromosome.len());
        chromosome.extend(brain_chromosome);
        //println!("  -> {}", eye_chromosome.len());
        Self {
            energy: body.energy(),
            chromosome,
        }
    }
    pub fn into_components(
        self,
        config: &OrganismConfig,
    ) -> (Body, Option<Eye>, Option<Locomotion>, Brain) {
        let mut genes = self.chromosome.into_iter();
        let body = Body::new(&config.body);
        let locomotion = config.locomotion.as_ref().map(Locomotion::new);
        let eye = config
            .eye
            .as_ref()
            .map(|cfg| Eye::from_genes(&mut genes, cfg));
        let brain = Brain::from_genes(&mut genes, &body, &eye.as_ref(), &locomotion.as_ref());
        (body, eye, locomotion, brain)
    }
    pub fn random(mut rng: &mut dyn RngCore, config: &OrganismConfig) -> Self {
        let body = Body::new(&config.body);
        let locomotion = config.locomotion.as_ref().map(Locomotion::new);
        let eye = config.eye.as_ref().map(|cfg| Eye::random(&mut rng, cfg));
        let brain = Brain::random(&mut rng, &body, &eye.as_ref(), &locomotion.as_ref());
        Self::from_components(config, &body, &eye.as_ref(), &brain)
    }
}
