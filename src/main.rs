use crate::collision_detection_performance_test::collision_detection_performance_test;

pub mod colliding_pair;
pub mod collision_detection_performance_test;
pub mod components_and_resources;
pub mod config;
pub mod cpu_collision_detection;
pub mod gpu_collision_detection;
pub mod graphics;
pub mod helpers;
fn main() {
    collision_detection_performance_test();
}
