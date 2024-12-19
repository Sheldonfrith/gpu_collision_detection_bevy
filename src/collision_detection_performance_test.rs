// Designed to test different performance optimizations for herd interactions system

use std::{
    collections::HashMap,
    ops::Bound,
    process::Command,
    sync::Mutex,
    time::{Duration, Instant},
};

use bevy::{
    DefaultPlugins,
    app::{App, AppExit, FixedPostUpdate, FixedUpdate, PreUpdate, Startup, Update},
    asset::{AssetServer, Assets},
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    ecs::{batching::BatchingStrategy, entity},
    log::{self, info_span},
    math::{
        Vec2, Vec3,
        bounding::{Aabb2d, BoundingCircle, IntersectsVolume},
    },
    prelude::{
        Circle, Commands, Component, Entity, EventWriter, IntoSystemConfigs, Mesh, Mesh2d, Mut,
        Query, Rectangle, Res, ResMut, Resource, Transform, With, Without,
    },
    render::{
        RenderPlugin, mesh::RectangleMeshBuilder, renderer::RenderDevice, settings::WgpuSettings,
    },
    sprite::MeshMaterial2d,
    text::cosmic_text::ttf_parser::Fixed,
    time::Time,
    utils::default,
};

use crate::{
    colliding_pair::CollidingPairs,
    config::{BODY_RADIUS, BOTTOM_LEFT_X, BOTTOM_LEFT_Y, SENSOR_RADIUS, TOP_RIGHT_X, TOP_RIGHT_Y},
    cpu_collision_detection::cpu_collision_detection::CpuCollisionDetectionPlugin,
    graphics::{
        color::colors_and_handles::{AvailableColor, ColorHandles},
        plugin::GraphicsPlugin,
    },
    helpers::math::my_rads::MyRads,
};

use super::{
    components_and_resources::{
        BoundingCircleComponent, DeterministicRng, PerformanceMetrics, PositionCache, Sensor,
        SysInfo,
    },
    gpu_collision_detection::plugin::GpuCollisionDetectionPlugin,
};
use bevy::prelude::PluginGroup;

pub fn collision_detection_performance_test() {
    let mut binding = App::new();
    let app = binding
        .add_plugins(DefaultPlugins)
        .init_resource::<DeterministicRng>()
        .init_resource::<PerformanceMetrics>()
        .init_resource::<SysInfo>()
        .add_plugins(GraphicsPlugin)
        .add_systems(Startup, (setup, setup_position_cache).chain())
        // .add_plugins(GpuCollisionDetectionPlugin {
        // max_detectable_collisions_scale: 0.2,
        // workgroup_size: 64,
        // })
        .add_plugins(CpuCollisionDetectionPlugin)
        .add_systems(PreUpdate, (move_entities_deterministic,).chain())
        .add_systems(
            Update,
            (process_collisions, track_performance_and_exit).chain(),
        )
        .run();
}

fn track_performance_and_exit(
    mut metrics: ResMut<PerformanceMetrics>,
    time: Res<Time>,
    mut exit: EventWriter<AppExit>,
) {
    if metrics.is_first_frame {
        metrics.is_first_frame = false;
        return;
    }

    let frame_time = time.delta();

    // Initialize start time on second frame
    if metrics.start_time.is_none() {
        metrics.start_time = Some(Instant::now());
    }

    metrics.frame_count += 1;
    metrics.total_frame_time += frame_time;
    metrics.max_frame_time = metrics.max_frame_time.max(frame_time);

    // Check if we've reached the target frame count
    if metrics.frame_count >= metrics.target_frames {
        let total_duration = metrics.start_time.unwrap().elapsed();
        let avg_frame_time = metrics.total_frame_time / metrics.frame_count;

        // Log the results
        log::info!("Performance Test Results:");
        log::info!(
            "Width: {}, Height: {}",
            TOP_RIGHT_X - BOTTOM_LEFT_X,
            TOP_RIGHT_Y - BOTTOM_LEFT_Y
        );
        log::info!(
            "Total Duration (excluding first frame): {:?}",
            total_duration
        );
        log::info!("Average Frame Time: {:?}", avg_frame_time);
        log::info!("Maximum Frame Time: {:?}", metrics.max_frame_time);
        log::info!("Total Frames: {}", metrics.frame_count);
        log::info!(
            "Total Collisions Detected: {}",
            metrics.total_collisions_processed
        );

        // Exit the application
        exit.send(AppExit::Success);
    }
}

// spawn a shit ton of entities
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_handles: Res<ColorHandles>,
) {
    commands.insert_resource(CollidingPairs(Vec::new()));
    let mut count = 0;
    for x in BOTTOM_LEFT_X..TOP_RIGHT_X {
        for y in BOTTOM_LEFT_Y..TOP_RIGHT_Y {
            spawn_body(
                x as f32,
                y as f32,
                &mut commands,
                &mut meshes,
                &mut color_handles,
            );
            spawn_sensor(
                x as f32,
                y as f32,
                SENSOR_RADIUS,
                &mut commands,
                &mut meshes,
                &mut color_handles,
            );
            count += 2;
        }
    }
    log::info!("total of {} entities spawned", count)
}
fn spawn_body(
    x: f32,
    y: f32,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    color_handles: &mut Res<ColorHandles>,
) {
    let color = color_handles
        .handles
        .get(&AvailableColor::BLACK)
        .unwrap()
        .clone();
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(0.5, 2.5))),
        MeshMaterial2d(color),
        Transform {
            translation: Vec3::new(x, y, 0.0),
            ..default()
        },
        BoundingCircleComponent(BoundingCircle::new(Vec2::new(x, y), BODY_RADIUS)),
    ));
}

fn spawn_sensor(
    x: f32,
    y: f32,
    radius: f32,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    color_handles: &mut Res<ColorHandles>,
) {
    let color = color_handles
        .handles
        .get(&AvailableColor::LIGHTBLUE)
        .unwrap()
        .clone();
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(radius))),
        MeshMaterial2d(color),
        Transform {
            translation: Vec3::new(x, y, 0.0),
            ..default()
        },
        // Collider::circle(radius),
        BoundingCircleComponent(BoundingCircle::new(Vec2::new(x, y), radius)),
        Sensor {},
        // CollidingEntities::default(),
    ));
}

// Setup system to initialize the movement cache
fn setup_position_cache(mut commands: Commands, query: Query<(Entity), With<Transform>>) {
    let cache_size = 1000; // Number of frames to pre-generate

    commands.insert_resource(PositionCache::new(
        Vec2::new(BOTTOM_LEFT_X as f32, BOTTOM_LEFT_Y as f32),
        Vec2::new(TOP_RIGHT_X as f32, TOP_RIGHT_Y as f32),
        query.iter().map(|(entity)| entity).collect(),
        cache_size,
    ));
}
fn move_entities_deterministic(
    positions_cache: Res<PositionCache>,
    mut query: Query<(Entity, &mut Transform, &mut BoundingCircleComponent)>,
) {
    query
        .par_iter_mut()
        .batching_strategy(BatchingStrategy::new())
        .for_each(|(entity, mut transform, mut bounding_circle)| {
            if let Some(position) = positions_cache.get_position_and_radius(entity) {
                transform.translation.x = position.x;
                transform.translation.y = position.y;
                bounding_circle.0.center = Vec2::new(position.x, position.y);
            }
        });
}

/// no refining or collision groups
/// should only process sensor-body collisions
/**
 * Many optimizations could be done here, since we know the contents of "do_realistic_work_on_collision", but because we are trying to test a system where potentially those optimizations would break the actual logic that the user wants to do, we are not doing them here.
 */
pub fn process_collisions(
    mut commands: Commands,
    mut performance_metrics: ResMut<PerformanceMetrics>,
    collisions: Res<CollidingPairs>,
    mut sensors: Query<(&mut Transform), With<Sensor>>,
    mut bodies: Query<(&mut Transform), Without<Sensor>>,
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
                        for mut transform in body_transforms {
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
                if let Ok(mut body_transform) = bodies.get_mut(c) {
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
    mut performance_metrics: &mut PerformanceMetrics,
    mut sensor_transform: &mut Mut<'_, Transform>,
    mut entity_transform: Mut<'_, Transform>,
) {
    // let my_span = info_span!(
    //     "do_realistic_work_on_collision_sensor",
    //     name = "do_realistic_work_on_collision_sensor"
    // )
    // .entered();

    performance_metrics.total_collisions_processed += 1;
    sensor_transform.rotation =
        MyRads::new(sensor_transform.rotation.to_axis_angle().1 + 0.1).to_quat();
    entity_transform.rotation =
        MyRads::new(entity_transform.rotation.to_axis_angle().1 + 0.1).to_quat();
}
// testing
// test if a rng with specific seed always produces the same results
#[cfg(test)]
mod tests {
    use rand::{Rng, SeedableRng, rngs::StdRng};

    use super::*;
    #[test]
    fn test_rng() {
        let seed = 42;
        let mut rng = StdRng::seed_from_u64(seed);
        let mut rng2 = StdRng::seed_from_u64(seed);
        let arr1 = (0..100).map(|_| rng.r#gen::<f32>()).collect::<Vec<f32>>();
        let arr2 = (0..100).map(|_| rng2.r#gen::<f32>()).collect::<Vec<f32>>();
        // println!("{:?}", arr1);
        // println!("{:?}", arr2);
        assert_eq!(arr1, arr2);
    }
}
