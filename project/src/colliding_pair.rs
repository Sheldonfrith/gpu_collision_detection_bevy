use bevy::prelude::Resource;

use super::gpu_collision_detection::entity_metadata::CollidableMetadata;

#[derive(Debug, Clone)]
pub struct CollidingPair {
    pub metadata1: CollidableMetadata,
    pub metadata2: CollidableMetadata,
}
#[derive(Debug, Resource)]
pub struct CollidingPairs(pub Vec<CollidingPair>);
