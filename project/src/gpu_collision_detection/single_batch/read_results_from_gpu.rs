// use bevy::{
//     log,
//     prelude::{Res, ResMut},
//     render::renderer::{RenderDevice, RenderQueue},
// };
// use pollster::FutureExt;

// use crate::{
//     colliding_pair::CollidingPair,
//     gpu_collision_detection::{
//         multi_batch_manager::resources::{
//             GpuCollisionBatchJobs, GpuCollisionBatchManager, GpuCollisionBatchResults,
//         },
//         population_dependent_resources::batch_size_dependent_resources::resources::MaxNumResultsToReceiveFromGpu,
//         wgsl_processable_types::WgslCollisionResult,
//     },
// };

// use super::resources::{ResultsCountFromGpu, SingleBatchBuffers, WgslIdToMetadataMap};

// /**
//  *   We put this all into a single system instead of passing with resources because we cannot pass the buffer slice around without lifetimes
//  * The way the WGSL code works we can guarantee no duplicate collision detections WITHIN THE SAME FRAME due to entity ordering (as long as the batcher doesn't mess up the order when splitting up the data), but a collision detected as (entity1, entity2) in one frame may be detected as (entity2, entity1) in the next frame.
//  * */
// pub fn read_results_from_gpu(
//     render_device: Res<RenderDevice>,
//     render_queue: Res<RenderQueue>,
//     results_count_from_gpu: Res<ResultsCountFromGpu>,
//     max_num_results_to_recieve_from_gpu: Res<MaxNumResultsToReceiveFromGpu>,
//     buffers: Res<SingleBatchBuffers>,
//     wgsl_id_to_metadata: Res<WgslIdToMetadataMap>,
//     batch_manager: Res<GpuCollisionBatchManager>,
//     batch_jobs: Res<GpuCollisionBatchJobs>,
//     mut batch_results: ResMut<GpuCollisionBatchResults>,
// ) {
//     let mut encoder = render_device.create_command_encoder(&Default::default());
//     let copy_size = std::cmp::min(
//         std::mem::size_of::<WgslCollisionResult>() * results_count_from_gpu.0,
//         std::mem::size_of::<WgslCollisionResult>() * max_num_results_to_recieve_from_gpu.0,
//     );
//     encoder.copy_buffer_to_buffer(
//         &buffers.results_buffer.as_ref().unwrap(),
//         0,
//         &buffers.results_staging_buffer.as_ref().unwrap(),
//         0,
//         copy_size as u64,
//     );
//     render_queue.submit(std::iter::once(encoder.finish()));

//     let slice = buffers.results_staging_buffer.as_ref().unwrap().slice(..);
//     let (sender, receiver) = futures::channel::oneshot::channel();
//     slice.map_async(wgpu::MapMode::Read, move |result| {
//         sender.send(result).unwrap();
//     });
//     render_device.poll(wgpu::Maintain::Wait);

//     if receiver.block_on().unwrap().is_ok() {
//         {
//             let data = slice.get_mapped_range();
//             // log::info!("data.len(): {}", data.len());
//             let readable_data: &[WgslCollisionResult] = bytemuck::cast_slice(&data);
//             // log::info!("readable_data.len(): {}", readable_data.len());
//             // log::info!(
//             //     "readable_data: {:?}",
//             //     readable_data.iter().take(400).collect::<Vec<_>>()
//             // );
//             // pause for 10 seconds
//             // std::thread::sleep(std::time::Duration::from_secs(10));

//             let mut colliding_pairs = Vec::with_capacity(readable_data.len());
//             for result in readable_data.iter() {
//                 let e1 = result.entity1;
//                 let e2 = result.entity2;
//                 if e1 != e2 {
//                     colliding_pairs.push(CollidingPair {
//                         metadata1: wgsl_id_to_metadata.0[e1 as usize].clone(),
//                         metadata2: wgsl_id_to_metadata.0[e2 as usize].clone(),
//                     });
//                 }
//             }
//             // log::info!("colliding_pairs.len(): {}", colliding_pairs.len());
//             // log::info!("colliding_pairs: {:?}", colliding_pairs);
//             drop(data);
//             batch_results.0.push((
//                 batch_jobs.0[batch_manager.current_batch_job].clone(),
//                 colliding_pairs,
//             ));
//             buffers.results_staging_buffer.as_ref().unwrap().unmap();
//             return;
//         }
//     }
//     buffers.results_staging_buffer.as_ref().unwrap().unmap();
//     panic!(" receiver from gpu was not okay, probable BufferAsyncError");
// }
use bevy::{
    log,
    prelude::{Commands, Component, Query, Res, ResMut, Resource},
};
use gpu_accelerated_bevy::{
    resource::GpuAcceleratedBevy,
    run_ids::GpuAcceleratedBevyRunIds,
    task::{
        inputs::{
            input_data::InputData,
            input_vector_metadata_spec::{InputVectorMetadataDefinition, InputVectorMetadataSpec},
            input_vector_types_spec::InputVectorTypesSpec,
        },
        iteration_space::iteration_space::IterationSpace,
        outputs::definitions::{
            max_output_vector_lengths::MaxOutputVectorLengths,
            output_vector_metadata_spec::{
                OutputVectorMetadataDefinition, OutputVectorMetadataSpec,
            },
            output_vector_types_spec::OutputVectorTypesSpec,
            type_erased_output_data::TypeErasedOutputData,
        },
        task_components::task_run_id::TaskRunId,
        wgsl_code::WgslCode,
    },
    usage_example::Unused,
};

use crate::{
    colliding_pair::CollidingPair,
    gpu_collision_detection::{
        multi_batch_manager::resources::{
            GpuCollisionBatchJobs, GpuCollisionBatchManager, GpuCollisionBatchResults,
        },
        shareable_gpu_resources::CollisionDetectionOutputType,
    },
};

use super::resources::WgslIdToMetadataMap;

pub fn read_results_from_gpu(
    batch_jobs: Res<GpuCollisionBatchJobs>,
    batch_manager: Res<GpuCollisionBatchManager>,
    mut batch_results: ResMut<GpuCollisionBatchResults>,
    out_datas: Query<(&TaskRunId, &TypeErasedOutputData)>,
    wgsl_id_to_metadata: Res<WgslIdToMetadataMap>,
    gpu_accelerated_bevy: Res<GpuAcceleratedBevy>,
) {
    let job = &batch_jobs.0[batch_manager.current_batch_job];
    let task = gpu_accelerated_bevy.task(&job.name);
    let result_option =
        task.result::<CollisionDetectionOutputType>(job.run_id.unwrap(), &out_datas);
    if let Some(result) = result_option {
        let readable_data = result.get_output0().unwrap();
        log::info!("readable_data.len(): {}", readable_data.len());
        let mut colliding_pairs = Vec::with_capacity(readable_data.len());
        for result in readable_data.iter() {
            let e1 = result.entity1;
            let e2 = result.entity2;
            if e1 != e2 {
                colliding_pairs.push(CollidingPair {
                    metadata1: wgsl_id_to_metadata.0[e1 as usize].clone(),
                    metadata2: wgsl_id_to_metadata.0[e2 as usize].clone(),
                });
            }
        }
        log::info!("colliding_pairs.len(): {}", colliding_pairs.len());
        log::info!("colliding_pairs: {:?}", colliding_pairs);
        batch_results.0.push((
            batch_jobs.0[batch_manager.current_batch_job].clone(),
            colliding_pairs,
        ));
    } else {
        panic!("No result found for job: {}", job.name);
    }
}
