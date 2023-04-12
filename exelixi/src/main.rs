#![allow(clippy::type_complexity)]
mod ecosystem;
mod simulation;
mod visualization;

mod prelude {

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
    pub use crate::simulation::*;
    pub use crate::visualization::*;
}

use std::path::PathBuf;

use clap::Parser;
use prelude::*;

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
    /// Auto start the simulation without gui for the provided number of steps
    ///
    #[arg(long)]
    run_for: Option<u32>,
}

fn main() {
    let args = Args::parse();
    // Handle command line argument overrides
    let mut app = App::new();
    let with_gui = args.run_for.is_none();
    if with_gui {
        app.add_plugin(visualization::VisualizationPlugin);
    } else {
        app.add_plugins(MinimalPlugins);
    }
    app.add_plugin(ecosystem::EcosystemPlugin {
        seed: args.seed,
        config_path: args.config,
    });
    app.add_plugin(simulation::SimulationPlugin {
        run_for: args.run_for,
    });
    app.run();
}
