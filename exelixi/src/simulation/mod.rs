use crate::prelude::*;

mod config;
mod control;

use bevy::app::AppExit;
pub use config::*;
pub use control::*;

pub struct SimulationPlugin {
    pub run_for: Option<u32>,
}
impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Simulation::new(self.run_for));
        app.add_system(run_for);
        if self.run_for.is_none() {
            app.add_plugin(InputManagerPlugin::<SimulationSpeedAction>::default());
            app.add_startup_system(setup_simulation_speed_action);
            app.add_system(simulation_speed_action_input);
        }
    }
}

fn run_for(simulation: Res<Simulation>, mut app_exit_event: EventWriter<AppExit>) {
    if simulation.run_for.map(|g| simulation.generation >= g) == Some(true) {
        app_exit_event.send_default();
    }
}
