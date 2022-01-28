use crate::*;

#[derive(Clone)]
pub struct AnimalIndividual {
    fitness: f32,
    chromosome: ga::Chromosome,
}

impl ga::Individual for AnimalIndividual {
    fn chromosome(&self) -> &ga::Chromosome {
        &self.chromosome
    }
    fn create(chromosome: ga::Chromosome) -> Self {
        Self {
            fitness: 0.0,
            chromosome,
        }
    }
    fn fitness(&self) -> f32 {
        self.fitness
    }
}
impl AnimalIndividual {
    pub fn from_components(
        config: &SimulationConfig,
        stomach: &Stomach,
        eye: &Eye,
        brain: &Brain,
    ) -> Self {
        let mut eye_chromosome = eye.as_chromosome(config);
        let brain_chromosome = brain.as_chromosome();
        //println!("Size {} {}", eye_chromosome.len(), brain_chromosome.len());
        eye_chromosome.extend(brain_chromosome);
        //println!("  -> {}", eye_chromosome.len());
        Self {
            fitness: stomach.satiation as f32,
            chromosome: eye_chromosome,
        }
    }
    pub fn into_components(self, config: &SimulationConfig) -> (Eye, Brain) {
        let mut genes = self.chromosome.into_iter();
        let eye = Eye::from_genes(&mut genes, config);
        let brain = Brain::from_genes(&mut genes, &eye);
        (eye, brain)
    }
}
