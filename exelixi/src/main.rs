#![allow(clippy::type_complexity)]
mod animal_individual;
mod brain;
mod components;
mod eye;
mod simulation;
mod spawner;
mod systems;

mod prelude {
    pub use std::f32::consts::{FRAC_PI_2, PI};

    pub use bevy::diagnostic::Diagnostics;
    pub use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
    pub use bevy::prelude::*;
    pub use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};

    pub use rand::{thread_rng, Rng, RngCore};

    pub use lib_genetic_algorithm as ga;
    pub use lib_neural_network as nn;

    pub use crate::animal_individual::*;
    pub use crate::brain::*;
    pub use crate::components::*;
    pub use crate::eye::*;
    pub use crate::simulation::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;

    // Minimum linear velocity in pixels/step
    pub const V_LINEAR_MIN: f32 = 1.0;
    // Maximum linear velocity in pixels/step
    pub const V_LINEAR_MAX: f32 = 5.0;
    // Linear acceleration pixels/step^2
    pub const V_LINEAR_ACCEL: f32 = 40.0;
    /// Maximum angalur velocity in radians/step
    pub const V_ANGULAR_MAX: f32 = PI / 30.0;
}

use prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            width: 1280.0,
            height: 800.0,
            title: "Exelixi".to_string(),
            vsync: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(EguiPlugin)
        .add_startup_system(setup)
        .add_startup_system(spawn_animals)
        .add_startup_system(spawn_foods)
        .add_startup_system(spawn_floor)
        .add_system(debug_ui)
        .add_system(movement)
        .add_system(collision)
        .add_system(process_brain)
        .add_system(evolve)
        .add_system(simulation_config_update)
        .insert_resource(Simulation::default())
        .insert_resource(SimulationConfig::default())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
