use bevy::prelude::{Res, ResMut};

use crate::gpu_collision_detection::{
    multi_batch_manager::resources::{GpuCollisionBatchJobs, GpuCollisionBatchManager},
    population_dependent_resources::batch_size_dependent_resources::resources::BatchCollidablePopulation,
    resources::AllCollidablesThisFrame,
};

use super::resources::CollidablesBatch;

pub fn initialize_batch(
    batch_manager: Res<GpuCollisionBatchManager>,
    jobs: Res<GpuCollisionBatchJobs>,
    all_collidables: Res<AllCollidablesThisFrame>,
    mut batch: ResMut<CollidablesBatch>,
    mut batch_collidable_population: ResMut<BatchCollidablePopulation>,
) {
    let job = &jobs.0[batch_manager.current_batch_job];
    batch.0 = all_collidables.0[job.start_index_incl..job.end_index_excl].to_vec();
    batch_collidable_population.0 = job.end_index_excl - job.start_index_incl;
}
