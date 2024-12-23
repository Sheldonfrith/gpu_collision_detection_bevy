use bevy::{
    prelude::{Res, ResMut},
    render::renderer::{RenderDevice, RenderQueue},
};
use pollster::FutureExt;

use crate::{
    colliding_pair::CollidingPair,
    gpu_collision_detection::{
        multi_batch_manager::resources::{
            GpuCollisionBatchJobs, GpuCollisionBatchManager, GpuCollisionBatchResults,
        },
        population_dependent_resources::batch_size_dependent_resources::resources::MaxNumResultsToReceiveFromGpu,
        wgsl_processable_types::WgslCollisionResult,
    },
};

use super::resources::{ResultsCountFromGpu, SingleBatchBuffers, WgslIdToMetadataMap};

/**
 *   We put this all into a single system instead of passing with resources because we cannot pass the buffer slice around without lifetimes
 * The way the WGSL code works we can guarantee no duplicate collision detections WITHIN THE SAME FRAME due to entity ordering (as long as the batcher doesn't mess up the order when splitting up the data), but a collision detected as (entity1, entity2) in one frame may be detected as (entity2, entity1) in the next frame.
 * */
pub fn read_results_from_gpu(
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    results_count_from_gpu: Res<ResultsCountFromGpu>,
    max_num_results_to_recieve_from_gpu: Res<MaxNumResultsToReceiveFromGpu>,
    buffers: Res<SingleBatchBuffers>,
    wgsl_id_to_metadata: Res<WgslIdToMetadataMap>,
    batch_manager: Res<GpuCollisionBatchManager>,
    batch_jobs: Res<GpuCollisionBatchJobs>,
    mut batch_results: ResMut<GpuCollisionBatchResults>,
) {
    let mut encoder = render_device.create_command_encoder(&Default::default());
    let copy_size = std::cmp::min(
        std::mem::size_of::<WgslCollisionResult>() * results_count_from_gpu.0,
        std::mem::size_of::<WgslCollisionResult>() * max_num_results_to_recieve_from_gpu.0,
    );
    encoder.copy_buffer_to_buffer(
        &buffers.results_buffer.as_ref().unwrap(),
        0,
        &buffers.results_staging_buffer.as_ref().unwrap(),
        0,
        copy_size as u64,
    );
    render_queue.submit(std::iter::once(encoder.finish()));

    let slice = buffers.results_staging_buffer.as_ref().unwrap().slice(..);
    let (sender, receiver) = futures::channel::oneshot::channel();
    slice.map_async(wgpu::MapMode::Read, move |result| {
        sender.send(result).unwrap();
    });
    render_device.poll(wgpu::Maintain::Wait);

    if receiver.block_on().unwrap().is_ok() {
        {
            let data = slice.get_mapped_range();
            let readable_data: &[WgslCollisionResult] = bytemuck::cast_slice(&data);
            let mut colliding_pairs = Vec::with_capacity(readable_data.len());
            for result in readable_data.iter() {
                colliding_pairs.push(CollidingPair {
                    metadata1: wgsl_id_to_metadata.0[result.0[0] as usize].clone(),
                    metadata2: wgsl_id_to_metadata.0[result.0[1] as usize].clone(),
                });
            }
            drop(data);
            batch_results.0.push((
                batch_jobs.0[batch_manager.current_batch_job].clone(),
                colliding_pairs,
            ));
            buffers.results_staging_buffer.as_ref().unwrap().unmap();
            return;
        }
    }
    buffers.results_staging_buffer.as_ref().unwrap().unmap();
    panic!(" receiver from gpu was not okay, probable BufferAsyncError");
}
