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
    pub use bevy::ecs::schedule::ShouldRun;
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

use std::time::Duration;

use prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            width: 1500.0,
            height: 900.0,
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
        .add_system(status_bar_ui)
        .add_system(simulation_config_update)
        .insert_resource(Simulation::default())
        .insert_resource(SimulationConfig::default())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(run_simulation_speed)
                .with_system(movement)
                .with_system(collision)
                .with_system(process_brain)
                .with_system(evolve),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
fn run_simulation_speed(time: Res<Time>, mut simulation: ResMut<Simulation>) -> ShouldRun {
    simulation.step_duration += time.delta();
    let run = match simulation.speed {
        SimulationSpeed::Paused => ShouldRun::No,
        SimulationSpeed::Normal => {
            if simulation.step_duration.as_secs_f32() >= STEP_LENGTH_NORMAL {
                ShouldRun::Yes
            } else {
                ShouldRun::No
            }
        }
        SimulationSpeed::Fast => {
            if simulation.step_duration.as_secs_f32() >= STEP_LENGTH_FAST {
                ShouldRun::Yes
            } else {
                ShouldRun::No
            }
        }
        SimulationSpeed::Fastest => ShouldRun::Yes,
    };
    if run == ShouldRun::Yes {
        simulation.step_duration = Duration::ZERO;
    }
    run
}
