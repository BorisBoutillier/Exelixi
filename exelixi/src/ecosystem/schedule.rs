use bevy::ecs::schedule::ScheduleLabel;

use crate::prelude::*;
use std::time::Instant;

const FPS: u32 = 60;
// Maximum duration the simulation steps car run per frame
const MAX_SIMULATION_DURATION_PER_FRAME: f32 = 1.0 / (FPS as f32);
#[derive(Resource)]
struct MySchedule(Schedule);

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct CoreSimulationSchedule;
impl CoreSimulationSchedule {
    pub fn create() -> Schedule {
        let mut schedule = Schedule::new();
        schedule.add_systems(
            (
                locomotion_movement,
                mouth_eating,
                apply_system_buffers,
                brain_processing,
                body_energy_consumption,
                food_decay,
                apply_system_buffers,
                evolve,
                food_spawning,
                apply_system_buffers,
                dump_debug_info,
            )
                .chain(),
        );
        schedule
    }
    pub fn run(world: &mut World) {
        let (start_generation, control) = {
            let simulation = world.get_resource::<Simulation>().unwrap();
            (simulation.generation, simulation.control)
        };
        if control.state == SimulationControlState::Paused {
            return;
        }
        let start_time = Instant::now();
        let mut cur_steps = 0;
        loop {
            world.run_schedule(CoreSimulationSchedule);
            cur_steps += 1;
            let cur_generation = world.get_resource::<Simulation>().unwrap().generation;
            // Always give back control on generation increase
            if cur_generation > start_generation {
                break;
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
