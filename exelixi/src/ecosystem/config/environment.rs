use std::f32::consts::PI;

use crate::ecosystem::*;

#[derive(Reflect, Serialize, Deserialize, Debug, Clone)]
pub struct EnvironmentConfig {
    // Width of the floor
    pub width: i32,
    // Height of the floor
    pub height: i32,
    // Presence of wall on the boundary.
    // Without walls the world is a torus
    pub wall: bool,
}

impl EnvironmentConfig {
    pub fn get_random_position(&self, rng: &mut dyn RngCore) -> Position {
        let half_width = self.width / 2;
        let half_height = self.height / 2;
        Position::new(
            rng.gen_range(-half_width..half_width) as f32,
            rng.gen_range(-half_height..half_height) as f32,
            rng.gen_range(-PI..PI),
        )
    }
}
