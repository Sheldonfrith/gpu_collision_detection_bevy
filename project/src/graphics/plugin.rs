use bevy::app::{App, Plugin, Startup};

use super::{camera::spawn_camera, colors_and_handles::ColorHandles};

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ColorHandles>()
            .add_systems(Startup, spawn_camera);
    }
}
