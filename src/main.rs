use collision_detection_plugin::CollisionDetectionMethod;
use config::RunConfig;

use crate::collision_detection_performance_test::collision_detection_performance_test;

pub mod colliding_pair;
pub mod collision_detection_performance_test;
pub mod collision_detection_plugin;
pub mod collision_processing;
pub mod components_and_resources;
pub mod config;
pub mod cpu_collision_detection;
pub mod entity_movement;
pub mod entity_spawning;
pub mod gpu_collision_detection;
pub mod graphics;
pub mod helpers;
pub mod performance;
fn main() {
    let path_to_run_config_json = "./run_config.json";
    let run_config = serde_json::from_str::<RunConfig>(
        &std::fs::read_to_string(path_to_run_config_json).unwrap(),
    )
    .ok()
    .unwrap();

    // Choose the method you want to test here
    let mut method = CollisionDetectionMethod::Cpu;
    if run_config.use_gpu {
        method = CollisionDetectionMethod::Gpu;
    }
    collision_detection_performance_test(method, run_config);
}
