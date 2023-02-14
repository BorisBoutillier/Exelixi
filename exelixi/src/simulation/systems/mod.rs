mod brain;
mod collision;
mod decay;
mod energy;
mod environment;
mod evolve;
mod movement;

use bevy::app::AppExit;
pub use brain::*;
pub use collision::*;
pub use decay::*;
pub use energy::*;
pub use environment::*;
pub use evolve::*;
pub use movement::*;

use crate::prelude::*;
use std::time::Instant;

const FPS: u32 = 60;
// Maximum duration the simulation steps car run per frame
const MAX_SIMULATION_DURATION_PER_FRAME: f32 = 1.0 / (FPS as f32);

#[derive(Resource)]
struct MySchedule(Schedule);

pub fn insert_simulation_steps_schedule(mut commands: Commands) {
    let mut schedule = Schedule::default();
    schedule.add_stage("a", SystemStage::parallel());
    schedule.add_stage_after("a", "b", SystemStage::parallel());
    schedule.add_stage_after("b", "c", SystemStage::parallel());
    schedule.add_stage_after("c", "d", SystemStage::parallel());
    schedule.add_stage_after("d", "e", SystemStage::parallel());
    schedule.add_stage_after("e", "f", SystemStage::parallel());
    schedule.add_stage_after("f", "g", SystemStage::parallel());
    schedule.add_stage_after("g", "h", SystemStage::parallel());
    schedule.add_system_to_stage("a", movement);
    schedule.add_system_to_stage("b", collision);
    schedule.add_system_to_stage("c", process_brain);
    schedule.add_system_to_stage("d", energy);
    schedule.add_system_to_stage("e", decay);
    schedule.add_system_to_stage("f", evolve);
    schedule.add_system_to_stage("g", spawn_food);
    schedule.add_system_to_stage("h", dump_debug_info);
    commands.insert_resource(MySchedule(schedule));
}

pub fn simulation_steps(world: &mut World) {
    let (start_generation, control) = {
        let simulation = world.get_resource::<Simulation>().unwrap();
        (simulation.generation, simulation.control)
    };
    if control.state == SimulationControlState::Paused {
        return;
    }
    let start_time = Instant::now();
    world.resource_scope(|world: &mut World, mut schedule: Mut<MySchedule>| {
        let mut cur_steps = 0;
        loop {
            schedule.0.run(world);
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
    });
}

pub fn exit_at_generation(
    simulation: Res<Simulation>,
    config: Res<SimulationConfig>,
    mut app_exit_event: EventWriter<AppExit>,
) {
    if config
        .exit_at_generation
        .map(|g| simulation.generation >= g)
        == Some(true)
    {
        app_exit_event.send_default();
    }
}
