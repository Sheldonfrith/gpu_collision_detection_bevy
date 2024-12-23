use bevy::prelude::{Camera2d, Commands, Component, OrthographicProjection, Transform};

#[derive(Component)]
pub struct MyCamera;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            near: -1000.0,
            far: 1000.0,
            scale: 0.1,
            ..OrthographicProjection::default_2d()
        },
        Transform::from_xyz(
            0., 0., 10.0, // 100.0,
        ),
        MyCamera,
    ));
}
