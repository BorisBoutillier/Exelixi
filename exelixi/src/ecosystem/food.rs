use crate::prelude::*;

#[derive(Component)]
pub struct Food {
    pub eaten: bool,
    pub energy: i32,
}
impl Food {
    pub fn new(config: &EcosystemConfig) -> Self {
        Self {
            eaten: false,
            energy: config.environment.food_energy,
        }
    }
}

#[derive(Component)]
pub struct Decay {
    // Number of steps after which this entity will be despawned
    pub time: i32,
}

pub fn food_decay(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Decay), With<Food>>,
    mut simulation: ResMut<Simulation>,
) {
    let mut food_decay = 0;
    for (entity, mut decay) in query.iter_mut() {
        decay.time -= 1;
        if decay.time <= 0 {
            commands.entity(entity).despawn();
            food_decay += 1;
        }
    }
    simulation.statistics.add_food_decay(food_decay);
}

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
        commands.spawn((
            Food::new(&config),
            Position::new(x as f32, y as f32, 0.0),
            Decay {
                time: config.environment.food_decay_time as i32,
            },
        ));
    }
}
