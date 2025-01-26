use bevy::{
    log,
    prelude::{Commands, Query, Res, ResMut},
};
use bevy_gpu_compute::prelude::{GpuTaskRunner, IterationSpace};

use crate::{
    gpu_collision_detection::{
        multi_batch_manager::resources::{GpuCollisionBatchJobs, GpuCollisionBatchManager},
        resources::{AllCollidablesThisFrame, MaxDetectableCollisionsScale},
        shader::collision_detection_module,
    },
    helpers::math::max_collisions::max_collisions,
};

use super::{
    convert_collidables_to_wgsl_types::{
        PerCollidableDataRequiredByGpu, convert_collidables_to_wgsl_types,
    },
    resources::WgslIdToMetadataMap,
};

pub fn initialize_batch(
    mut commands: Commands,
    batch_manager: Res<GpuCollisionBatchManager>,
    mut jobs: ResMut<GpuCollisionBatchJobs>,
    all_collidables: Res<AllCollidablesThisFrame>,
    mut wgsl_id_to_metadata: ResMut<WgslIdToMetadataMap>,
    max_detectable_collisions_scale: Res<MaxDetectableCollisionsScale>,
    mut gpu_tasks: GpuTaskRunner,
) {
    log::info!("initialize_batch");
    let job = &mut jobs.0[batch_manager.current_batch_job];
    let batch: Vec<PerCollidableDataRequiredByGpu> =
        all_collidables.0[job.start_index_incl..job.end_index_excl].to_vec();
    let input = convert_collidables_to_wgsl_types(batch, &mut wgsl_id_to_metadata);
    log::info!(
        "initialize_batch: input.positions.positions.len() = {}",
        input.positions.len()
    );
    let l = input.positions.len();
    let r = max_collisions(l as u128) as f32 * max_detectable_collisions_scale.0;
    let i_space = IterationSpace::new(l, l, 1);
    let maxes = collision_detection_module::MaxOutputLengthsBuilder::new()
        .set_collision_result(r as usize)
        .finish();
    let queued_commands = gpu_tasks
        .task("collision_detection")
        .mutate(Some(i_space), Some(maxes))
        .set_inputs(
            collision_detection_module::InputDataBuilder::new()
                .set_position(input.positions)
                .set_radius(input.radii)
                .finish(),
        )
        .run();
    gpu_tasks.run_commands(queued_commands);
}
