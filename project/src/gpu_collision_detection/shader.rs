use bevy_gpu_compute::prelude::wgsl_shader_module;
#[wgsl_shader_module]
pub mod collision_detection_module {
    use bevy_gpu_compute::prelude::*;

    #[wgsl_input_array]
    struct Position {
        pub v: Vec2F32,
    }
    #[wgsl_input_array]
    type Radius = f32;
    #[wgsl_output_vec]
    struct CollisionResult {
        pub entity1: u32,
        pub entity2: u32,
    }
    fn calculate_distance_squared(p1: Vec2F32, p2: Vec2F32) -> f32 {
        let dx = p1.x - p2[0];
        let dy = p1.y - p2[1];
        return dx * dx + dy * dy;
    }
    fn main(iter_pos: WgslIterationPosition) {
        let current_entity = iter_pos.x;
        let other_entity = iter_pos.y;
        // Early exit conditions
        let out_of_bounds = current_entity >= WgslVecInput::vec_len::<Position>()
            || other_entity >= WgslVecInput::vec_len::<Position>();
        if out_of_bounds || current_entity == other_entity || current_entity >= other_entity {
            return;
        }
        let current_radius = WgslVecInput::vec_val::<Radius>(current_entity);
        let other_radius = WgslVecInput::vec_val::<Radius>(other_entity);
        if current_radius <= 0.0 || other_radius <= 0.0 {
            return;
        }
        let current_pos = WgslVecInput::vec_val::<Position>(current_entity);
        let other_pos = WgslVecInput::vec_val::<Position>(other_entity);
        let dist_squared = calculate_distance_squared(current_pos.v, other_pos.v);
        let radius_sum = (current_radius + other_radius);
        let rad_sum_sq = radius_sum * radius_sum;
        let is_collision = dist_squared < rad_sum_sq;
        if is_collision {
            WgslOutput::push::<CollisionResult>(CollisionResult {
                entity1: current_entity,
                entity2: other_entity,
            });
        }
    }
}
