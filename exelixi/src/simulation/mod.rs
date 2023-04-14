use std::time::Instant;

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
        app.add_system(run_ecosystem_schedule);
        if self.run_for.is_none() {
            app.add_plugin(InputManagerPlugin::<SimulationSpeedAction>::default());
            app.add_startup_system(setup_simulation_speed_action);
            app.add_system(simulation_speed_action_input.in_base_set(CoreSet::PostUpdate));
        }
    }
}

const MIN_FPS: u32 = 10;
// Maximum duration the simulation steps car run per frame
const MAX_SIMULATION_DURATION_PER_FRAME: f32 = 1.0 / (MIN_FPS as f32);

pub fn run_ecosystem_schedule(world: &mut World) {
    let (start_steps, control, run_for) = {
        let simulation = world.get_resource::<Simulation>().unwrap();
        (simulation.steps, simulation.control, simulation.run_for)
    };
    if let Some(n_steps) = run_for {
        loop {
            world.run_schedule(EcosystemSchedule);
            let cur_steps = world.get_resource::<Simulation>().unwrap().steps;
            // Always give back control on generation increase
            if cur_steps >= start_steps + n_steps {
                world
                    .get_resource_mut::<Events<AppExit>>()
                    .unwrap()
                    .send_default();
                break;
            }
        }
    } else {
        if control.state == SimulationControlState::Paused {
            return;
        }
        let start_time = Instant::now();
        let mut cur_steps = 0;
        loop {
            world.run_schedule(EcosystemSchedule);
            cur_steps += 1;
            if let Some(mut events) = world.get_resource_mut::<Events<NewGenerationEvent>>() {
                if !events.is_empty() {
                    events.update();
                    break;
                }
            }

            // Give back control every 1/60s
            if (Instant::now() - start_time).as_secs_f32() >= MAX_SIMULATION_DURATION_PER_FRAME {
                break;
            }

            if control.state == SimulationControlState::Run && cur_steps >= control.speed_factor {
                break;
            }
        }
    }
}
