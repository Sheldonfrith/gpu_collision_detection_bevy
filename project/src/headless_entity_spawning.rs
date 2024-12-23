use bevy::{
    log,
    math::{Vec2, Vec3, bounding::BoundingCircle},
    prelude::{Commands, Res, Transform},
    utils::default,
};

use crate::{
    components_and_resources::{BoundingCircleComponent, NumEntitiesSpawned, Sensor},
    config::RunConfig,
};

pub fn spawn_entities_headless(mut commands: Commands, run_config: Res<RunConfig>) {
    let mut count = 0;
    for x in run_config.bottom_left_x..run_config.top_right_x {
        for y in run_config.bottom_left_y..run_config.top_right_y {
            spawn_body_headless(x as f32, y as f32, run_config.body_radius, &mut commands);
            spawn_sensor_headless(x as f32, y as f32, run_config.sensor_radius, &mut commands);
            count += 2;
        }
    }
    log::info!("total of {} entities spawned", count);
    commands.insert_resource(NumEntitiesSpawned(count));
}

fn spawn_body_headless(x: f32, y: f32, radius: f32, commands: &mut Commands) {
    commands.spawn((
        Transform {
            translation: Vec3::new(x, y, 0.0),
            ..default()
        },
        BoundingCircleComponent(BoundingCircle::new(Vec2::new(x, y), radius)),
    ));
}

fn spawn_sensor_headless(x: f32, y: f32, radius: f32, commands: &mut Commands) {
    commands.spawn((
        Sensor {},
        Transform {
            translation: Vec3::new(x, y, 0.0),
            ..default()
        },
        BoundingCircleComponent(BoundingCircle::new(Vec2::new(x, y), radius)),
    ));
}
