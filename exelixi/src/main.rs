#![allow(clippy::type_complexity)]
mod camera;
mod components;
mod organism;
mod simulation;
mod spawner;
mod systems;
mod ui;

mod prelude {
    pub use std::f32::consts::{FRAC_PI_2, PI};

    pub use bevy::diagnostic::Diagnostics;
    pub use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
    pub use bevy::ecs::schedule::ShouldRun;
    pub use bevy::log;
    pub use bevy::prelude::*;
    pub use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};

    pub use rand::{thread_rng, Rng, RngCore};

    pub use lib_genetic_algorithm as ga;
    pub use lib_neural_network as nn;

    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::organism::*;
    pub use crate::simulation::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::ui::*;
}

use prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 1500.0,
                height: 900.0,
                title: "Exelixi".to_string(),
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(EguiPlugin)
        .add_plugin(CameraPlugin {})
        .add_plugin(UiPlugin {})
        .add_system(spawn_starting_organisms)
        .add_system(spawn_floor)
        .add_system(save_default_config)
        .add_startup_system(insert_simulation_steps_schedule)
        .add_system(simulation_steps)
        .insert_resource(Simulation::default())
        .insert_resource(SimulationConfig::get_default_config())
        .run();
}
