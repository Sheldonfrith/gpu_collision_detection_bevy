use std::time::Instant;

use bevy::{
    DefaultPlugins,
    app::{App, AppExit, PreUpdate, Startup, Update},
    diagnostic::{Diagnostic, Diagnostics, DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    log,
    prelude::{Commands, EventWriter, IntoSystemConfigs, Res, ResMut, Resource},
    time::Time,
};

use crate::{
    colliding_pair::CollidingPairs,
    collision_detection_plugin::{CollisionDetectionMethod, CollisionDetectionPlugin},
    collision_processing::process_collisions,
    components_and_resources::SysInfo,
    config::RunConfig,
    entity_movement::{move_entities_deterministic, setup_position_cache},
    entity_spawning::spawn_entities,
    graphics::plugin::GraphicsPlugin,
    performance::{PerformanceMetrics, track_performance_and_exit},
};

pub fn collision_detection_performance_test(
    collision_detection_type: CollisionDetectionMethod,
    run_config: RunConfig,
) {
    let mut binding = App::new();
    let _app = binding
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(PerformanceMetrics::new(run_config.num_frames_to_test))
        .init_resource::<SysInfo>()
        .insert_resource(run_config.clone())
        .add_plugins(GraphicsPlugin)
        .add_systems(
            Startup,
            (setup, spawn_entities, setup_position_cache).chain(),
        )
        .add_plugins(CollisionDetectionPlugin {
            method: collision_detection_type,
            run_config,
        })
        .add_systems(PreUpdate, (move_entities_deterministic,).chain())
        .add_systems(
            Update,
            (process_collisions, track_performance_and_exit).chain(),
        )
        .run();
}
fn setup(mut commands: Commands) {
    commands.insert_resource(CollidingPairs(Vec::new()));
}
