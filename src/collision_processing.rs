use std::collections::HashMap;

use bevy::prelude::{Entity, Mut, Query, Res, ResMut, Transform, With, Without};

use crate::{
    colliding_pair::CollidingPairs,
    components_and_resources::{PerformanceMetrics, Sensor},
    helpers::math::my_rads::MyRads,
};

/// no refining or collision groups
/// should only process sensor-body collisions
/**
 * Many optimizations could be done here, since we know the contents of "do_realistic_work_on_collision", but because we are trying to test a system where potentially those optimizations would break the actual logic that the user wants to do, we are not doing them here.
 */
pub fn process_collisions(
    mut performance_metrics: ResMut<PerformanceMetrics>,
    collisions: Res<CollidingPairs>,
    mut sensors: Query<&mut Transform, With<Sensor>>,
    mut bodies: Query<&mut Transform, Without<Sensor>>,
) {
    const CHUNK_SIZE: usize = 32;
    let mut sensor_updates: HashMap<Entity, Vec<Entity>> = HashMap::new();
    // Group collisions by entity, only interested in sensor-body collisions

    for collision in collisions.0.iter() {
        let m = &collision.metadata1;
        let m2 = &collision.metadata2;
        if m.is_sensor && !m2.is_sensor {
            sensor_updates.entry(m.entity).or_default().push(m2.entity);
        } else if m2.is_sensor && !m.is_sensor {
            sensor_updates.entry(m2.entity).or_default().push(m.entity);
        }
    }
    for (sensor_entity, colliding_bodies) in sensor_updates.iter() {
        if let Ok(mut sensor_transform) = sensors.get_mut(*sensor_entity) {
            let chunks = colliding_bodies.chunks_exact(CHUNK_SIZE);
            let remainder = chunks.remainder().to_vec();
            for chunk in chunks {
                if let Ok(chunk_array) = <[Entity; CHUNK_SIZE]>::try_from(chunk) {
                    if let Ok(body_transforms) = bodies.get_many_mut(chunk_array) {
                        for transform in body_transforms {
                            do_realistic_work_on_collision(
                                &mut performance_metrics,
                                &mut sensor_transform,
                                transform,
                            );
                        }
                    }
                }
            }
            for c in remainder {
                if let Ok(body_transform) = bodies.get_mut(c) {
                    do_realistic_work_on_collision(
                        &mut performance_metrics,
                        &mut sensor_transform,
                        body_transform,
                    );
                }
            }
        }
    }
}

/**
 * The idea here is to have a function that responds realistically to a collision, by mutating the translations of the entities involved, but in a way that avoids any possibility of the deterministic positions cache being used to predict the next frame's positions. So we just rotate, we don't change position
 */
fn do_realistic_work_on_collision(
    performance_metrics: &mut PerformanceMetrics,
    sensor_transform: &mut Mut<'_, Transform>,
    mut entity_transform: Mut<'_, Transform>,
) {
    performance_metrics.total_collisions_processed += 1;
    sensor_transform.rotation =
        MyRads::new(sensor_transform.rotation.to_axis_angle().1 + 0.1).to_quat();
    entity_transform.rotation =
        MyRads::new(entity_transform.rotation.to_axis_angle().1 + 0.1).to_quat();
}
