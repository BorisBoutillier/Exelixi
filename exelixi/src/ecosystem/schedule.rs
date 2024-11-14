use bevy::ecs::schedule::{ExecutorKind, ScheduleLabel};

use crate::ecosystem::*;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct EcosystemSchedule;
impl EcosystemSchedule {
    pub fn new_schedule() -> Schedule {
        let mut schedule = Schedule::new(EcosystemSchedule);
        schedule.add_systems(
            (
                organism_aging,
                (
                    ensure_minimum_population,
                    organism_lifecycle,
                    apply_deferred,
                )
                    .chain(),
                statistics_accumulation,
                locomotion_processing,
                build_organism_kdtree,
                (mouth_processing, organism_lifecycle, apply_deferred).chain(),
                uterus_processing,
                eye_processing,
                leaf_processing,
                brain_processing,
                (body_processing, organism_lifecycle, apply_deferred).chain(),
                (
                    evolve,
                    auto_spawning,
                    reproduction_birth,
                    organism_lifecycle,
                    apply_deferred,
                )
                    .chain(),
            )
                .chain(),
        );
        schedule.set_executor_kind(ExecutorKind::SingleThreaded);
        schedule
    }
}
