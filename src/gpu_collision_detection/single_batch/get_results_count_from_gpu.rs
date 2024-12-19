use bevy::{
    log::info_span,
    prelude::{Res, ResMut},
    render::renderer::{RenderDevice, RenderQueue},
};
use pollster::FutureExt;

use crate::gpu_collision_detection::{
    resources::CounterStagingBuffer, wgsl_processable_types::WgslCounter,
};

use super::resources::{ResultsCountFromGpu, SingleBatchBuffers};

pub fn get_results_count_from_gpu(
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    buffers: Res<SingleBatchBuffers>,
    counter_staging_buffer: Res<CounterStagingBuffer>,
    mut results_count_from_gpu: ResMut<ResultsCountFromGpu>,
) {
    // let my_span = info_span!("getting_count_buffer", name = "getting_count_buffer").entered();

    let mut encoder = render_device.create_command_encoder(&Default::default());

    encoder.copy_buffer_to_buffer(
        &buffers.counter_buffer.as_ref().unwrap(),
        0,
        &counter_staging_buffer.0,
        0,
        std::mem::size_of::<WgslCounter>() as u64,
    );
    render_queue.submit(std::iter::once(encoder.finish()));

    let counter_slice = counter_staging_buffer.0.slice(..);
    let (sender, receiver) = futures::channel::oneshot::channel();
    counter_slice.map_async(wgpu::MapMode::Read, move |result| {
        sender.send(result).unwrap();
    });
    render_device.poll(wgpu::Maintain::Wait);

    let count = if receiver.block_on().unwrap().is_ok() {
        let data = counter_slice.get_mapped_range();
        let counter: &WgslCounter = bytemuck::from_bytes(&data);
        counter.count as usize
    } else {
        0
    };
    counter_staging_buffer.0.unmap();
    // log::info!("Collision count from counter buffer: {}", count);
    results_count_from_gpu.0 = count;
}
