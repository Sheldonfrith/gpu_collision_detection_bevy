use bevy::prelude::{Entity, Res, ResMut};

use crate::gpu_collision_detection::{
    entity_metadata::CollidableMetadata,
    wgsl_processable_types::{WgslDynamicPositions, WgslDynamicRadii},
};

use super::resources::{CollidablesBatch, SingleBatchDataForWgsl, WgslIdToMetadataMap};

#[derive(Debug, Clone)]
pub struct PerCollidableDataRequiredByGpu {
    pub center_x: f32,
    pub center_y: f32,
    pub radius: f32,
    pub entity: Entity,
    pub is_sensor: bool,
}

pub fn convert_collidables_to_wgsl_types(
    collidables: Res<CollidablesBatch>,
    mut wgsl_id_to_metadata: ResMut<WgslIdToMetadataMap>,
    mut single_batch_data_for_wgsl: ResMut<SingleBatchDataForWgsl>,
) {
    // Process only the current batch
    let mut positions = WgslDynamicPositions {
        positions: Vec::new(),
    };
    let mut radii = WgslDynamicRadii { radii: Vec::new() };
    wgsl_id_to_metadata.0 = Vec::new();

    for collidable in &collidables.0 {
        positions
            .positions
            //  we need the x and y position, and the radius,and the entity and if it is a sensor or not
            .push([collidable.center_x, collidable.center_y]);
        radii.radii.push(collidable.radius);
        wgsl_id_to_metadata
            .0
            .push(CollidableMetadata::from(collidable));
    }
    single_batch_data_for_wgsl.positions = positions;
    single_batch_data_for_wgsl.radii = radii;
}
