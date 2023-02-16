use crate::prelude::*;

pub fn spawn_food(mut commands: Commands, config: Res<SimulationConfig>, mut rng: ResMut<MyRng>) {
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
