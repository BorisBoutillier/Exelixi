use crate::*;

const FPS: u32 = 60;
// Maximum duration the simulation steps car run per frame
pub const MAX_SIMULATION_DURATION_PER_FRAME: f32 = 1.0 / (FPS as f32);

pub struct SimulationControl {
    pub state: SimulationControlState,
    // Speed_factor, a speed of 1 is 60 steps per seconds,
    pub speed_factor: u32,
}
impl Default for SimulationControl {
    fn default() -> Self {
        Self {
            state: SimulationControlState::Paused,
            speed_factor: 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq)]
pub enum SimulationControlState {
    // Simulation is currently paused
    Paused,
    // Simulation is running live at a speed of speed_factor * 60 steps per seconds,
    Run,
    // Simulation is running at maximum speed, giving back control to GUI each generation,
    Fastest,
}
pub struct SimulationRunInternalState {
    cur_generation: u32,
    cur_step: u32,
    cur_duration: Duration,
}
impl Default for SimulationRunInternalState {
    fn default() -> Self {
        Self {
            cur_generation: 0,
            cur_step: 0,
            cur_duration: Duration::ZERO,
        }
    }
}
pub fn simulation_run_criteria(
    mut internal: Local<SimulationRunInternalState>,
    time: Res<Time>,
    simulation: Res<Simulation>,
) -> ShouldRun {
    // Never advance when paused
    if simulation.control.state == SimulationControlState::Paused {
        return ShouldRun::No;
    }
    // Give back control when reaching a new generation
    // Allows gui updates and minium user control
    if simulation.generation > internal.cur_generation {
        internal.cur_generation = simulation.generation;
        internal.cur_step = 0;
        internal.cur_duration = Duration::ZERO;
        return ShouldRun::No;
    }
    internal.cur_duration += time.delta();
    // When we have reached 1/60s we can reset our counters
    if internal.cur_duration.as_secs_f32() >= MAX_SIMULATION_DURATION_PER_FRAME {
        internal.cur_step = 0;
        internal.cur_duration = Duration::ZERO;
        return ShouldRun::No;
    }
    if simulation.control.state == SimulationControlState::Run
        && internal.cur_step >= simulation.control.speed_factor
    {
        return ShouldRun::No;
    }
    internal.cur_step += 1;
    ShouldRun::YesAndCheckAgain
}
