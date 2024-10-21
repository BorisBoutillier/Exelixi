use std::{path::PathBuf, time::Instant};

use crate::prelude::*;

mod config;
mod control;

use bevy::{app::AppExit, ecs::system::RunSystemOnce};
pub use config::*;
pub use control::*;

pub struct SimulationPlugin {
    pub load_path: Option<PathBuf>,
    pub run_for: Option<u32>,
    pub save_path: Option<PathBuf>,
    pub with_gui: bool,
}
impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Simulation {
            load: self.load_path.clone(),
            run: self.run_for,
            save: self.save_path.clone(),
            exit: !self.with_gui,
            ..Default::default()
        });
        app.add_systems(Update, run_ecosystem_schedule);
        if self.with_gui {
            app.add_plugins(InputManagerPlugin::<SimulationAction>::default());
            app.add_systems(Startup, setup_simulation_speed_action);
            app.add_systems(PostUpdate, simulation_action_input);
        }
    }
}

const MIN_FPS: u32 = 10;
// Maximum duration the simulation steps car run per frame
const MAX_SIMULATION_DURATION_PER_FRAME: f32 = 1.0 / (MIN_FPS as f32);

pub fn run_ecosystem_schedule(world: &mut World) {
    // Check for load request and apply
    if let Some(path) = world.get_resource_mut::<Simulation>().unwrap().load.take() {
        crate::ecosystem::load_ecosystem_from_file(&path, world);
    }
    // Run the simulation for the defined number of steps
    if let Some(n_steps) = world.get_resource_mut::<Simulation>().unwrap().run.take() {
        let start_steps = world.get_resource::<EcosystemRuntime>().unwrap().steps;
        loop {
            world.run_schedule(EcosystemSchedule);
            let cur_steps = world.get_resource::<EcosystemRuntime>().unwrap().steps;
            // Always give back control on generation increase
            if cur_steps >= start_steps + n_steps {
                break;
            }
        }
    } else {
        let control = world.get_resource::<Simulation>().unwrap().control;
        if control.state != SimulationControlState::Paused {
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
                if (Instant::now() - start_time).as_secs_f32() >= MAX_SIMULATION_DURATION_PER_FRAME
                {
                    break;
                }

                if control.state == SimulationControlState::Run && cur_steps >= control.speed_factor
                {
                    break;
                }
            }
        }
    }
    // Check for save request and apply
    if let Some(save_path) = world.resource_mut::<Simulation>().save.take() {
        world.run_system_once(accumulate_statistics);
        crate::ecosystem::save_ecosystem_to_file(&save_path, world);
    }
    // Check for exit request and apply
    if world.resource::<Simulation>().exit {
        world.send_event(AppExit::Success);
    }
}
