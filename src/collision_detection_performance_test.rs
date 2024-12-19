use std::time::Instant;

use bevy::{
    DefaultPlugins,
    app::{App, AppExit, PreUpdate, Startup, Update},
    log,
    prelude::{Commands, EventWriter, IntoSystemConfigs, Res, ResMut},
    time::Time,
};

use crate::{
    colliding_pair::CollidingPairs,
    collision_detection_plugin::{CollisionDetectionMethod, CollisionDetectionPlugin},
    collision_processing::process_collisions,
    entity_movement::{move_entities_deterministic, setup_position_cache},
    entity_spawning::spawn_entities,
    graphics::plugin::GraphicsPlugin,
};

use super::components_and_resources::{PerformanceMetrics, SysInfo};

pub fn collision_detection_performance_test(collision_detection_type: CollisionDetectionMethod) {
    let mut binding = App::new();
    let _app = binding
        .add_plugins(DefaultPlugins)
        .init_resource::<PerformanceMetrics>()
        .init_resource::<SysInfo>()
        .add_plugins(GraphicsPlugin)
        .add_systems(
            Startup,
            (setup, spawn_entities, setup_position_cache).chain(),
        )
        .add_plugins(CollisionDetectionPlugin {
            method: collision_detection_type,
        })
        .add_systems(PreUpdate, (move_entities_deterministic,).chain())
        .add_systems(
            Update,
            (process_collisions, track_performance_and_exit).chain(),
        )
        .run();
}
fn setup(mut commands: Commands) {
    commands.insert_resource(CollidingPairs(Vec::new()));
}
fn track_performance_and_exit(
    mut metrics: ResMut<PerformanceMetrics>,
    time: Res<Time>,
    method: Res<CollisionDetectionMethod>,
    mut exit: EventWriter<AppExit>,
) {
    if metrics.is_first_frame {
        metrics.is_first_frame = false;
        return;
    }

    let frame_time = time.delta();

    if metrics.start_time.is_none() {
        metrics.start_time = Some(Instant::now());
    }
    metrics.fps_sum += 1.0 / frame_time.as_secs_f32();
    metrics.fps_count += 1;
    metrics.frame_count += 1;
    metrics.total_frame_time += frame_time;
    metrics.max_frame_time = metrics.max_frame_time.max(frame_time);

    if metrics.frame_count == metrics.target_frames {
        let total_duration = metrics.start_time.unwrap().elapsed();
        let avg_frame_time = metrics.total_frame_time / metrics.frame_count;
        let max = metrics.max_frame_time;
        let collisions = metrics.total_collisions_processed;
        let frames = metrics.frame_count;
        let ave_fps = metrics.fps_sum / metrics.fps_count as f32;
        let collisions_per_frame = collisions as f32 / frames as f32;
        log::info!(
            "RESULTS FOR METHOD '{method:?}':
        Collisions Processed: {collisions}
        Collisions Per Frame: {collisions_per_frame:?}
        Duration: {total_duration:?}
        Average Frame Time: {avg_frame_time:?}
        Maximum Frame Time: {max:?}
        Average FPS: {ave_fps}
        Total Frames: {frames}
        "
        );
    }
    if metrics.frame_count >= metrics.target_frames {
        exit.send(AppExit::Success);
    }
}
