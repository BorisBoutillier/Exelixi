use crate::prelude::*;

pub fn spawn_food(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<SimulationConfig>,
    mut rng: ResMut<MyRng>,
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
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    color: Color::rgb(0.1, 0.7, 0.1),
                    ..Default::default()
                },
                texture: asset_server.load("food.png"),
                ..Default::default()
            })
            .insert(Food::new(&config))
            .insert(Position::new(x, y, 0.0))
            .insert(Decay {
                time: config.environment.food_decay_time as i32,
            });
    }
}
