use bevy::prelude::{Res, ResMut};

use crate::gpu_collision_detection::multi_batch_manager::resources::GpuCollisionBatchJob;
use crate::gpu_collision_detection::population_dependent_resources::resources::CollidablePopulation;
use crate::gpu_collision_detection::resources::MaxBatchSize;

use super::resources::GpuCollisionBatchJobs;

pub fn generate_batch_jobs(
    population: Res<CollidablePopulation>,
    max_batch_size: Res<MaxBatchSize>,
    mut batch_jobs: ResMut<GpuCollisionBatchJobs>,
) {
    batch_jobs.0.clear();
    // Process full batches
    for i in (0..population.0).step_by(max_batch_size.0) {
        let end_index = std::cmp::min(i + max_batch_size.0, population.0);
        let batch_job_id_for_compare = batch_jobs.0.len();
        batch_jobs.0.push(GpuCollisionBatchJob {
            start_index_incl: i,
            end_index_excl: end_index,
            dedup_against_other_batch_job: None,
            second_start_index_incl: None,
            second_end_index_excl: None,
        });
        // Then process collisions between this batch and remaining entities
        let remaining_start = i + max_batch_size.0;
        if remaining_start < population.0 {
            let remaining_entities_len = population.0 - remaining_start;

            // Process cross-batch collisions in smaller chunks to respect GPU memory
            for j in (remaining_start..remaining_entities_len).step_by(max_batch_size.0) {
                let j_end = std::cmp::min(j + max_batch_size.0, remaining_entities_len);
                batch_jobs.0.push(GpuCollisionBatchJob {
                    start_index_incl: i,
                    end_index_excl: end_index,
                    dedup_against_other_batch_job: Some(batch_job_id_for_compare),
                    second_start_index_incl: Some(j),
                    second_end_index_excl: Some(j_end),
                });
            }
        }
    }
}
