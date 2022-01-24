use crate::prelude::*;

pub fn save_default_config(config: Res<SimulationConfig>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::S) {
        config.save_as_default_file();
    }
}
