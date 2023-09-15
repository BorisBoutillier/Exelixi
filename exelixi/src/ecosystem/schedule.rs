use bevy::ecs::schedule::ScheduleLabel;

use crate::ecosystem::*;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct EcosystemSchedule;
impl EcosystemSchedule {
    pub fn new_schedule() -> Schedule {
        let mut schedule = Schedule::new();
        schedule.add_systems(
            (
                statistics_accumulation,
                locomotion_movement,
                build_organism_kdtree,
                mouth_eating,
                apply_deferred,
                brain_processing,
                body_energy_consumption,
                leaf_lifecycle,
                apply_deferred,
                evolve,
                auto_spawning,
                apply_deferred,
            )
                .chain(),
        );
        schedule
    }
}
