#![feature(type_alias_impl_trait)]
use std::ops::Index;

use rand::{prelude::SliceRandom, Rng, RngCore};

pub trait Individual {
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
    fn create(chromosome: Chromosome) -> Self;
}

pub trait SelectionMethod {
    // Select one individual among the provided population with its fitness higher than the threshold
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I], threshold: f32) -> &'a I
    where
        I: Individual;
}

pub trait CrossoverMethod: Send + Sync {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome;
}
pub trait MutationMethod: Send + Sync {
    fn mutate(&self, rng: &mut dyn RngCore, chromosome: &mut Chromosome);
}

#[derive(Clone)]
pub struct Chromosome {
    genes: Vec<f32>,
}
impl Chromosome {
    pub fn len(&self) -> usize {
        self.genes.len()
    }
    pub fn is_empty(&self) -> bool {
        self.genes.is_empty()
    }
    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.genes.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.genes.iter_mut()
    }
}
impl Index<usize> for Chromosome {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.genes[index]
    }
}
impl FromIterator<f32> for Chromosome {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        Self {
            genes: iter.into_iter().collect(),
        }
    }
}
impl IntoIterator for Chromosome {
    type Item = f32;
    type IntoIter = impl Iterator<Item = f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
    }
}
#[derive(Clone, Debug)]
pub struct RouletteWheelSelection;

impl RouletteWheelSelection {
    pub fn new() -> Self {
        Self
    }
}
impl Default for RouletteWheelSelection {
    fn default() -> Self {
        RouletteWheelSelection::new()
    }
}
impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I], threshold: f32) -> &'a I
    where
        I: Individual,
    {
        population
            .choose_weighted(rng, |individual| {
                let f = individual.fitness();
                if f > threshold {
                    f
                } else {
                    0.0
                }
            })
            .expect("Got an empty population to choose from")
    }
}

pub struct UniformCrossover;
impl UniformCrossover {
    pub fn new() -> Self {
        Self
    }
}
impl CrossoverMethod for UniformCrossover {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome {
        assert_eq!(parent_a.len(), parent_b.len());
        parent_a
            .iter()
            .zip(parent_b.iter())
            .map(|(a, b)| if rng.gen_bool(0.5) { *a } else { *b })
            .collect()
    }
}
impl Default for UniformCrossover {
    fn default() -> Self {
        UniformCrossover::new()
    }
}
pub struct GaussianMutation {
    /// Probability of changing a gene:
    /// - 0.0 = no genes will be touched
    /// - 1.0 = all genes will be touched
    chance: f32,

    /// Magnitude of that change:
    /// - 0.0 = touched genes will not be modified
    /// - 3.0 = touched genes will be += or -= by at most 3.0
    coeff: f32,
}
impl GaussianMutation {
    pub fn new(chance: f32, coeff: f32) -> Self {
        assert!((0.0..=1.0).contains(&chance));
        assert!(coeff >= 0.0);
        Self { chance, coeff }
    }
}
impl MutationMethod for GaussianMutation {
    fn mutate(&self, rng: &mut dyn RngCore, chromosome: &mut Chromosome) {
        for gene in chromosome.iter_mut() {
            if rng.gen_bool(self.chance as f64) {
                let sign = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
                *gene += sign * self.coeff * rng.gen::<f32>();
            }
        }
    }
}

pub struct GeneticAlgorithm<S> {
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    mutation_method: Box<dyn MutationMethod>,
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod,
{
    pub fn new(
        selection_method: S,
        crossover_method: impl CrossoverMethod + 'static,
        mutation_method: impl MutationMethod + 'static,
    ) -> Self {
        Self {
            selection_method,
            crossover_method: Box::new(crossover_method),
            mutation_method: Box::new(mutation_method),
        }
    }
    pub fn evolve<I>(
        &self,
        rng: &mut dyn RngCore,
        population: &[I],
        death_threshold: f32,
        fertility_rate: f32,
    ) -> (Vec<I>, PopulationStatistics)
    where
        I: Individual,
    {
        let n_survivors = population
            .iter()
            .filter(|i| i.fitness() > death_threshold)
            .count();
        let n_children = n_survivors as f32 * fertility_rate;
        let n_children = n_children as usize
            + if rng.gen_bool((n_children % 1.0) as f64) {
                1
            } else {
                0
            };
        let new_chromosomes = (0..n_children)
            .map(|_| {
                let parent_a = self
                    .selection_method
                    .select(rng, population, death_threshold);
                let parent_b = self
                    .selection_method
                    .select(rng, population, death_threshold);
                self.crossover_method
                    .crossover(rng, parent_a.chromosome(), parent_b.chromosome())
            })
            .collect::<Vec<_>>();
        // Apply mutation and create new individuals
        let new_population = new_chromosomes
            .into_iter()
            .map(|mut c| {
                self.mutation_method.mutate(rng, &mut c);
                I::create(c)
            })
            .collect::<Vec<_>>();

        let stats = PopulationStatistics::new(population, death_threshold);
        (new_population, stats)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[derive(Clone, Debug, PartialEq)]
    pub struct TestIndividual {
        fitness: f32,
    }

    impl TestIndividual {
        pub fn new(fitness: f32) -> Self {
            Self { fitness }
        }
    }

    impl Individual for TestIndividual {
        fn fitness(&self) -> f32 {
            self.fitness
        }
        fn chromosome(&self) -> &Chromosome {
            panic!()
        }
        fn create(_chromosome: Chromosome) -> Self {
            panic!()
        }
    }

    #[test]
    fn test() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let population = vec![
            TestIndividual::new(2.0),
            TestIndividual::new(1.0),
            TestIndividual::new(4.0),
            TestIndividual::new(3.0),
        ];
        let mut actual_histogram = BTreeMap::new();

        for _ in 0..1000 {
            let fitness = RouletteWheelSelection::new()
                .select(&mut rng, &population, 0.0)
                .fitness() as i32;
            *actual_histogram.entry(fitness).or_insert(0) += 1;
        }
        let expected_histogram = BTreeMap::from_iter(vec![(1, 98), (2, 202), (3, 278), (4, 422)]);
        assert_eq!(actual_histogram, expected_histogram);
    }
}

pub struct PopulationStatistics {
    min_fitness: f32,
    max_fitness: f32,
    avg_fitness: f32,
    size: usize,
    dead: usize,
}

impl PopulationStatistics {
    fn new<I>(population: &[I], death_threshold: f32) -> Self
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        let mut min_fitness = population[0].fitness();
        let mut max_fitness = min_fitness;
        let mut sum_fitness = 0.0;

        for individual in population {
            let fitness = individual.fitness();

            min_fitness = min_fitness.min(fitness);
            max_fitness = max_fitness.max(fitness);
            sum_fitness += fitness;
        }

        let size = population.len();
        let dead = population
            .iter()
            .filter(|i| i.fitness() < death_threshold)
            .count();
        Self {
            min_fitness,
            max_fitness,
            avg_fitness: sum_fitness / (population.len() as f32),
            size,
            dead,
        }
    }

    pub fn min_fitness(&self) -> f32 {
        self.min_fitness
    }

    pub fn max_fitness(&self) -> f32 {
        self.max_fitness
    }

    pub fn avg_fitness(&self) -> f32 {
        self.avg_fitness
    }
    pub fn size(&self) -> usize {
        self.size
    }
    pub fn dead(&self) -> usize {
        self.dead
    }
}
impl Default for PopulationStatistics {
    fn default() -> Self {
        Self {
            min_fitness: 0.0,
            max_fitness: 0.0,
            avg_fitness: 0.0,
            size: 0,
            dead: 0,
        }
    }
}
