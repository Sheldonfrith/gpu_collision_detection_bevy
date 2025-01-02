use bevy::{
    log,
    prelude::{Commands, Res, ResMut},
};
use gpu_accelerated_bevy::{
    resource::GpuAcceleratedBevy,
    run_ids::GpuAcceleratedBevyRunIds,
    task::{
        inputs::input_data::InputData, iteration_space::iteration_space::IterationSpace,
        outputs::definitions::max_output_vector_lengths::MaxOutputVectorLengths,
        task_commands::TaskCommands, wgsl_code::WgslCode,
    },
};
use wgpu::naga::back::wgsl;

use crate::{
    gpu_collision_detection::{
        multi_batch_manager::resources::{GpuCollisionBatchJobs, GpuCollisionBatchManager},
        resources::{AllCollidablesThisFrame, MaxDetectableCollisionsScale},
        shareable_gpu_resources::{CollisionDetectionInputType, ShareableGpuResources},
    },
    helpers::math::max_collisions::max_collisions,
};

use super::{
    convert_collidables_to_wgsl_types::{
        PerCollidableDataRequiredByGpu, convert_collidables_to_wgsl_types,
    },
    resources::{CollidablesBatch, WgslIdToMetadataMap},
};

pub fn initialize_batch(
    mut commands: Commands,
    batch_manager: Res<GpuCollisionBatchManager>,
    mut jobs: ResMut<GpuCollisionBatchJobs>,
    all_collidables: Res<AllCollidablesThisFrame>,
    mut wgsl_id_to_metadata: ResMut<WgslIdToMetadataMap>,
    mut gpu_accelerated_bevy: ResMut<GpuAcceleratedBevy>,
    shareable_resources: Res<ShareableGpuResources>,
    mut task_run_ids: ResMut<GpuAcceleratedBevyRunIds>,
    max_detectable_collisions_scale: Res<MaxDetectableCollisionsScale>,
) {
    log::info!("initialize_batch");
    let job = &mut jobs.0[batch_manager.current_batch_job];
    let batch: Vec<PerCollidableDataRequiredByGpu> =
        all_collidables.0[job.start_index_incl..job.end_index_excl].to_vec();
    let input = convert_collidables_to_wgsl_types(batch, &mut wgsl_id_to_metadata);
    // todo, change wgsl file
    let l = input.positions.positions.len();
    let r = max_collisions(l as u128) as f32 * max_detectable_collisions_scale.0;
    let i_space = IterationSpace::new(l, l, 1);
    let maxes = MaxOutputVectorLengths::new(vec![r as usize]);
    let wgsl_code = updated_wgsl(
        shareable_resources.wgsl_code.clone(),
        l as u32,
        r as u32,
        64,
    );
    log::info!("maxes: {:?}", maxes);
    let task = if gpu_accelerated_bevy.task_exists(&job.name) {
        alter_single_batch_task(
            &job.name,
            i_space,
            maxes,
            &mut commands,
            &mut gpu_accelerated_bevy,
        )
    } else {
        create_single_batch_task(
            &job.name,
            i_space,
            maxes,
            &mut commands,
            &mut gpu_accelerated_bevy,
            &shareable_resources,
        )
    };
    log::info!("task: {:?}", task);
    let mut input_data = InputData::<CollisionDetectionInputType>::empty();
    input_data.set_input0(input.positions.positions);
    input_data.set_input1(input.radii.radii);
    let run_id = task.run(&mut commands, input_data, task_run_ids);
    job.run_id = Some(run_id);
}

fn create_single_batch_task(
    name: &String,
    initial_iteration_space: IterationSpace,
    initial_max_out_vec_lengths: MaxOutputVectorLengths,
    wgsl_code: &WgslCode,
    commands: &mut Commands,
    mut gpu_accelerated_bevy: &mut GpuAcceleratedBevy,
    s: &ShareableGpuResources,
) -> TaskCommands {
    let task = gpu_accelerated_bevy.create_task(
        commands,
        name,
        initial_iteration_space,
        s.wgsl_code.clone(),
        s.input_vector_metadata_spec.clone(),
        s.output_vector_metadata_spec.clone(),
        initial_max_out_vec_lengths,
    );
    task
}

fn alter_single_batch_task(
    name: &String,
    iteration_space: IterationSpace,
    max_out_vec_lengths: MaxOutputVectorLengths,
    wgsl_code: WgslCode,
    commands: &mut Commands,
    mut gpu_accelerated_bevy: &mut GpuAcceleratedBevy,
) -> TaskCommands {
    let task = gpu_accelerated_bevy.task(name);
    task.set_iteration_space(commands, iteration_space);
    task.set_max_output_vector_lengths(commands, max_out_vec_lengths);
    task.set_wgsl_code(commands, wgsl_code);
    task.clone()
}

pub fn updated_wgsl(
    wgsl_code: WgslCode,
    num_colliders: u32,
    max_num_results: u32,
    workgroup_sizes: &WorkgroupSizes,
) -> WgslCode {
    let wgsl_string = wgsl_code.code();
    let wgsl_string = wgsl_string.replace(
        "const ARRAY_SIZE: u32 = 5;",
        &format!("const ARRAY_SIZE: u32 = {};", num_colliders),
    );
    let wgsl_string = wgsl_string.replace(
        "const MAX_ARRAY_SIZE: u32 = 5;",
        &format!("const MAX_ARRAY_SIZE: u32 = {};", max_num_results),
    );
    let wgsl_string = wgsl_string.replace(
        "const WORKGROUP_SIZE_X: u32 = 64;",
        &format!("const WORKGROUP_SIZE_X: u32 = {};", workgroup_sizes.x()),
    );
    return WgslCode::new(
        wgsl_string,
        wgsl_code.entry_point_function_name().to_string(),
    );
}
