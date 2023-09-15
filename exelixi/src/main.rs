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
    #[arg(long)]
    config: Option<PathBuf>,
    /// Initial seed for the simulation, randomized when not provided.
    #[arg(long)]
    seed: Option<u64>,
    /// Auto start the simulation without gui for the provided number of steps
    ///
    #[arg(long)]
    run_for: Option<u32>,
    /// Path to load a saved simulation state from
    #[arg(long)]
    load: Option<PathBuf>,
    /// Default path to save the simulation to
    #[arg(long)]
    save: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    // Handle command line argument overrides
    let mut app = App::new();
    let with_gui = args.run_for.is_none();
    if with_gui {
        app.add_plugins(visualization::VisualizationPlugin);
    } else {
        app.add_plugins(MinimalPlugins);
    }
    app.add_plugins(ecosystem::EcosystemPlugin {
        seed: args.seed,
        config_path: args.config,
        load_path: args.load,
    });
    app.add_plugins(simulation::SimulationPlugin {
        run_for: args.run_for,
        save_path: args.save,
    });
    app.run();
}
