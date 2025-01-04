use bevy::{
    log,
    prelude::{Entity, Res, ResMut},
};

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
    collidables: std::vec::Vec<PerCollidableDataRequiredByGpu>,
    mut wgsl_id_to_metadata: &mut WgslIdToMetadataMap,
) -> SingleBatchDataForWgsl {
    let mut positions = WgslDynamicPositions {
        positions: Vec::new(),
    };
    let mut radii = WgslDynamicRadii { radii: Vec::new() };
    wgsl_id_to_metadata.0 = Vec::new();

    let mut count = 0;
    for collidable in &collidables {
        if collidable.is_sensor {
            count += 1;
        }
        positions
            .positions
            //  we need the x and y position, and the radius,and the entity and if it is a sensor or not
            .push([collidable.center_x, collidable.center_y]);
        radii.radii.push(collidable.radius);
        wgsl_id_to_metadata
            .0
            .push(CollidableMetadata::from(collidable));
    }
    let sensors_in_wgsl = wgsl_id_to_metadata
        .0
        .iter()
        .filter(|metadata| metadata.is_sensor)
        .count();
    log::info!(
        "sensors in wgsl_id_to_metadata...count: {}",
        sensors_in_wgsl
    );

    log::info!("sensor count in convert coll to wgsl...count: {}", count);
    SingleBatchDataForWgsl { positions, radii }
}
