use crate::ecosystem::*;

pub fn auto_spawning(
    mut commands: Commands,
    ecosystem_config: Res<EcosystemConfig>,
    mut ecosystem: ResMut<EcosystemRuntime>,
    kdtree: Res<OrganismKdTree>,
) {
    let half_width = ecosystem_config.environment.width / 2;
    let half_height = ecosystem_config.environment.height / 2;
    for (species, config) in ecosystem_config.species.iter() {
        if let ReproductionConfig::AutoSpawn {
            spawn_rate,
            minimum_distance_from_eater,
        } = config.reproduction
        {
            let n_to_spawn = spawn_rate as u32
                + if ecosystem.rng.gen_bool(spawn_rate % 1.0) {
                    1
                } else {
                    0
                };

            let mut can_eat_me = vec![];
            for (other_species, other_config) in ecosystem_config.species.iter() {
                if species != other_species
                    && other_config
                        .mouth
                        .as_ref()
                        .map(|m| m.edible_species.contains(species))
                        .unwrap_or(false)
                {
                    can_eat_me.push(other_species);
                }
            }
            for _ in 0..n_to_spawn {
                let mut pos = None;
                // do 100 tentatives to find a valid spawn position.
                // If we can't find one, don't spawn.
                for _ in 0..100 {
                    let rng_pos = Position::new(
                        ecosystem.rng.gen_range(-half_width..half_width) as f32,
                        ecosystem.rng.gen_range(-half_height..half_height) as f32,
                        0.0,
                    );
                    for species_kdtree in can_eat_me
                        .iter()
                        .map(|other_species| kdtree.per_species.get(other_species).unwrap())
                    {
                        if species_kdtree
                            .within_radius(
                                &KdTreeEntry::new(&rng_pos, Entity::PLACEHOLDER),
                                minimum_distance_from_eater,
                            )
                            .is_empty()
                        {
                            pos = Some(rng_pos);
                            break;
                        }
                    }
                }
                if let Some(pos) = pos {
                    let mut command =
                        commands.spawn((Organism::new(config), pos, Body::new(&config.body)));
                    if let Some(leaf_config) = &config.leaf {
                        command.insert(Leaf::new(leaf_config));
                    }
                }
            }
        }
    }
}
