use rand::Rng;
pub struct LayerTopology {
    pub neurons: usize,
}
pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn random(rng: &mut dyn rand::RngCore, layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);
        let built_layers = layers
            .windows(2)
            .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
            .collect();
        Self {
            layers: built_layers,
        }
    }
    pub fn propagate(&self, inputs: &[f32]) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs.to_vec(), |inputs, layer| layer.propagate(inputs))
    }
}

struct Layer {
    neurons: Vec<Neuron>,
}
impl Layer {
    pub fn random(
        rng: &mut dyn rand::RngCore,
        input_neurons: usize,
        output_neurons: usize,
    ) -> Self {
        Self {
            neurons: (0..output_neurons)
                .map(|_| Neuron::random(rng, input_neurons))
                .collect(),
        }
    }
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }
}

struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}
impl Neuron {
    pub fn random(rng: &mut dyn rand::RngCore, n_inputs: usize) -> Self {
        Self {
            bias: rng.gen_range(-1.0..1.0),
            weights: (0..n_inputs).map(|_| rng.gen_range(-1.0..1.0)).collect(),
        }
    }
    pub fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());
        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(i, w)| i * w)
            .sum::<f32>();
        (output + self.bias).max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test() {
        // Because we always use the same seed, our `rng` in here will
        // always return the same set of values
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let neuron = Neuron::random(&mut rng, 4);

        assert_eq!(neuron.bias, -0.6255188);
        assert_eq!(
            neuron.weights,
            &[0.67383933, 0.81812596, 0.26284885, 0.5238805]
        );
    }
}
