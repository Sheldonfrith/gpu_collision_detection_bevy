use crate::{
    colliding_pair::{CollidingPair, CollidingPairs},
    collision_processing::process_collisions,
    components_and_resources::{BoundingCircleComponent, Sensor},
    gpu_collision_detection::entity_metadata::CollidableMetadata,
};
use bevy::{math::bounding::IntersectsVolume, prelude::*};
use rayon::{iter::ParallelIterator, slice::ParallelSlice};
use std::{
    cmp::max,
    sync::{Arc, Mutex},
};

pub struct CpuCollisionDetectionPlugin;

impl Plugin for CpuCollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, detect_collisions_cpu.before(process_collisions));
    }
}

fn detect_collisions_cpu(
    collidable_query: Query<(Entity, &BoundingCircleComponent, Option<&Sensor>)>,
    mut collisions: ResMut<CollidingPairs>,
) {
    let collisions_shared: Arc<Mutex<Vec<CollidingPair>>> = Arc::new(Mutex::new(Vec::new()));
    let entities: Vec<_> = collidable_query.iter().enumerate().collect();
    let num_threads = rayon::current_num_threads();
    let batch_size = max(entities.len() / num_threads, 1);
    entities.par_chunks(batch_size).for_each(|chunk| {
        chunk
            .iter()
            .for_each(|(i, (entity, bounding_circle, sensor))| {
                let collisions_inner: Arc<Mutex<Vec<CollidingPair>>> =
                    Arc::new(Mutex::new(Vec::new()));

                // Only check against entities with higher indices
                entities[i + 1..]
                    .par_chunks(batch_size)
                    .for_each(|other_chunk| {
                        other_chunk.iter().for_each(
                            |(_k, (other_entity, other_bounding_circle, other_sensor))| {
                                if bounding_circle.0.intersects(&other_bounding_circle.0) {
                                    let mut vec = collisions_inner.lock().unwrap();
                                    vec.push(CollidingPair {
                                        metadata1: CollidableMetadata {
                                            entity: *entity,
                                            is_sensor: sensor.is_some(),
                                            x: bounding_circle.0.center.x,
                                            y: bounding_circle.0.center.y,
                                        },
                                        metadata2: CollidableMetadata {
                                            entity: *other_entity,
                                            is_sensor: other_sensor.is_some(),
                                            x: other_bounding_circle.0.center.x,
                                            y: other_bounding_circle.0.center.y,
                                        },
                                    });
                                }
                            },
                        )
                    });

                let c = Arc::try_unwrap(collisions_inner)
                    .unwrap_or_else(|_| panic!("Arc unwrap failed"));
                collisions_shared
                    .lock()
                    .unwrap()
                    .append(&mut c.lock().unwrap());
            });
    });

    let collisions_final = Arc::try_unwrap(collisions_shared)
        .unwrap_or_else(|_| panic!("Arc unwrap failed"))
        .into_inner()
        .unwrap();
    collisions.0 = collisions_final;
}
