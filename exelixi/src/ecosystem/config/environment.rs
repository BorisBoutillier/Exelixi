use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvironmentConfig {
    // Width of the floor
    pub width: i32,
    // Height of the floor
    pub height: i32,
    // Presence of wall on the boundary.
    // Without walls the world is a torus
    pub wall: bool,
}
