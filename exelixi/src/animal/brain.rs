use crate::prelude::*;

#[derive(Component)]
pub struct Brain {
    pub nn: nn::Network,
}

impl Brain {
    pub fn random(rng: &mut dyn RngCore, eye: &Eye, locomotion: &Locomotion) -> Self {
        Self {
            nn: nn::Network::random(rng, &Self::topology(eye, locomotion)),
        }
    }

    pub fn from_genes(
        genes: impl IntoIterator<Item = f32>,
        eye: &Eye,
        locomotion: &Locomotion,
    ) -> Self {
        Self {
            nn: nn::Network::from_weights(&Self::topology(eye, locomotion), genes),
        }
    }
    pub fn as_chromosome(&self) -> ga::Chromosome {
        self.nn.weights().collect()
    }

    fn topology(eye: &Eye, locomotion: &Locomotion) -> [nn::LayerTopology; 3] {
        [
            nn::LayerTopology {
                neurons: eye.n_sensors(),
            },
            nn::LayerTopology {
                neurons: 2 * eye.n_sensors(),
            },
            nn::LayerTopology {
                neurons: locomotion.n_actuators(),
            },
        ]
    }
    pub fn energy_cost(&self) -> f32 {
        0.0
    }
}
