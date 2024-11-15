#![allow(clippy::type_complexity)]
mod ecosystem;
mod simulation;
mod visualization;

mod prelude {

    pub use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
    pub use bevy::prelude::*;
    pub use bevy_egui::{egui, EguiContexts, EguiPlugin};
    pub use leafwing_input_manager::prelude::*;

    pub use rand::Rng;

    pub use crate::ecosystem::*;
    pub use crate::simulation::*;
    pub use crate::visualization::*;
}

use std::path::PathBuf;

use clap::Parser;
use log::LogPlugin;
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
    /// When set, no GUI is open and program exist after doing the possible load/run_for/save commands.
    #[arg(long)]
    exit: bool,
    /// When defined override configuration statistics_aggregation_start value
    #[arg(long)]
    stats_start: Option<u32>,
    /// When defined override configuration statistics_aggregation_rate value
    #[arg(long)]
    stats_rate: Option<u32>,
}

fn main() {
    let args = Args::parse();
    // Handle command line argument overrides
    let mut app = App::new();
    if !args.exit {
        app.add_plugins(visualization::VisualizationPlugin);
    } else {
        app.add_plugins((MinimalPlugins, LogPlugin::default()));
    }
    app.add_plugins(ecosystem::EcosystemPlugin {
        seed: args.seed,
        config_path: args.config,
        override_stats_start: args.stats_start,
        override_stats_rate: args.stats_rate,
    });
    app.add_plugins(simulation::SimulationPlugin {
        load_path: args.load,
        run_for: args.run_for,
        save_path: args.save,
        exit: args.exit,
    });
    app.run();
}
