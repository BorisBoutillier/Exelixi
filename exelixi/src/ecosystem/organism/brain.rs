use crate::prelude::*;

#[derive(Component)]
pub struct Brain {
    pub nn: nn::Network,
}

impl Brain {
    pub fn random(rng: &mut dyn RngCore, body: &Body, eye: &Eye, locomotion: &Locomotion) -> Self {
        Self {
            nn: nn::Network::random(rng, &Self::topology(body, eye, locomotion)),
        }
    }

    pub fn from_genes(
        genes: impl IntoIterator<Item = f32>,
        body: &Body,
        eye: &Eye,
        locomotion: &Locomotion,
    ) -> Self {
        Self {
            nn: nn::Network::from_weights(&Self::topology(body, eye, locomotion), genes),
        }
    }
    pub fn as_chromosome(&self) -> ga::Chromosome {
        self.nn.weights().collect()
    }

    fn topology(body: &Body, eye: &Eye, locomotion: &Locomotion) -> [nn::LayerTopology; 3] {
        let n_sensors = eye.n_sensors() + body.n_sensors();
        [
            nn::LayerTopology { neurons: n_sensors },
            nn::LayerTopology {
                neurons: 2 * n_sensors,
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
