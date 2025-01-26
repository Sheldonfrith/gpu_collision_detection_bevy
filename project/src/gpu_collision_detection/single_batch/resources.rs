use bevy::{
    prelude::Resource,
    render::render_resource::{BindGroup, Buffer},
};

use crate::gpu_collision_detection::{
    entity_metadata::CollidableMetadata, shader::collision_detection_module,
};

use super::convert_collidables_to_wgsl_types::PerCollidableDataRequiredByGpu;

#[derive(Debug, Resource)]
pub struct SingleBatchBuffers {
    pub positions_buffer: Option<Buffer>,
    pub radii_buffer: Option<Buffer>,
    pub results_buffer: Option<Buffer>,
    pub results_staging_buffer: Option<Buffer>,
    pub counter_buffer: Option<Buffer>,
}
impl Default for SingleBatchBuffers {
    fn default() -> Self {
        SingleBatchBuffers {
            positions_buffer: None,
            radii_buffer: None,
            results_buffer: None,
            results_staging_buffer: None,
            counter_buffer: None,
        }
    }
}

#[derive(Debug, Resource)]
pub struct SingleBatchBindGroup(pub Option<BindGroup>);

#[derive(Resource)]
pub struct SingleBatchDataForWgsl {
    pub positions: Vec<collision_detection_module::Position>,
    pub radii: Vec<collision_detection_module::Radius>,
}
impl Default for SingleBatchDataForWgsl {
    fn default() -> Self {
        SingleBatchDataForWgsl {
            positions: Vec::new(),
            radii: Vec::new(),
        }
    }
}

#[derive(Resource)]
pub struct CollidablesBatch(pub Vec<PerCollidableDataRequiredByGpu>);

#[derive(Resource)]
pub struct ResultsCountFromGpu(pub usize);
#[derive(Resource)]
/// Not necessary to add a dummy value at index zero. Its true that if the GPU cant find a collision it returns ID zero, but it will always return both entities with the same ID = 0, so as long as we check for duplicate entities we will never incorrectly find a collision due to this.
pub struct WgslIdToMetadataMap(pub Vec<CollidableMetadata>);
