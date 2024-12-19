use bevy::diagnostic::SystemInfo;
use bevy::ecs::world;
use bevy::log::tracing_subscriber::fmt::time::SystemTime;
use bevy::render::render_resource::{
    BindGroup, BindGroupLayout, Buffer, BufferDescriptor, BufferSlice, BufferUsages,
    ComputePipeline, ShaderModule,
};
use bevy::render::renderer::{RenderDevice, RenderQueue};
use bevy::scene::ron::de;
use bevy::tasks::futures_lite::stream::block_on;
use bevy::{log, prelude::*, render};

use futures_intrusive::channel::shared::{ChannelReceiveFuture, GenericOneshotReceiver};

use crate::collision_detection_performance_test::process_collisions;

// use futures_intrusive::channel::shared::GenericOneshotReceiver;
// use futures_intrusive::channel::ChannelReceiveFuture;
use super::custom_schedule::{
    BatchedCollisionDetectionSchedule, run_batched_collision_detection_schedule,
};
use super::entity_metadata::CollidableMetadata;
use super::get_collidables::get_collidables;
use super::multi_batch_manager::combine_results::combine_results;
use super::multi_batch_manager::generate_batch_jobs::generate_batch_jobs;
use super::multi_batch_manager::resources::setup_multi_batch_manager_resources;
use super::population_dependent_resources::batch_size_dependent_resources::pipeline::update::update_pipeline;
use super::population_dependent_resources::batch_size_dependent_resources::update_wgsl_consts::update_wgsl_consts;
use super::population_dependent_resources::plugin::GpuCollisionPopDependentResourcesPlugin;
use super::resources::{
    AllCollidablesThisFrame, BindGroupLayoutsResource, CounterStagingBuffer, MaxBatchSize,
    MaxDetectableCollisionsScale, PipelineLayoutResource, WgslFile, WorkgroupSize,
};
use super::single_batch::plugin::GpuCollisionSingleBatchRunnerPlugin;
use super::wgsl_processable_types::{WgslCollisionResult, WgslCounter};
use pollster::FutureExt;
use rayon::prelude::*;
use rayon::result;
use std::char::MAX;
use std::collections::{HashMap, HashSet};
use std::future::IntoFuture;
use std::sync::{Arc, Mutex};
use wgpu::naga::back::wgsl;
use wgpu::util::BufferInitDescriptor;
use wgpu::{BufferAsyncError, ComputePipelineDescriptor, Device, Queue};

pub struct GpuCollisionDetectionPlugin {
    /**
     * A value between 0 and 1 that scales down the maximum possible collision buffer size. A value of 1.0 allocates space for all possible entity pairs to collide, while lower values reduce memory usage when you know many entities cannot possibly collide (e.g., due to spatial distribution). Default: 1.0
     */
    pub max_detectable_collisions_scale: f32,
    pub workgroup_size: u32,
}

impl Default for GpuCollisionDetectionPlugin {
    fn default() -> Self {
        Self {
            max_detectable_collisions_scale: 1.0,
            workgroup_size: 64,
        }
    }
}

/*
each frame we first get all the collidables and put those into collidables awaiting processing,
also getting the number of them, at the same time,
if the number has changed since the last frame we update all resources that rely on that
then we run the collision detection algorithm
then we process the results
 */

impl Plugin for GpuCollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        let max_detectable_collisions_scale = self.max_detectable_collisions_scale;
        let workgroup_size = self.workgroup_size;
        app.add_plugins(GpuCollisionPopDependentResourcesPlugin)
            .add_plugins(GpuCollisionSingleBatchRunnerPlugin)
            .add_systems(
                Startup,
                (
                    // create max_detectable_collisions_scale resource
                    move |mut commands: Commands| {
                        commands.insert_resource(MaxDetectableCollisionsScale(
                            max_detectable_collisions_scale,
                        ));
                        commands.insert_resource(WorkgroupSize(workgroup_size));
                        commands.insert_resource(MaxBatchSize(10));
                        commands.insert_resource(AllCollidablesThisFrame(Vec::new()));
                    },
                    setup_multi_batch_manager_resources,
                    create_persistent_gpu_resources,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (
                    update_max_batch_size,
                    get_collidables,
                    generate_batch_jobs,
                    run_batched_collision_detection_schedule,
                    combine_results,
                )
                    .chain()
                    .before(process_collisions),
            );
    }
}

fn update_max_batch_size(
    render_device: Res<RenderDevice>,                // static
    scale_factor: Res<MaxDetectableCollisionsScale>, // dynamic
    mut max_batch_size: ResMut<MaxBatchSize>,
) {
    if (scale_factor.is_changed() || max_batch_size.0 < 1) {
        let max_storage_buffer_bytes = render_device.limits().max_storage_buffer_binding_size;
        let safety_factor = 1.1;
        let per_result_size = std::mem::size_of::<WgslCollisionResult>();
        let p = per_result_size as f32;
        let t = max_storage_buffer_bytes as f32;
        let s = scale_factor.0 * safety_factor;
        let b: f32 = (1. / 2.) * (((p * s + 8. * t).sqrt() / (p.sqrt() * s.sqrt())) + 1.);
        max_batch_size.0 = b.floor() as usize;
    }
}

fn create_persistent_gpu_resources(mut commands: Commands, render_device: Res<RenderDevice>) {
    let wgsl_file =
        std::fs::read_to_string("src/game/test/gpu_collision_detection/collision.wgsl").unwrap();
    commands.insert_resource(WgslFile(wgsl_file));
    let counter_staging_buffer = render_device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Counter Staging Buffer"),
        size: std::mem::size_of::<WgslCounter>() as u64,
        usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    commands.insert_resource(CounterStagingBuffer(counter_staging_buffer));
    // Create bind group layout once
    let bind_group_layouts =
        render_device.create_bind_group_layout(Some("Collision Detection Bind Group Layout"), &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 3,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ]);

    let pipeline_layout = render_device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Collision Detection Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layouts],
        push_constant_ranges: &[],
    });
    commands.insert_resource(PipelineLayoutResource(pipeline_layout));
    commands.insert_resource(BindGroupLayoutsResource(bind_group_layouts));
}
