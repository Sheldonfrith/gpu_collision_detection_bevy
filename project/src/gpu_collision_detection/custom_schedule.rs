use bevy::{ecs::schedule::ScheduleLabel, prelude::World};

use super::multi_batch_manager::resources::{GpuCollisionBatchJobs, GpuCollisionBatchManager};

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct BatchedCollisionDetectionSchedule;

// An exclusive system to run this schedule
pub fn run_batched_collision_detection_schedule(world: &mut World) {
    let mut current_job = world
        .resource::<GpuCollisionBatchManager>()
        .current_batch_job;
    let total_jobs = world.resource::<GpuCollisionBatchJobs>().0.len();
    while current_job < total_jobs {
        world.run_schedule(BatchedCollisionDetectionSchedule);
        current_job = world
            .resource_mut::<GpuCollisionBatchManager>()
            .current_batch_job;
    }
}
