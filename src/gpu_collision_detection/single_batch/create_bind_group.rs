use bevy::{
    prelude::{Res, ResMut},
    render::renderer::RenderDevice,
};

use crate::gpu_collision_detection::resources::BindGroupLayoutsResource;

use super::resources::{SingleBatchBindGroup, SingleBatchBuffers};

pub fn create_bind_group(
    render_device: Res<RenderDevice>,
    bind_group_layouts: Res<BindGroupLayoutsResource>,
    buffers: Res<SingleBatchBuffers>,
    mut bind_group_res: ResMut<SingleBatchBindGroup>,
) {
    bind_group_res.0 = Some(
        render_device.create_bind_group(
            Some("Collision Detection Bind Group"),
            &bind_group_layouts.0,
            &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffers
                        .positions_buffer
                        .as_ref()
                        .unwrap()
                        .as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: buffers.radii_buffer.as_ref().unwrap().as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: buffers.results_buffer.as_ref().unwrap().as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: buffers.counter_buffer.as_ref().unwrap().as_entire_binding(),
                },
            ],
        ),
    );
}
