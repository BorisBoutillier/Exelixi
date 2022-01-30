use crate::*;

pub struct AnimalIndividual {
    pub energy: f32,
    chromosome: ga::Chromosome,
}

impl ga::Individual for AnimalIndividual {
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
impl AnimalIndividual {
    pub fn from_components(
        config: &SimulationConfig,
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
            energy: body.energy(),
            chromosome: eye_chromosome,
        }
    }
    pub fn into_components(self, config: &SimulationConfig) -> (Eye, Brain) {
        let mut genes = self.chromosome.into_iter();
        let locomotion = Locomotion::new(config);
        let eye = Eye::from_genes(&mut genes, config);
        let brain = Brain::from_genes(&mut genes, &eye, &locomotion);
        (eye, brain)
    }
    pub fn random(mut rng: &mut dyn RngCore, config: &SimulationConfig) -> Self {
        let locomotion = Locomotion::new(config);
        let eye = Eye::random(&mut rng, config);
        let brain = Brain::random(&mut rng, &eye, &locomotion);
        let body = Body::new(config);
        Self::from_components(config, &body, &eye, &brain)
    }
}
