use std::vec;

use bevy::prelude::*;
use bevy::render::render_resource::BufferUsages;
use bevy::render::renderer::RenderDevice;
use gpu_accelerated_bevy::GpuAcceleratedBevyPlugin;
use gpu_accelerated_bevy::resource::GpuAcceleratedBevy;
use gpu_accelerated_bevy::task::inputs::input_data::InputData;
use gpu_accelerated_bevy::task::inputs::input_vector_types_spec::InputVectorTypesSpec;
use gpu_accelerated_bevy::task::outputs::definitions::max_output_vector_lengths::MaxOutputVectorLengths;
use gpu_accelerated_bevy::task::outputs::definitions::output_vector_types_spec::OutputVectorTypesSpec;
use gpu_accelerated_bevy::task::wgsl_code::WgslCode;
use gpu_accelerated_bevy::usage_example::Unused;

use crate::collision_processing::process_collisions;
use crate::config::RunConfig;

use super::custom_schedule::run_batched_collision_detection_schedule;
use super::get_collidables::get_collidables;
use super::multi_batch_manager::combine_results::combine_results;
use super::multi_batch_manager::generate_batch_jobs::generate_batch_jobs;
use super::multi_batch_manager::population::CollidablePopulation;
use super::multi_batch_manager::resources::setup_multi_batch_manager_resources;
use super::resources::{
    AllCollidablesThisFrame, BindGroupLayoutsResource, CounterStagingBuffer, MaxBatchSize,
    MaxDetectableCollisionsScale,
};
use super::shareable_gpu_resources::ShareableGpuResources;
use super::single_batch::plugin::GpuCollisionSingleBatchRunnerPlugin;
use super::wgsl_processable_types::{WgslCollisionResult, WgslCounter};

pub struct GpuCollisionDetectionPlugin {
    /**
     * A value between 0 and 1 that scales down the maximum possible collision buffer size. A value of 1.0 allocates space for all possible entity pairs to collide, while lower values reduce memory usage when you know many entities cannot possibly collide (e.g., due to spatial distribution).
     *
     * RATIONALE: We have to allocate the buffer memory to receive results from the GPU without knowing how many results we are going to receive. We know the upper limit of the number of results we are going to receive is all possible combinations of input collidable entities. However reserving that much memory every time leads to huge reductions in performance. If we dont allocate enough memory, on the other hand, then collisions are silently dropped.

    The "max_detectable_collisions_scale" variable is multiplied by the maximum theoretical possible memory size of the results, and is used to reserve LESS than the maximum amount of memory in order to improve performance. The correct value for that variable is very hard to determine. I have used manual testing to come up with a very rough function describing what that variable should be, but it still most of the time overshoots signicantly, reducing performance.

    The variable is held in a Bevy resource so if you are using this code I encourage you to mutate that value yourself, since you will know a lot more about the number of expected collisions for your scenario and therefore guess much better how much memory will be needed for results.
     */
    pub max_detectable_collisions_scale: f32,
}

impl GpuCollisionDetectionPlugin {
    pub fn new(run_config: &RunConfig) -> Self {
        Self {
            max_detectable_collisions_scale: estimate_minimum_scale_factor_to_catch_all_collisions(
                (run_config.top_right_x - run_config.bottom_left_x) as f32,
                (run_config.top_right_y - run_config.bottom_left_y) as f32,
                (run_config.sensor_radius + run_config.body_radius) / 2.,
            ),
        }
    }
}
fn estimate_minimum_scale_factor_to_catch_all_collisions(
    width: f32,
    height: f32,
    average_radius: f32,
) -> f32 {
    let f = (width * height) / average_radius;
    // equation based on very limited manual testing, lots of room for improvement
    0.07396755 + (1.054372 - 0.07396755) / (1.0 + (f / 401.5207).powf(1.816759))
}

impl Plugin for GpuCollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        let max_detectable_collisions_scale = self.max_detectable_collisions_scale;
        app.add_plugins(GpuAcceleratedBevyPlugin::no_default_schedule())
            .add_plugins(GpuCollisionSingleBatchRunnerPlugin)
            .add_systems(
                Startup,
                (
                    // create max_detectable_collisions_scale resource
                    move |mut commands: Commands| {
                        commands.insert_resource(MaxDetectableCollisionsScale(
                            max_detectable_collisions_scale,
                        ));
                        commands.insert_resource(ShareableGpuResources::default());
                        commands.insert_resource(MaxBatchSize(10));
                        commands.insert_resource(AllCollidablesThisFrame(Vec::new()));
                        commands.insert_resource(CollidablePopulation(0));
                    },
                    setup_multi_batch_manager_resources,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (
                    update_max_batch_size,
                    get_collidables,
                    generate_batch_jobs,
                    run_batched_collision_detection_schedule,
                    combine_results,
                )
                    .chain()
                    .before(process_collisions),
            );
    }
}

fn update_max_batch_size(
    render_device: Res<RenderDevice>,                // static
    scale_factor: Res<MaxDetectableCollisionsScale>, // dynamic
    mut max_batch_size: ResMut<MaxBatchSize>,
) {
    if scale_factor.is_changed() || max_batch_size.0 < 1 {
        let max_storage_buffer_bytes = render_device.limits().max_storage_buffer_binding_size;
        let safety_factor = 1.1;
        let per_result_size = std::mem::size_of::<WgslCollisionResult>();
        let p = per_result_size as f32;
        let t = max_storage_buffer_bytes as f32;
        let s = scale_factor.0 * safety_factor;
        let b: f32 = (1. / 2.) * (((p * s + 8. * t).sqrt() / (p.sqrt() * s.sqrt())) + 1.);
        max_batch_size.0 = b.floor() as usize;
    }
}
