use bevy::{
    prelude::Resource,
    render::render_resource::{BindGroup, Buffer},
};

use crate::gpu_collision_detection::{
    entity_metadata::CollidableMetadata,
    wgsl_processable_types::{WgslDynamicPositions, WgslDynamicRadii},
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
    pub positions: WgslDynamicPositions,
    pub radii: WgslDynamicRadii,
}
impl Default for SingleBatchDataForWgsl {
    fn default() -> Self {
        SingleBatchDataForWgsl {
            positions: WgslDynamicPositions::default(),
            radii: WgslDynamicRadii::default(),
        }
    }
}

#[derive(Resource)]
pub struct CollidablesBatch(pub Vec<PerCollidableDataRequiredByGpu>);

#[derive(Resource)]
pub struct ResultsCountFromGpu(pub usize);
#[derive(Resource)]
pub struct WgslIdToMetadataMap(pub Vec<CollidableMetadata>);
