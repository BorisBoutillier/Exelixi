#![allow(clippy::type_complexity)]
mod ecosystem;
mod visualization;

mod prelude {
    use rand_chacha::ChaCha8Rng;

    pub use std::f32::consts::{FRAC_PI_2, PI};

    pub use bevy::diagnostic::Diagnostics;
    pub use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
    pub use bevy::log;
    pub use bevy::prelude::*;
    pub use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiSettings};
    pub use leafwing_input_manager::prelude::*;

    pub use rand::Rng;
    pub use rand::RngCore;

    pub use lib_genetic_algorithm as ga;
    pub use lib_neural_network as nn;

    pub use crate::ecosystem::*;
    pub use crate::visualization::*;

    #[derive(Resource)]
    pub struct MyRng(pub ChaCha8Rng);
}

use std::path::PathBuf;

use bevy::window::WindowResolution;
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
    /// Initial seed for the simulation, randomized when not provided.
    #[arg(short, long)]
    seed: Option<u64>,
    /// Defines if the simulation is run without GUI
    #[arg(long = "no-gui", default_value_t = false)]
    no_gui: bool,
}

fn main() {
    let args = Args::parse();
    let mut start_config = SimulationConfig::from_path(args.config);
    // Handle command line argument overrides
    if args.no_gui {
        start_config.with_gui = false;
    }
    let rng = if let Some(seed) = args.seed {
        ChaCha8Rng::seed_from_u64(seed)
    } else {
        ChaCha8Rng::from_entropy()
    };
    let mut app = App::new();
    if start_config.with_gui {
        app.insert_resource(ClearColor(Color::BLACK));
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1500.0, 900.0),
                title: "Exelixi".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(visualization::VisualizationPlugin);
        app.add_plugin(InputManagerPlugin::<SimulationSpeedAction>::default());
        app.add_startup_system(setup_simulation_speed_action);
        app.add_system(simulation_speed_action_input);
    } else {
        app.add_plugins(MinimalPlugins);
    }
    app.insert_resource(MyRng(rng))
        .add_system(spawn_starting_organisms.in_base_set(CoreSet::PreUpdate))
        .add_system(spawn_floor.in_base_set(CoreSet::PreUpdate))
        .add_system(exit_at_generation)
        .insert_resource(Simulation::new(&start_config))
        .insert_resource(start_config);
    app.add_schedule(CoreSimulationSchedule, CoreSimulationSchedule::create())
        .add_system(CoreSimulationSchedule::run);
    app.run();
}
