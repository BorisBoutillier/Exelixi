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
        config: &SpeciesConfig,
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
            energy: body.energy,
            chromosome,
        }
    }
    pub fn into_components(
        self,
        config: &SpeciesConfig,
    ) -> (Body, Option<Eye>, Option<Locomotion>, Brain) {
        let mut genes = self.chromosome.into_iter();
        let body = Body::new(&config.body);
        let locomotion = config.locomotion.as_ref().map(Locomotion::new);
        let eye = config
            .eye
            .as_ref()
            .map(|cfg| Eye::from_genes(&mut genes, cfg));
        let mut n_sensors = body.n_sensors();
        if let Some(eye) = &eye {
            n_sensors += eye.n_sensors();
        }
        let mut n_actuators = 0;
        if let Some(locomotion) = &locomotion {
            n_actuators += locomotion.n_actuators();
        }
        let brain = Brain::from_genes(&mut genes, n_sensors, n_actuators);
        (body, eye, locomotion, brain)
    }
    pub fn random(mut rng: &mut dyn RngCore, config: &SpeciesConfig) -> Self {
        let body = Body::new(&config.body);
        let locomotion = config.locomotion.as_ref().map(Locomotion::new);
        let eye = config.eye.as_ref().map(|cfg| Eye::random(&mut rng, cfg));
        let mut n_sensors = body.n_sensors();
        if let Some(eye) = &eye {
            n_sensors += eye.n_sensors();
        }
        let mut n_actuators = 0;
        if let Some(locomotion) = &locomotion {
            n_actuators += locomotion.n_actuators();
        }
        let brain = Brain::random(&mut rng, n_sensors, n_actuators);
        Self::from_components(config, &body, &eye.as_ref(), &brain)
    }
}
