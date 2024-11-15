#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]
use std::ops::Index;

use bevy_reflect::Reflect;
use rand::{prelude::SliceRandom, Rng, RngCore};

pub trait Individual {
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
    fn create(chromosome: Chromosome) -> Self;
}

pub trait SelectionMethod {
    // Select one individual among the provided population with its fitness higher than the threshold
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
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

#[derive(Clone, Default, Reflect, Debug)]
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
    pub fn extend(&mut self, iter: impl IntoIterator<Item = f32>) {
        self.genes.extend(iter);
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
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual,
    {
        population
            .choose_weighted(rng, |individual| individual.fitness())
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
        fertility_rate: f32,
        minimum_population: usize,
    ) -> Vec<I>
    where
        I: Individual,
    {
        if population.is_empty() {
            return vec![];
        }
        let n_children = population.len() as f32 * fertility_rate;
        let n_children = n_children as usize
            + if rng.gen_bool((n_children % 1.0) as f64) {
                1
            } else {
                0
            };
        let n_children = n_children.max(minimum_population);
        let new_chromosomes = (0..n_children)
            .map(|_| {
                let parent_a = self.selection_method.select(rng, population);
                let parent_b = self.selection_method.select(rng, population);
                self.crossover_method
                    .crossover(rng, parent_a.chromosome(), parent_b.chromosome())
            })
            .collect::<Vec<_>>();
        // Apply mutation and create new individuals
        new_chromosomes
            .into_iter()
            .map(|mut c| {
                self.mutation_method.mutate(rng, &mut c);
                I::create(c)
            })
            .collect::<Vec<_>>()
    }
}
