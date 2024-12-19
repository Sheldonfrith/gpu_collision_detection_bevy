use std::num::NonZero;

use bevy::{
    app::AppExit,
    log,
    prelude::{Entity, EventWriter, Query, Res, ResMut, Transform},
};

use crate::{
    colliding_pair::CollidingPair,
    components_and_resources::{BoundingCircleComponent, Sensor, SysInfo},
    gpu_collision_detection::entity_metadata::CollidableMetadata,
    helpers::math::max_collisions::max_collisions,
};

use super::{
    population_dependent_resources::resources::CollidablePopulation,
    resources::{AllCollidablesThisFrame, MaxDetectableCollisionsScale},
    single_batch::convert_collidables_to_wgsl_types::PerCollidableDataRequiredByGpu,
};

pub fn get_collidables(
    query: Query<(
        Entity,
        &Transform,
        &BoundingCircleComponent,
        Option<&Sensor>,
    )>,
    max_detectable_collisions_scale: Res<MaxDetectableCollisionsScale>,
    sys_info: Res<SysInfo>,
    mut exit: EventWriter<AppExit>,

    mut population: ResMut<CollidablePopulation>,
    mut all_collidables: ResMut<AllCollidablesThisFrame>,
) {
    log::info!("Getting collidables");
    let mut collidables = Vec::new();
    for (entity, transform, bounding_circle, sensor) in query.iter() {
        let collidable = PerCollidableDataRequiredByGpu {
            entity,
            center_x: transform.translation.x,
            center_y: transform.translation.y,
            radius: bounding_circle.0.radius(),
            is_sensor: sensor.is_some(),
        };
        collidables.push(collidable);
    }
    population.0 = collidables.len();
    log::info!("Population: {}", population.0);
    // get theoretical max memory size of collisions
    let max_num_results_to_receive_from_gpu =
        (max_collisions(population.0 as u128) as f32 * max_detectable_collisions_scale.0) as usize;
    let collision_size = std::mem::size_of::<CollidingPair>() * max_num_results_to_receive_from_gpu;
    let in_gb = collision_size as f32 / 1024.0 / 1024.0 / 1024.0;
    let available_memory = sys_info.total_mem;
    log::info!(
        "Available memory: {} GB",
        available_memory as f32 / 1024.0 / 1024.0 / 1024.0
    );
    log::info!("Max Collision size: {} GB", in_gb);
    if (collision_size as f32 > available_memory as f32 * 0.9) {
        log::error!(
            "Not enough memory to store all collisions, either reduce the number of entities or allow more potential collision misses by lowering the max_detectable_collisions_scale"
        );
        panic!("Not enough memory to store all collisions");
    }
    all_collidables.0 = collidables;
}
