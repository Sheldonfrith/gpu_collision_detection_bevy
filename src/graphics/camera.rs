use bevy::{
    asset::{AssetServer, Assets},
    log,
    prelude::{
        Camera2d, Camera2dBundle, Commands, Component, Mesh, OrthographicProjection, Res, ResMut,
        Transform,
    },
    utils::default,
};

#[derive(Component)]
pub struct Flag_MyGameCamera;

pub fn spawn_camera(mut commands: Commands) {
    log::info!("Spawning camera");
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
        Flag_MyGameCamera,
    ));
}
