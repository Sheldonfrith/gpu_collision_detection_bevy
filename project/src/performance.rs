use bevy::app::AppExit;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::log;
use bevy::prelude::{EventWriter, Res, ResMut, Resource};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, Read};
use std::time::Instant;

use crate::collision_detection_plugin::CollisionDetectionMethod;
use crate::components_and_resources::EntitiesSpawned;
use crate::config::RunConfig;

// measure total time from second frame
// to final frame
// measure average and max frame time in that span
// measure total number of collisions detected
// Resource to track performance metrics
#[derive(Resource)]
pub struct PerformanceMetrics {
    pub start_time: Option<Instant>,
    pub max_frame_time_ms: f64,
    pub total_frame_time_ms: f64,
    pub fps_sum: f32,
    pub fps_count: u32,
    pub target_frames: u32,
    pub is_first_frame: bool,
    pub total_collisions_processed: u32,
}

impl PerformanceMetrics {
    pub fn new(num_frames_to_test: u32) -> Self {
        Self {
            start_time: None,
            max_frame_time_ms: 0.,
            total_frame_time_ms: 0.,
            fps_sum: 0.0,
            fps_count: 0,
            target_frames: num_frames_to_test,
            is_first_frame: true,
            total_collisions_processed: 0,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct PerformanceResult {
    method: String,
    collisions: u32,
    collisions_per_frame: f32,
    duration_ms: u128,
    avg_frame_time: f64,
    max_frame_time: f64,
    avg_fps: f32,
    total_frames: u32,
    entities_spawned: usize,
}

pub fn track_performance_and_exit(
    run_config: Res<RunConfig>,
    entities_spawned: Res<EntitiesSpawned>,
    mut metrics: ResMut<PerformanceMetrics>,
    diagnostics: Res<DiagnosticsStore>,
    method: Res<CollisionDetectionMethod>,
    mut exit: EventWriter<AppExit>,
) {
    if metrics.is_first_frame
        || diagnostics
            .get_measurement(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
            .is_none()
        || diagnostics
            .get_measurement(&FrameTimeDiagnosticsPlugin::FRAME_COUNT)
            .is_none()
    {
        metrics.is_first_frame = false;
        return;
    }
    if metrics.start_time.is_none() {
        metrics.start_time = Some(Instant::now());
    }
    let frame_time = diagnostics
        .get_measurement(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
        .unwrap()
        .value;
    let fps = diagnostics
        .get_measurement(&FrameTimeDiagnosticsPlugin::FPS)
        .unwrap()
        .value;
    let frame_count = diagnostics
        .get_measurement(&FrameTimeDiagnosticsPlugin::FRAME_COUNT)
        .unwrap()
        .value as u32;
    metrics.fps_sum += fps as f32;
    metrics.fps_count += 1;
    metrics.total_frame_time_ms += frame_time;
    metrics.max_frame_time_ms = metrics.max_frame_time_ms.max(frame_time);

    if frame_count == metrics.target_frames {
        let total_duration = metrics.start_time.unwrap().elapsed();
        let avg_frame_time = metrics.total_frame_time_ms / frame_count as f64;
        let max = metrics.max_frame_time_ms;
        let collisions = metrics.total_collisions_processed;
        let frames = frame_count;
        let ave_fps = metrics.fps_sum / metrics.fps_count as f32;
        let collisions_per_frame = collisions as f32 / frames as f32;

        // Create performance result object
        let result = PerformanceResult {
            method: format!("{:?}", *method),
            collisions,
            collisions_per_frame,
            duration_ms: total_duration.as_millis(),
            avg_frame_time,
            max_frame_time: max,
            avg_fps: ave_fps,
            total_frames: frames,
            entities_spawned: entities_spawned.0,
        };

        // Append to JSON file
        if let Err(e) = append_json_result(&run_config.path_to_output_json, result) {
            log::error!("Failed to write performance results to JSON: {}", e);
        }

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
    if frame_count >= metrics.target_frames {
        exit.send(AppExit::Success);
    }
}

fn append_json_result(path: &str, result: PerformanceResult) -> io::Result<()> {
    let mut contents = String::new();
    let mut existing_results: Vec<PerformanceResult> = Vec::new();

    if let Ok(mut file) = File::open(path) {
        file.read_to_string(&mut contents)?;
        if !contents.is_empty() {
            existing_results = serde_json::from_str(&contents)?;
        }
    }

    existing_results.push(result);

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    serde_json::to_writer_pretty(file, &existing_results)?;
    Ok(())
}
