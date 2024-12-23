use bevy::{
    prelude::{Res, ResMut},
    utils::HashSet,
};

use crate::colliding_pair::{CollidingPair, CollidingPairs};

use super::resources::GpuCollisionBatchResults;

pub fn combine_results(
    batch_results: Res<GpuCollisionBatchResults>,
    mut combined_results: ResMut<CollidingPairs>,
) {
    combined_results.0 = Vec::new();
    for (job, result) in batch_results.0.iter() {
        if job.dedup_against_other_batch_job.is_some() {
            let other_batch_job = batch_results.0[job.dedup_against_other_batch_job.unwrap()]
                .1
                .clone();
            let deduped = dedup_cross_batch_collisions(result, &other_batch_job);
            combined_results.0.extend(deduped);
        } else {
            combined_results.0.extend(result.iter().cloned());
        }
    }
}

pub fn dedup_cross_batch_collisions(
    to_dedup: &Vec<CollidingPair>,
    comparer: &Vec<CollidingPair>,
) -> Vec<CollidingPair> {
    let comparer_metadata1_hashes: HashSet<_> =
        comparer.iter().map(|pair| pair.metadata1.entity).collect();
    let valid_cross_collisions: Vec<CollidingPair> = to_dedup
        .iter()
        .filter(|pair| {
            comparer_metadata1_hashes.contains(&pair.metadata1.entity)
                != comparer_metadata1_hashes.contains(&pair.metadata2.entity)
        })
        .cloned() // Add clone to create owned values
        .collect();
    valid_cross_collisions
}
