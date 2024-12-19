use bevy::{
    ecs::batching::BatchingStrategy,
    math::Vec2,
    prelude::{Commands, Entity, Query, Res, Transform, With},
};

use crate::{
    components_and_resources::{BoundingCircleComponent, PositionCache},
    config::{BOTTOM_LEFT_X, BOTTOM_LEFT_Y, TOP_RIGHT_X, TOP_RIGHT_Y},
};

// Setup system to initialize the movement cache
pub fn setup_position_cache(mut commands: Commands, query: Query<Entity, With<Transform>>) {
    let cache_size = 1000; // Number of frames to pre-generate

    commands.insert_resource(PositionCache::new(
        Vec2::new(BOTTOM_LEFT_X as f32, BOTTOM_LEFT_Y as f32),
        Vec2::new(TOP_RIGHT_X as f32, TOP_RIGHT_Y as f32),
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
