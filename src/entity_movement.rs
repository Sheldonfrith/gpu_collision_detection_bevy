use bevy::{
    ecs::batching::BatchingStrategy,
    math::Vec2,
    prelude::{Commands, Entity, Query, Res, Transform, With},
};

use crate::{
    components_and_resources::{BoundingCircleComponent, PositionCache},
    config::RunConfig,
};

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
