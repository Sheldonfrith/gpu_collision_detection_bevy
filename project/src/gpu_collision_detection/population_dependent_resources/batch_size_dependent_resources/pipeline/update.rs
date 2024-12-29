use bevy::{
    prelude::{DetectChanges, Res, ResMut},
    render::renderer::RenderDevice,
};
use wgpu::ComputePipelineDescriptor;

use crate::gpu_collision_detection::{
    population_dependent_resources::batch_size_dependent_resources::resources::{
        BatchCollidablePopulation, MaxNumResultsToReceiveFromGpu,
    },
    resources::{PipelineLayoutResource, WgslFile, WorkgroupSizes},
};

use super::{
    cache::{PipelineCache, PipelineKey},
    shader_module::create_collision_shader_module,
};

pub fn update_pipeline(
    // if either of these has changed
    batch_population: Res<BatchCollidablePopulation>,
    max_num_results: Res<MaxNumResultsToReceiveFromGpu>,
    workgroup_sizes: Res<WorkgroupSizes>,
    wgsl_file: Res<WgslFile>,
    render_device: Res<RenderDevice>,
    pipeline_layout_resource: Res<PipelineLayoutResource>,
    mut pipeline_cache: ResMut<PipelineCache>,
) {
    if max_num_results.0 > 0 && (batch_population.is_changed() || max_num_results.is_changed()) {
        let key = PipelineKey {
            batch_population: batch_population.0,
            max_num_results: max_num_results.0,
        };
        if pipeline_cache.cache.contains_key(&key) {
            return;
        } else {
            // Create new object and cache it

            let shader_module = create_collision_shader_module(
                batch_population.0 as u32,
                max_num_results.0 as u32,
                workgroup_sizes.0,
                &render_device,
                &wgsl_file.0,
            );

            // Recreate pipeline with new shader

            let compute_pipeline =
                render_device.create_compute_pipeline(&ComputePipelineDescriptor {
                    label: Some("Collision Detection Pipeline"),
                    layout: Some(&pipeline_layout_resource.0), // Use the pipeline layout instead of None
                    module: &shader_module,
                    entry_point: Some("main"),
                    compilation_options: Default::default(),
                    cache: None,
                });
            pipeline_cache.cache.insert(key, compute_pipeline);
        }
    }
}
