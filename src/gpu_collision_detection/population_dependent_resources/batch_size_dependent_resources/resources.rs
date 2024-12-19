use bevy::prelude::Resource;

#[derive(Resource)]
pub struct BatchCollidablePopulation(pub usize);

#[derive(Resource)]

pub struct MaxNumResultsToReceiveFromGpu(pub usize);

#[derive(Resource)]
pub struct NumGpuWorkgroupsRequired(pub usize);
