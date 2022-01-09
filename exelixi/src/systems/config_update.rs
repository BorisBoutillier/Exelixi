use crate::prelude::*;

pub fn simulation_config_update(
    config: Res<SimulationConfig>,
    mut floor: Query<&mut Sprite, With<Floor>>,
    mut visibilities: Query<&mut Visibility, Or<(With<Food>, With<Animal>)>>,
) {
    if config.is_changed() {
        let mut floor_sprite = floor.get_single_mut().unwrap();
        floor_sprite.custom_size = Some(Vec2::new(
            config.environment_size.width,
            config.environment_size.height,
        ));
        for mut visibility in visibilities.iter_mut() {
            visibility.is_visible = true;
        }
    }
}