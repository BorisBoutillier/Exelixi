use crate::*;

#[derive(Component)]
pub struct Floor {}

pub fn show_floor(
    mut commands: Commands,
    config: Res<EcosystemConfig>,
    mut floor_sprite: Query<&mut Sprite, With<Floor>>,
) {
    if config.is_added() {
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(
                        config.environment.width as f32 + 20.0,
                        config.environment.height as f32 + 20.0,
                    )),
                    color: Color::rgb_u8(0x4B, 0x37, 0x1C),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Floor {});
    } else if config.is_changed() {
        let mut sprite = floor_sprite.get_single_mut().unwrap();
        sprite.custom_size = Some(Vec2::new(
            config.environment.width as f32 + 20.0,
            config.environment.height as f32 + 20.0,
        ));
    }
}
