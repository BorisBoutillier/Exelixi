use crate::ecosystem::*;

pub fn food_spawning(
    mut commands: Commands,
    config: Res<EcosystemConfig>,
    mut rng: ResMut<EcosystemRng>,
) {
    let n_food_to_spawn = config.environment.food_spawn_rate as u32
        + if rng.0.gen_bool(config.environment.food_spawn_rate % 1.0) {
            1
        } else {
            0
        };

    for _ in 0..n_food_to_spawn {
        let half_width = config.environment.width / 2;
        let half_height = config.environment.height / 2;
        let x = rng.0.gen_range(-half_width..half_width);
        let y = rng.0.gen_range(-half_height..half_height);
        // The food energy will linearly increase from min to max in first 2/3 of its lifetime
        //                 then linearly decreate from max to 0 in last 1/3 of its lifetime.
        let start_energy = config.environment.food_energy;
        let lifetime = config.environment.food_decay_time as i32;
        let max_energy = start_energy * 2;
        let total_energy = (max_energy - start_energy) * (lifetime * 2 / 3) / 2;
        let body_cost = total_energy / lifetime;
        commands.spawn((
            Organism {
                kind: OrganismKind::Plant,
            },
            Position::new(x as f32, y as f32, 0.0),
            Body::new(start_energy, max_energy, body_cost),
            Leaf {
                lifetime: lifetime as u32 * 2 / 3,
                energy_production: (body_cost + (max_energy - start_energy) / (lifetime * 2 / 3))
                    as f32,
            },
        ));
    }
}
