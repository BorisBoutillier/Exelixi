use bevy::ecs::schedule::ScheduleLabel;

use crate::ecosystem::*;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct EcosystemSchedule;
impl EcosystemSchedule {
    pub fn new_schedule() -> Schedule {
        let mut schedule = Schedule::new();
        schedule.add_systems(
            (
                locomotion_movement,
                mouth_eating,
                apply_system_buffers,
                brain_processing,
                body_energy_consumption,
                leaf_lifecycle,
                apply_system_buffers,
                evolve,
                food_spawning,
                apply_system_buffers,
            )
                .chain(),
        );
        schedule
    }
}
