use crate::prelude::*;

pub fn simulation_duration(time: Res<Time>, mut simulation: ResMut<Simulation>) {
    if simulation.control.state != SimulationControlState::Paused {
        simulation.duration += time.delta();
    }
}

pub fn save_default_config(config: Res<SimulationConfig>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::S) {
        config.save_as_default_file();
    }
}
