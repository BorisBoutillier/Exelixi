use crate::*;

pub fn spawn_floor(mut commands: Commands, config: Res<SimulationConfig>) {
    if config.is_changed() {
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(
                        config.environment.width + 20.0,
                        config.environment.height + 20.0,
                    )),
                    color: Color::rgb(0.1, 0.3, 0.1),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Floor {});
    }
}
