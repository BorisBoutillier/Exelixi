#![allow(clippy::type_complexity)]
mod camera;
mod components;
mod organism;
mod simulation;
mod spawner;
mod ui;

mod prelude {
    use rand_chacha::ChaCha8Rng;

    pub use std::f32::consts::{FRAC_PI_2, PI};

    pub use bevy::diagnostic::Diagnostics;
    pub use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
    pub use bevy::ecs::schedule::ShouldRun;
    pub use bevy::log;
    pub use bevy::prelude::*;
    pub use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};

    pub use rand::{Rng, RngCore};

    pub use lib_genetic_algorithm as ga;
    pub use lib_neural_network as nn;

    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::organism::*;
    pub use crate::simulation::*;
    pub use crate::spawner::*;
    pub use crate::ui::*;

    #[derive(Resource)]
    pub struct MyRng(pub ChaCha8Rng);
}

use std::path::PathBuf;

use clap::Parser;
use prelude::*;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

// Organism evolution simulation
#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
struct Args {
    /// Path to the simulation config to use
    #[arg(short, long)]
    config: Option<PathBuf>,
    // Define the initial seed for the simulation
    // If not provided will be 'randomized'
    #[arg(short, long)]
    seed: Option<u64>,
}

fn main() {
    let args = Args::parse();
    let start_config = SimulationConfig::from_path(args.config);
    let rng = if let Some(seed) = args.seed {
        ChaCha8Rng::seed_from_u64(seed)
    } else {
        ChaCha8Rng::from_entropy()
    };
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::BLACK))
        .insert_resource(MyRng(rng))
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
        .add_system_to_stage(CoreStage::PreUpdate, spawn_starting_organisms)
        .add_system_to_stage(CoreStage::PreUpdate, spawn_floor)
        .add_startup_system(insert_simulation_steps_schedule)
        .add_system(simulation_steps)
        .add_system(exit_at_generation)
        .add_system(transform_update)
        .insert_resource(Simulation::new(&start_config))
        .insert_resource(start_config)
        .run();
}
