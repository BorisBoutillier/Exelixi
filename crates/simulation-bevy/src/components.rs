use crate::prelude::*;

#[derive(Component)]
pub struct Velocity {
    pub linear: f32,
    pub angular: f32,
}
