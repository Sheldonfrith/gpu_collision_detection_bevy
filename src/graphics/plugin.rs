use bevy::{
    app::{App, Plugin, Startup},
    prelude::{ClearColor, Color},
};

use super::{camera::spawn_camera, color::colors_and_handles::ColorHandles};

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ColorHandles>()
            .insert_resource(ClearColor(Color::srgb(0.8, 0.8, 0.8)))
            .add_systems(Startup, spawn_camera);
    }
}
