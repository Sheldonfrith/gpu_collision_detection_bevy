use std::time::{Duration, Instant};

use bevy::{
    math::{Vec2, bounding::BoundingCircle},
    prelude::{Component, Entity, Resource},
};
use rand::{Rng, SeedableRng, rngs::StdRng};
use sysinfo::System;

#[derive(Debug, Component)]
pub struct BoundingCircleComponent(pub BoundingCircle);

// Pre-generated random movements for deterministic behavior
#[derive(Resource)]
pub struct PositionCache {
    // Store pre-generated random values for each frame
    /// (entity, position)
    cached_positions: Vec<Vec<(Entity, Vec2)>>,
    current_frame: usize,
}

impl PositionCache {
    pub fn new(
        rng_seed: u32,
        bottom_left_bounds: Vec2,
        top_right_bounds: Vec2,
        entities: Vec<Entity>,
        cache_size: usize,
    ) -> Self {
        let mut rng = StdRng::seed_from_u64(rng_seed as u64);
        let mut cached_positions = Vec::with_capacity(cache_size);
        // Pre-generate positions for each frame
        let entity_count = entities.len();
        for _ in 0..cache_size {
            let mut frame_positions = Vec::with_capacity(entity_count);
            for entity in entities.iter() {
                // limit
                let x = rng.r#gen::<f32>() * (top_right_bounds.x - bottom_left_bounds.x)
                    + bottom_left_bounds.x;
                let y = rng.r#gen::<f32>() * (top_right_bounds.y - bottom_left_bounds.y)
                    + bottom_left_bounds.y;
                let position = Vec2::new(x, y);
                frame_positions.push((*entity, position));
            }
            cached_positions.push(frame_positions);
        }

        PositionCache {
            cached_positions,
            current_frame: 0,
        }
    }

    pub fn get_position_and_radius(&self, entity: Entity) -> Option<&Vec2> {
        self.cached_positions
            .get(self.current_frame)
            .and_then(|frame_positions| {
                frame_positions
                    .iter()
                    .find(|(e, _)| *e == entity)
                    .map(|(_, position)| position)
            })
    }

    pub fn advance_frame(&mut self) {
        self.current_frame = (self.current_frame + 1) % self.cached_positions.len();
    }
}

#[derive(Resource)]
pub struct SysInfo {
    pub total_mem: u64,
}

impl Default for SysInfo {
    fn default() -> Self {
        let mut sys = System::new_all();
        // First we update all information of our `System` struct.
        sys.refresh_all();
        SysInfo {
            total_mem: sys.total_memory(),
        }
    }
}

#[derive(Debug, Clone, Component)]
pub struct CollisionTask {
    pub sensor_entity: Entity,
    pub entity: Entity,
}
#[derive(Component)]
pub struct Sensor {}

#[derive(Resource)]
pub struct EntitiesSpawned(pub usize);
