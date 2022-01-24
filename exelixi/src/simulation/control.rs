#[derive(Copy, Clone)]
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
