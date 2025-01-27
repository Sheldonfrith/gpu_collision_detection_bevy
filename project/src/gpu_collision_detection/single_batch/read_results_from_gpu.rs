use bevy::{
    log,
    prelude::{Query, Res, ResMut},
};
use bevy_gpu_compute::prelude::GpuTaskReader;

use crate::{
    colliding_pair::CollidingPair,
    gpu_collision_detection::{
        multi_batch_manager::resources::{
            GpuCollisionBatchJobs, GpuCollisionBatchManager, GpuCollisionBatchResults,
        },
        shader::collision_detection_module,
    },
};

use super::resources::WgslIdToMetadataMap;

pub fn read_results_from_gpu(
    batch_jobs: Res<GpuCollisionBatchJobs>,
    batch_manager: Res<GpuCollisionBatchManager>,
    mut batch_results: ResMut<GpuCollisionBatchResults>,
    mut gpu_task_reader: GpuTaskReader,
    wgsl_id_to_metadata: Res<WgslIdToMetadataMap>,
) {
    let job = &batch_jobs.0[batch_manager.current_batch_job];
    let results = gpu_task_reader
        .latest_results::<collision_detection_module::OutputDataBuilder>("collision_detection");
    if let Ok(result) = results {
        let readable_data: Vec<collision_detection_module::CollisionResult> =
            result.collision_result.unwrap();
        log::info!("readable_data.len(): {}", readable_data.len());
        let mut colliding_pairs = Vec::with_capacity(readable_data.len());
        for result in readable_data.iter() {
            let e1 = result.entity1;
            let e2 = result.entity2;
            if e1 != e2 {
                let m1 = wgsl_id_to_metadata.0[e1 as usize].clone();
                let m2 = wgsl_id_to_metadata.0[e2 as usize].clone();
                if m1.is_sensor || m2.is_sensor {
                    log::info!("sensor collision detected in read results");
                }
                colliding_pairs.push(CollidingPair {
                    metadata1: wgsl_id_to_metadata.0[e1 as usize].clone(),
                    metadata2: wgsl_id_to_metadata.0[e2 as usize].clone(),
                });
            }
        }
        log::info!("colliding_pairs.len(): {}", colliding_pairs.len());
        // log::info!("colliding_pairs: {:?}", colliding_pairs);
        batch_results.0.push((
            batch_jobs.0[batch_manager.current_batch_job].clone(),
            colliding_pairs,
        ));
    } else {
        panic!("No result found for job: {}", job.name);
    }
}
