use std::hash::Hash;
use std::{collections::HashSet, marker::PhantomData};

use bevy::prelude::{Res, ResMut};
use bevy::{
    log,
    prelude::{Entity, Query, Transform},
};

use crate::gpu_collision_detection::multi_batch_manager::resources::GpuCollisionBatchJob;
use crate::gpu_collision_detection::population_dependent_resources::resources::CollidablePopulation;
use crate::gpu_collision_detection::resources::{AllCollidablesThisFrame, MaxBatchSize};
use crate::gpu_collision_detection::single_batch::convert_collidables_to_wgsl_types::PerCollidableDataRequiredByGpu;

use super::resources::GpuCollisionBatchJobs;

/// Process all possible collisions while respecting GPU memory constraints
/// Returns set of collision pairs
pub fn generate_batch_jobs(
    all_collidables: Res<AllCollidablesThisFrame>,
    population: Res<CollidablePopulation>,
    max_batch_size: Res<MaxBatchSize>,
    mut batch_jobs: ResMut<GpuCollisionBatchJobs>,
) {
    batch_jobs.0.clear();
    // Changed return type to owned values
    log::info!("Processing collisions in batcher");
    // Process full batches

    for i in (0..population.0).step_by(max_batch_size.0) {
        let end_index = std::cmp::min(i + max_batch_size.0, population.0);
        // let current_batch: Vec<EntityInput> =
        // all_collidables[i..end_index].to_vec();
        let current_batch_length = end_index - i;
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
            // let remaining_entities = &all_collidables[remaining_start..];
            let remaining_entities_len = population.0 - remaining_start;

            // Process cross-batch collisions in smaller chunks to respect GPU memory
            for j in (remaining_start..remaining_entities_len).step_by(max_batch_size.0) {
                // Create a vector that combines both batches
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
