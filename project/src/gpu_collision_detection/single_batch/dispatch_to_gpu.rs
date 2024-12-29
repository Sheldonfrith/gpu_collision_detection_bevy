use bevy::{
    log,
    prelude::{Res, ResMut},
    render::renderer::{RenderDevice, RenderQueue},
};

use crate::gpu_collision_detection::population_dependent_resources::batch_size_dependent_resources::{pipeline::cache::{PipelineCache, PipelineKey}, resources::{BatchCollidablePopulation, MaxNumResultsToReceiveFromGpu, NumGpuWorkgroupsRequired}};

use super::resources::SingleBatchBindGroup;

pub fn dispatch_to_gpu(
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    mut compute_pipeline_cache: ResMut<PipelineCache>,
    bind_group: Res<SingleBatchBindGroup>,
    num_gpu_workgroups_required: Res<NumGpuWorkgroupsRequired>,
    batch_collidable_population: Res<BatchCollidablePopulation>,
    max_num_results: Res<MaxNumResultsToReceiveFromGpu>,
) {
    let mut encoder = render_device.create_command_encoder(&Default::default());
    {
        let mut compute_pass = encoder.begin_compute_pass(&Default::default());
        let key = PipelineKey {
            batch_population: batch_collidable_population.0,
            max_num_results: max_num_results.0,
        };
        compute_pass.set_pipeline(&compute_pipeline_cache.cache.get(&key).unwrap());
        compute_pass.set_bind_group(0, bind_group.0.as_ref().unwrap(), &[]);
        log::info!(
            "Dispatching workgroups: {:?}",
            num_gpu_workgroups_required.0
        );
        compute_pass.dispatch_workgroups(
            num_gpu_workgroups_required.0.0 as u32,
            // 1,
            num_gpu_workgroups_required.0.1 as u32,
            1,
            // num_gpu_workgroups_required.0.2 as u32,
        );
    }
    render_queue.submit(std::iter::once(encoder.finish()));
}
