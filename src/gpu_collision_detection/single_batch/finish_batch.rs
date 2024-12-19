use bevy::{
    log,
    prelude::{Res, ResMut},
};

use crate::gpu_collision_detection::{
    multi_batch_manager::resources::{GpuCollisionBatchJobs, GpuCollisionBatchManager},
    resources::AllCollidablesThisFrame,
};

use super::resources::CollidablesBatch;

pub fn finish_batch(
    mut batch_manager: ResMut<GpuCollisionBatchManager>,
    jobs: Res<GpuCollisionBatchJobs>,
    all_collidables: Res<AllCollidablesThisFrame>,
    mut batch: ResMut<CollidablesBatch>,
) {
    log::info!("Finishing batch");
    batch_manager.current_batch_job += 1;
}
