use crate::ecosystem::*;
pub use lib_genetic_algorithm as ga;
pub use lib_neural_network as nn;

#[derive(Component)]
pub struct Brain {
    pub nn: nn::Network,
}

impl Brain {
    pub fn random(
        rng: &mut dyn RngCore,
        body: &Body,
        eye: &Option<&Eye>,
        locomotion: &Option<&Locomotion>,
    ) -> Self {
        Self {
            nn: nn::Network::random(rng, &Self::topology(body, eye, locomotion)),
        }
    }

    pub fn from_genes(
        genes: impl IntoIterator<Item = f32>,
        body: &Body,
        eye: &Option<&Eye>,
        locomotion: &Option<&Locomotion>,
    ) -> Self {
        Self {
            nn: nn::Network::from_weights(&Self::topology(body, eye, locomotion), genes),
        }
    }
    pub fn as_chromosome(&self) -> ga::Chromosome {
        self.nn.weights().collect()
    }

    fn topology(
        body: &Body,
        eye: &Option<&Eye>,
        locomotion: &Option<&Locomotion>,
    ) -> [nn::LayerTopology; 3] {
        let mut n_sensors = body.n_sensors();
        if let Some(eye) = eye {
            n_sensors += eye.n_sensors();
        }
        let mut n_actuators = 0;
        if let Some(locomotion) = locomotion {
            n_actuators += locomotion.n_actuators();
        }
        [
            nn::LayerTopology { neurons: n_sensors },
            nn::LayerTopology {
                neurons: 2 * n_sensors,
            },
            nn::LayerTopology {
                neurons: n_actuators,
            },
        ]
    }
    pub fn energy_cost(&self) -> f32 {
        0.0
    }
}

pub fn brain_processing(
    mut organisms: Query<(&Body, &Position, &mut Locomotion, &Eye, &Brain)>,
    positions: Query<(&Position, &Organism)>,
    config: Res<EcosystemConfig>,
) {
    let positions = positions.iter().collect::<Vec<_>>();
    organisms.for_each_mut(|(body, position, mut locomotion, eye, brain)| {
        let mut inputs = eye.process_vision(position, &positions, &config);
        inputs.extend(body.get_sensors().iter());
        let response = brain.nn.propagate(&inputs);
        locomotion.actuates(response);
    });
}
