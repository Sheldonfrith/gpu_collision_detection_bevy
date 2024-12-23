use bevy::{prelude::Resource, render::render_resource::ComputePipeline};

use crate::helpers::ecs::lru_cache::LruCache;

// #[derive(Resource)]
// pub struct ComputePipelineResource(pub ComputePipeline);

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct PipelineKey {
    pub batch_population: usize,
    pub max_num_results: usize,
}

#[derive(Resource)]
pub struct PipelineCache {
    pub cache: LruCache<PipelineKey, ComputePipeline>,
}

impl PipelineCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: LruCache::new(capacity),
        }
    }
}
