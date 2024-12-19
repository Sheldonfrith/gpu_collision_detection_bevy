use bevy::{
    app::{App, Plugin, Startup},
    prelude::Commands,
};

use super::{
    batch_size_dependent_resources::plugin::GpuCollisionBatchSizeDependentResourcesPlugin,
    resources::CollidablePopulation,
};

pub struct GpuCollisionPopDependentResourcesPlugin;

impl Plugin for GpuCollisionPopDependentResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GpuCollisionBatchSizeDependentResourcesPlugin)
            .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(CollidablePopulation(0));
}
