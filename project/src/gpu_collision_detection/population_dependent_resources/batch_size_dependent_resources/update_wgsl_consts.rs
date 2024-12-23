use bevy::prelude::{DetectChanges, Res, ResMut};

use crate::{
    gpu_collision_detection::resources::{MaxDetectableCollisionsScale, WorkgroupSize},
    helpers::math::max_collisions::max_collisions,
};

use super::resources::{
    BatchCollidablePopulation, MaxNumResultsToReceiveFromGpu, NumGpuWorkgroupsRequired,
};

pub fn update_wgsl_consts(
    batch_population: Res<BatchCollidablePopulation>,
    max_detectable_collisions_scale: Res<MaxDetectableCollisionsScale>,
    gpu_workgroup_size: Res<WorkgroupSize>,
    mut max_num_results_to_receive_from_gpu: ResMut<MaxNumResultsToReceiveFromGpu>,
    mut num_gpu_workgroups_required: ResMut<NumGpuWorkgroupsRequired>,
) {
    if batch_population.0 > 0 {
        if batch_population.is_changed()
            || max_detectable_collisions_scale.is_changed()
            || gpu_workgroup_size.is_changed()
        {
            max_num_results_to_receive_from_gpu.0 =
                (max_collisions(batch_population.0 as u128) as f32
                    * max_detectable_collisions_scale.0) as usize;
            num_gpu_workgroups_required.0 =
                (batch_population.0 as f32 / gpu_workgroup_size.0 as f32).ceil() as usize;
        }
    }
}
