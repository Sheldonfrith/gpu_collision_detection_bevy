use bevy::{
    app::{App, Plugin, Startup, Update},
    prelude::{Commands, IntoSystemConfigs},
};

use crate::gpu_collision_detection::{
    get_collidables::get_collidables, multi_batch_manager::generate_batch_jobs::generate_batch_jobs,
};

use super::{
    pipeline::{cache::PipelineCache, update::update_pipeline},
    resources::{
        BatchCollidablePopulation, MaxNumResultsToReceiveFromGpu, NumGpuWorkgroupsRequired,
    },
    update_wgsl_consts::{self, update_wgsl_consts},
};

pub struct GpuCollisionBatchSizeDependentResourcesPlugin;

impl Plugin for GpuCollisionBatchSizeDependentResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(MaxNumResultsToReceiveFromGpu(0));
    commands.insert_resource(NumGpuWorkgroupsRequired(0));
    commands.insert_resource(BatchCollidablePopulation(0));

    commands.insert_resource(PipelineCache::new(10));
}
