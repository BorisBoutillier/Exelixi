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

pub fn brain_processing(
    mut organisms: Query<(Entity, &Body, &Position, &mut Locomotion, &Eye, &Brain)>,
    food_positions: Query<&Position, With<Food>>,
    organism_positions: Query<(Entity, &Position), With<Organism>>,
    config: Res<EcosystemConfig>,
) {
    let food_positions = food_positions.iter().collect::<Vec<_>>();
    organisms.for_each_mut(|(entity, body, position, mut locomotion, eye, brain)| {
        let organism_positions = organism_positions
            .iter()
            .filter_map(|(e, t)| if e != entity { Some(t) } else { None })
            .collect::<Vec<_>>();
        let mut inputs =
            eye.process_vision(position, &food_positions, &organism_positions, &config);
        inputs.extend(body.get_sensors().iter());
        let response = brain.nn.propagate(&inputs);
        locomotion.actuates(response);
    });
}
