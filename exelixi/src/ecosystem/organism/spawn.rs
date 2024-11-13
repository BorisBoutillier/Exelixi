use std::f32::consts::PI;

use lib_genetic_algorithm::{Chromosome, Individual};

use crate::ecosystem::*;

#[derive(Event)]
pub struct SpawnOrganismEvent {
    // Species for the new organism
    pub species: SpeciesId,
    // Position for the new organism, when None, position is randomized.
    pub position: Option<Position>,
    // Body energy of the spawned organism, if None, spawned with the config stating_energy,
    pub energy: Option<f32>,
    // Chromosome for the new organism, when None, this organism species does not have brain.
    pub chromosome: Option<Chromosome>,
}

pub fn spawn_organism(
    mut commands: Commands,
    mut events: EventReader<SpawnOrganismEvent>,
    ecosystem_config: Res<EcosystemConfig>,
    mut ecosystem: ResMut<EcosystemRuntime>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    let half_width = ecosystem_config.environment.width / 2;
    let half_height = ecosystem_config.environment.height / 2;
    for event in events.read() {
        let position = event.position.unwrap_or_else(|| {
            Position::new(
                rng.gen_range(-half_width..half_width) as f32,
                rng.gen_range(-half_height..half_height) as f32,
                rng.gen_range(-PI..PI),
            )
        });
        let config = ecosystem_config
            .species
            .get(&event.species)
            .expect("Misconfigured species");
        let mut command = commands.spawn((Organism::new(config), position));
        let with_brain = config.eye.is_some();
        if with_brain {
            let individual = if let Some(chromosome) = &event.chromosome {
                OrganismIndividual::create(chromosome.clone())
            } else {
                OrganismIndividual::random(&mut *rng, config)
            };
            let (mut body, eye, locomotion, brain) = individual.into_components(config);
            if let Some(energy) = &event.energy {
                body.set_energy(*energy);
            }
            command.insert((body, brain));
            if let Some(locomotion) = locomotion {
                command.insert(locomotion);
            }
            if let Some(eye) = eye {
                command.insert(eye);
            }
        } else {
            let mut body = Body::new(&config.body);
            if let Some(energy) = &event.energy {
                body.set_energy(*energy);
            }
            command.insert(body);
        }
        if let Some(leaf_config) = &config.leaf {
            command.insert(Leaf::new(leaf_config));
        }
        if let Some(mouth_config) = &config.mouth {
            command.insert(Mouth::new(mouth_config));
        }
        if let Some(uterus_config) = &config.uterus {
            command.insert(Uterus::new(uterus_config));
        }
        ecosystem.increase_population(&config.id);
    }
}
