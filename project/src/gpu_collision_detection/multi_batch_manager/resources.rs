use bevy::prelude::{Commands, Resource};

use crate::{
    colliding_pair::CollidingPair,
    gpu_collision_detection::single_batch::convert_collidables_to_wgsl_types::PerCollidableDataRequiredByGpu,
};

pub fn setup_multi_batch_manager_resources(mut commands: Commands) {
    commands.insert_resource(GpuCollisionBatchManager {
        max_batch_size: 10,
        current_batch_job: 0,
        current_batch_data: Vec::new(),
    });
    commands.insert_resource(GpuCollisionBatchResults(Vec::new()));
    commands.insert_resource(GpuCollisionBatchJobs(Vec::new()));
}

#[derive(Resource)]
pub struct GpuCollisionBatchManager {
    pub max_batch_size: usize,
    pub current_batch_job: usize,
    pub current_batch_data: Vec<PerCollidableDataRequiredByGpu>,
}

#[derive(Resource)]
pub struct GpuCollisionBatchResults(pub Vec<(GpuCollisionBatchJob, Vec<CollidingPair>)>);

#[derive(Debug, Clone, Resource)]
pub struct GpuCollisionBatchJobs(pub Vec<GpuCollisionBatchJob>);

#[derive(Debug, Clone)]
pub struct GpuCollisionBatchJob {
    pub name: String,
    pub run_id: Option<u128>,
    pub start_index_incl: usize,
    pub end_index_excl: usize,
    // used to allow combination of two sections
    pub second_start_index_incl: Option<usize>, //todo not using this
    pub second_end_index_excl: Option<usize>,   // todo not using this
    pub dedup_against_other_batch_job: Option<usize>,
}
