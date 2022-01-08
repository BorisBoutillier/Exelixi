use crate::prelude::*;

pub fn collision(
    mut animals: Query<(&mut Stomach, &Transform), Without<Food>>,
    mut foods: Query<&mut Transform, With<Food>>,
    config: Res<SimulationConfig>,
) {
    for (mut animal_stomach, animal_transform) in animals.iter_mut() {
        for mut food_transform in foods.iter_mut() {
            let distance = (animal_transform.translation - food_transform.translation).length();
            if distance <= 10.0 {
                let half_width = config.environment_size.width / 2.0;
                let half_height = config.environment_size.height / 2.0;
                let mut rng = thread_rng();
                food_transform.translation.x = rng.gen_range(-half_width..half_width);
                food_transform.translation.y = rng.gen_range(-half_height..half_height);
                animal_stomach.satiation += 1.0;
            }
        }
    }
}
