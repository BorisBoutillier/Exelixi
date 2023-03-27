use crate::ecosystem::*;

pub fn auto_spawning(
    mut commands: Commands,
    config: Res<EcosystemConfig>,
    mut rng: ResMut<EcosystemRng>,
) {
    for organism_config in config.organisms.iter() {
        if let ReproductionConfig::AutoSpawn { spawn_rate } = organism_config.reproduction {
            let n_to_spawn = spawn_rate as u32
                + if rng.0.gen_bool(spawn_rate % 1.0) {
                    1
                } else {
                    0
                };

            for _ in 0..n_to_spawn {
                let half_width = config.environment.width / 2;
                let half_height = config.environment.height / 2;
                let x = rng.0.gen_range(-half_width..half_width);
                let y = rng.0.gen_range(-half_height..half_height);
                let mut command = commands.spawn((
                    Organism {
                        kind: OrganismKind::Plant,
                        name: organism_config.name.clone(),
                    },
                    Position::new(x as f32, y as f32, 0.0),
                    Body::new(&organism_config.body),
                ));
                if let Some(leaf_config) = &organism_config.leaf {
                    command.insert(Leaf::new(leaf_config));
                }
            }
        }
    }
}
