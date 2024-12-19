use collision_detection_plugin::CollisionDetectionMethod;

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

fn main() {
    // results: gpu 4.7s, 23fps, 32 851 794 collisions
    // results: cpu 7.8s, 10fps, 32 851 794 collisions
    // release build: cpu 8.3, 19.48fps, 32 851 794 collisions
    // release build: gpu 13.8s, 28 fps, 108 249 354 collisions
    // release build: cpu 26.1s, 22 fps, 108 249 354 collisions
    // Choose the method you want to test here
    let method = CollisionDetectionMethod::Cpu;
    collision_detection_performance_test(method);
}
