const ARRAY_SIZE: u32 = 5;
const MAX_ARRAY_SIZE: u32 = 5;
const WORKGROUP_SIZE: u32 = 64;
//! Do not alter the lines above! They are controlled automatically.

struct Positions {
    positions: array<array<f32,2>,ARRAY_SIZE>
}
struct Radii {
    radii: array<f32,ARRAY_SIZE>
}
struct CollisionResult {
    entity1: u32,
    entity2: u32,
}
struct CollisionResults {
    results: array<CollisionResult, MAX_ARRAY_SIZE>,
}
struct Counter {
    count: atomic<u32>,
}

@group(0) @binding(0) var<storage, read> positions: Positions;
@group(0) @binding(1) var<storage, read> radii: Radii;
@group(0) @binding(2) var<storage, read_write> results: CollisionResults;
@group(0) @binding(3) var<storage, read_write> counter: Counter;

// Optimized distance calculation
fn calculate_distance_squared(p1: array<f32,2>, p2: array<f32,2>) -> f32 {
    let dx = p1[0] - p2[0];
    let dy = p1[1] - p2[1];
    return dx * dx + dy * dy;
}

@compute @workgroup_size(WORKGROUP_SIZE)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let current_entity = global_id.x;
    
    // Early exit if invalid entity or zero radius
    if current_entity >= ARRAY_SIZE || radii.radii[current_entity] <= 0.0 {
        return;
    }

    let current_radius = radii.radii[current_entity];
    let current_pos = positions.positions[current_entity];
    
    // Only check entities with higher indices to avoid duplicate checks
    for (var i = current_entity + 1u; i < ARRAY_SIZE; i++) {
        let other_radius = radii.radii[i];
        
        // Skip if other entity has zero radius
        if other_radius <= 0.0 {
            continue;
        }

        let dist_squared = calculate_distance_squared(current_pos, positions.positions[i]);
        let radius_sum = current_radius + other_radius;
        
        // Compare squared distances to avoid sqrt
        if dist_squared < radius_sum * radius_sum {
            let index = atomicAdd(&counter.count, 1u);
            if index < MAX_ARRAY_SIZE {
                results.results[index].entity1 = current_entity;
                results.results[index].entity2 = i;
            }
        }
    }
}