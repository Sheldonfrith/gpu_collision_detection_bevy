use bevy_gpu_compute::prelude::{BevyGpuComputeTaskCreator, IterationSpace};

use super::shader::collision_detection_module;

pub fn create_gpu_task(mut gpu_task_creator: BevyGpuComputeTaskCreator) {
    let initial_iteration_space = IterationSpace::new(100, 100, 1);
    let initial_max_output_lengths = collision_detection_module::MaxOutputLengthsBuilder::new()
        .set_collision_result(100)
        .finish();
    gpu_task_creator.create_task_from_rust_shader::<collision_detection_module::Types>(
        "collision_detection", // ensure name is unique
        collision_detection_module::parsed(),
        initial_iteration_space,
        initial_max_output_lengths,
    );
}
