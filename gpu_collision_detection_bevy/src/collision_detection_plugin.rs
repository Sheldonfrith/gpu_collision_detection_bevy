use bevy::{
    app::{App, Plugin},
    log,
    prelude::Resource,
};

use crate::{
    config::RunConfig,
    cpu_collision_detection::cpu_collision_detection::CpuCollisionDetectionPlugin,
    gpu_collision_detection::plugin::GpuCollisionDetectionPlugin,
};

#[derive(Clone, Debug, Copy, Resource)]
pub enum CollisionDetectionMethod {
    Gpu,
    Cpu,
}

pub struct CollisionDetectionPlugin {
    pub method: CollisionDetectionMethod,
    pub run_config: RunConfig,
}

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        log::info!("Using collision detection method: {:?}", self.method);
        app.insert_resource(self.method.clone());
        if let CollisionDetectionMethod::Gpu = self.method {
            app.add_plugins(GpuCollisionDetectionPlugin::new(&self.run_config));
        } else {
            app.add_plugins(CpuCollisionDetectionPlugin);
        }
    }
}
