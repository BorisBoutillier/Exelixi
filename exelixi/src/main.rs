#![allow(clippy::type_complexity)]
mod animal_individual;
mod brain;
mod camera;
mod components;
mod eye;
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

    pub use crate::animal_individual::*;
    pub use crate::brain::*;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::eye::*;
    pub use crate::simulation::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::ui::*;

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
        .add_plugin(CameraPlugin {})
        .add_plugin(UiPlugin {})
        .add_system(spawn_starting_animals)
        .add_system(spawn_floor)
        .add_system(simulation_duration)
        .add_system(save_default_config)
        .insert_resource(Simulation::default())
        .insert_resource(SimulationConfig::get_default_config())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(run_simulation_speed)
                .with_system(movement)
                .with_system(collision)
                .with_system(process_brain)
                .with_system(evolve)
                .with_system(spawn_food),
        )
        .run();
}

fn run_simulation_speed(time: Res<Time>, mut simulation: ResMut<Simulation>) -> ShouldRun {
    if simulation.speed == SimulationSpeed::Paused {
        return ShouldRun::No;
    }
    simulation.cur_steps_duration += time.delta();
    let do_one_step = match simulation.speed {
        SimulationSpeed::Paused => false,
        SimulationSpeed::Normal => simulation.cur_steps < STEP_PER_FRAME_NORMAL,
        SimulationSpeed::Fast => simulation.cur_steps < STEP_PER_FRAME_FAST,
        SimulationSpeed::Fastest => true,
    };
    if do_one_step {
        simulation.cur_steps += 1;
        if simulation.cur_steps_duration.as_secs_f32() >= MAX_SIMULATION_DURATION_PER_FRAME {
            simulation.cur_steps = 0;
            simulation.cur_steps_duration = Duration::ZERO;
            ShouldRun::Yes
        } else {
            ShouldRun::YesAndCheckAgain
        }
    } else {
        if simulation.cur_steps_duration.as_secs_f32() >= MAX_SIMULATION_DURATION_PER_FRAME {
            simulation.cur_steps = 0;
            simulation.cur_steps_duration = Duration::ZERO;
        }
        ShouldRun::No
    }
}
