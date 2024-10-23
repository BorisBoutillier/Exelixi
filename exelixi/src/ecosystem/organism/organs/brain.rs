use crate::ecosystem::*;
pub use lib_genetic_algorithm as ga;
pub use lib_neural_network as nn;

use super::traits::Sensor;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
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
    brains: Query<(Entity, &Brain)>,
    sensors: Query<(&Body, &Eye)>,
    mut actuators: Query<&mut Locomotion>,
) {
    for (entity, brain) in brains.iter() {
        let nn_inputs = if let Ok((body, eye)) = sensors.get(entity) {
            [eye.sensors(), body.sensors()].concat()
        } else {
            vec![]
        };
        let mut nn_output = brain.nn.propagate(&nn_inputs).into_iter();
        if let Ok(mut locomotion) = actuators.get_mut(entity) {
            locomotion.actuates(&mut nn_output);
        }
        assert_eq!(
            nn_output.next(),
            None,
            "Not all neuron outputs have been consumed"
        );
    }
}
