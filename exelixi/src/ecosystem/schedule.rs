use bevy::ecs::schedule::ScheduleLabel;

use crate::ecosystem::*;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct EcosystemSchedule;
impl EcosystemSchedule {
    pub fn new_schedule() -> Schedule {
        let mut schedule = Schedule::new(EcosystemSchedule);
        schedule.add_systems(
            (
                ensure_minimum_population,
                spawn_organism,
                statistics_accumulation,
                locomotion_movement,
                build_organism_kdtree,
                mouth_eating,
                mating,
                apply_deferred,
                eye_processing,
                brain_processing,
                body_energy_consumption,
                leaf_lifecycle,
                apply_deferred,
                (evolve, auto_spawning),
                spawn_organism,
                apply_deferred,
            )
                .chain(),
        );
        schedule
    }
}
