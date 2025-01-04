use bevy::{
    log,
    prelude::{Commands, Query, Res, ResMut},
};
use gpu_accelerated_bevy::task::task_specification::iteration_space::IterationSpace;
use gpu_accelerated_bevy::task::task_specification::{
    gpu_workgroup_sizes::GpuWorkgroupSizes, max_output_vector_lengths::MaxOutputVectorLengths,
};
use gpu_accelerated_bevy::{
    resource::GpuAcceleratedBevy,
    run_ids::GpuAcceleratedBevyRunIds,
    task::{
        inputs::input_data::InputData, task_commands::TaskCommands,
        task_specification::task_specification::TaskUserSpecification, wgsl_code::WgslCode,
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
    mut gpu_task_specs: Query<&mut TaskUserSpecification>,
    shareable_resources: Res<ShareableGpuResources>,
    mut task_run_ids: ResMut<GpuAcceleratedBevyRunIds>,
    max_detectable_collisions_scale: Res<MaxDetectableCollisionsScale>,
) {
    log::info!("initialize_batch");
    let job = &mut jobs.0[batch_manager.current_batch_job];
    let batch: Vec<PerCollidableDataRequiredByGpu> =
        all_collidables.0[job.start_index_incl..job.end_index_excl].to_vec();
    let input = convert_collidables_to_wgsl_types(batch, &mut wgsl_id_to_metadata);
    log::info!(
        "initialize_batch: input.positions.positions.len() = {}",
        input.positions.positions.len()
    );
    let l = input.positions.positions.len();
    let r = max_collisions(l as u128) as f32 * max_detectable_collisions_scale.0;
    let i_space = IterationSpace::new(l, l, 1);
    let maxes = MaxOutputVectorLengths::new(vec![r as usize]);
    log::info!("initialize_batch: maxes.get(0) = {}", maxes.get(0));
    let task = if gpu_accelerated_bevy.task_exists(&job.name) {
        log::info!("initialize_batch: task exists");
        let task = gpu_accelerated_bevy.task(&job.name);
        let mut task_spec = gpu_task_specs.get_mut(task.entity).unwrap();
        task_spec.set_iteration_space(&mut commands, task.entity, i_space);
        task_spec.set_max_output_vector_lengths(&mut commands, task.entity, maxes);
        let updated_wgsl = updated_wgsl(
            task_spec.wgsl_code().clone(),
            l as u32,
            r as u32,
            &task_spec.gpu_workgroup_sizes(),
        );
        task_spec.set_wgsl_code(&mut commands, task.entity, updated_wgsl);
        task
    } else {
        log::info!("initialize_batch: task does NOT exist");
        let mut new_task_spec = TaskUserSpecification::new(
            shareable_resources.input_vectors_metadata_spec.clone(),
            shareable_resources.output_vectors_metadata_spec.clone(),
            i_space,
            maxes,
            shareable_resources.wgsl_code.clone(),
        );
        log::info!("got new task spec");
        let new_wgsl_code = updated_wgsl(
            shareable_resources.wgsl_code.clone(),
            l as u32,
            r as u32,
            new_task_spec.gpu_workgroup_sizes(),
        );
        log::info!("updated wgsl code");
        new_task_spec.set_wgsl_code_no_event(new_wgsl_code);
        log::info!("set wgsl code");
        &create_single_batch_task(
            &job.name,
            new_task_spec,
            &mut commands,
            &mut gpu_accelerated_bevy,
        )
    };
    log::info!("initialize_batch: task = {:?}", task);
    let mut input_data = InputData::<CollisionDetectionInputType>::empty();
    input_data.set_input0(input.positions.positions);
    input_data.set_input1(input.radii.radii);
    let run_id = task.run(&mut commands, input_data, task_run_ids);
    job.run_id = Some(run_id);
}

fn create_single_batch_task(
    name: &String,
    task_spec: TaskUserSpecification,
    commands: &mut Commands,
    mut gpu_accelerated_bevy: &mut GpuAcceleratedBevy,
) -> TaskCommands {
    let task = gpu_accelerated_bevy.create_task(commands, name, task_spec);
    task
}

pub fn updated_wgsl(
    wgsl_code: WgslCode,
    num_colliders: u32,
    max_num_results: u32,
    workgroup_sizes: &GpuWorkgroupSizes,
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
