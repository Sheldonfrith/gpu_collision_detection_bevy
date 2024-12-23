use bevy::prelude::ResMut;

use crate::gpu_collision_detection::multi_batch_manager::resources::GpuCollisionBatchManager;

pub fn finish_batch(mut batch_manager: ResMut<GpuCollisionBatchManager>) {
    batch_manager.current_batch_job += 1;
}
