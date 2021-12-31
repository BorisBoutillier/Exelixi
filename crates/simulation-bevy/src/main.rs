mod components;
mod eye;
mod spawner;
mod systems;

mod prelude {
    pub use std::f32::consts::{FRAC_PI_2, PI};

    pub use bevy::prelude::*;
    pub use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};
    pub use rand::{Rng, RngCore};

    pub use genetic_algorithm as ga;
    pub use neural_network as nn;

    pub use crate::components::*;
    pub use crate::eye::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;

    // Minimum linear velocity in pixels/s
    pub const V_LINEAR_MIN: f32 = 5.0;
    // Maximum linear velocity in pixels/s
    pub const V_LINEAR_MAX: f32 = 200.0;
    /// Maximum angalur velocity in radians/s
    pub const V_ANGULAR_MAX: f32 = PI;
    pub const N_ANIMAL: usize = 40;
    pub const N_FOOD: usize = 60;
}

use prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.6)))
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_startup_system(setup)
        .add_startup_system(spawn_animals)
        .add_startup_system(spawn_foods)
        .add_system(debug_ui)
        .add_system(movement)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
