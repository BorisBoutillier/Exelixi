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
    pub fn from_stomach_and_brain(stomach: &Stomach, brain: &Brain) -> Self {
        Self {
            fitness: stomach.satiation as f32,
            chromosome: brain.as_chromosome(),
        }
    }
    pub fn into_brain(self, eye: &Eye) -> Brain {
        Brain::from_chromosome(self.chromosome, eye)
    }
}
