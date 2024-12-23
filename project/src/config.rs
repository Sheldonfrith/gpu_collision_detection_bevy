// pub const BOTTOM_LEFT_X: i32 = -80;
// pub const BOTTOM_LEFT_Y: i32 = -80;
// pub const TOP_RIGHT_X: i32 = 80;
// pub const TOP_RIGHT_Y: i32 = 80;
// pub const SENSOR_RADIUS: f32 = 20.5;
// pub const BODY_RADIUS: f32 = 2.5;
// pub const RNG_SEED: u32 = 1;
// pub const NUM_FRAMES_TO_TEST: u32 = 3;

use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize, Resource)]
pub struct RunConfig {
    pub bottom_left_x: i32,
    pub bottom_left_y: i32,
    pub top_right_x: i32,
    pub top_right_y: i32,
    pub sensor_radius: f32,
    pub body_radius: f32,
    pub rng_seed: u32,
    pub num_frames_to_test: u32,
    pub use_gpu: bool,
    pub path_to_output_json: String,
}
