use bevy::{
    ecs::batching::BatchingStrategy,
    math::Vec2,
    prelude::{Commands, Entity, Query, Res, Resource, Transform, With},
};
use rand::{Rng, SeedableRng, rngs::StdRng};

use crate::{components_and_resources::BoundingCircleComponent, config::RunConfig};

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

// Setup system to initialize the movement cache
pub fn setup_position_cache(
    mut commands: Commands,
    run_config: Res<RunConfig>,
    query: Query<Entity, With<Transform>>,
) {
    let cache_size = 1000; // Number of frames to pre-generate

    commands.insert_resource(PositionCache::new(
        run_config.rng_seed,
        Vec2::new(
            run_config.bottom_left_x as f32,
            run_config.bottom_left_y as f32,
        ),
        Vec2::new(run_config.top_right_x as f32, run_config.top_right_y as f32),
        query.iter().map(|entity| entity).collect(),
        cache_size,
    ));
}
pub fn move_entities_deterministic(
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
