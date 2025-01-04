use bevy::prelude::{Commands, Component, Query, Res, ResMut, Resource};
use gpu_accelerated_bevy::{
    resource::GpuAcceleratedBevy,
    run_ids::GpuAcceleratedBevyRunIds,
    task::{
        inputs::{
            input_data::InputData,
            input_vector_metadata_spec::{InputVectorMetadataDefinition, InputVectorsMetadataSpec},
            input_vector_types_spec::InputVectorTypesSpec,
        },
        outputs::definitions::{
            output_vector_metadata_spec::{
                OutputVectorMetadataDefinition, OutputVectorsMetadataSpec,
            },
            output_vector_types_spec::OutputVectorTypesSpec,
            type_erased_output_data::TypeErasedOutputData,
        },
        task_components::task_run_id::TaskRunId,
        wgsl_code::WgslCode,
    },
    usage_example::Unused,
};

use crate::colliding_pair::CollidingPair;

use super::{
    multi_batch_manager::resources::{
        GpuCollisionBatchJobs, GpuCollisionBatchManager, GpuCollisionBatchResults,
    },
    single_batch::resources::WgslIdToMetadataMap,
    wgsl_processable_types::WgslCollisionResult,
};

#[derive(Component)]
pub struct CollisionDetectionInputType {}
impl InputVectorTypesSpec for CollisionDetectionInputType {
    type Input0 = [f32; 2];
    type Input1 = f32;
    type Input2 = Unused;
    type Input3 = Unused;
    type Input4 = Unused;
    type Input5 = Unused;
}
pub struct CollisionDetectionOutputType {}
impl OutputVectorTypesSpec for CollisionDetectionOutputType {
    type Output0 = WgslCollisionResult;
    type Output1 = Unused;
    type Output2 = Unused;
    type Output3 = Unused;
    type Output4 = Unused;
    type Output5 = Unused;
}

#[derive(Resource)]
pub struct ShareableGpuResources {
    pub wgsl_code: WgslCode,
    pub output_vectors_metadata_spec: OutputVectorsMetadataSpec,
    pub input_vectors_metadata_spec: InputVectorsMetadataSpec,
}
impl Default for ShareableGpuResources {
    fn default() -> Self {
        ShareableGpuResources {
            wgsl_code: WgslCode::from_file(
                "src/gpu_collision_detection/collision.wgsl",
                "main".to_string(),
            ),
            input_vectors_metadata_spec: InputVectorsMetadataSpec::from_input_vector_types_spec::<
                CollisionDetectionInputType,
            >([
                // positions
                Some(&InputVectorMetadataDefinition { binding_number: 0 }),
                //radii
                Some(&InputVectorMetadataDefinition { binding_number: 1 }),
                None,
                None,
                None,
                None,
            ]),
            output_vectors_metadata_spec: OutputVectorsMetadataSpec::from_output_vector_types_spec::<
                CollisionDetectionOutputType,
            >([
                Some(&OutputVectorMetadataDefinition {
                    binding_number: 2,
                    include_count: true,
                    count_binding_number: Some(3),
                }),
                None,
                None,
                None,
                None,
                None,
            ]),
        }
    }
}
