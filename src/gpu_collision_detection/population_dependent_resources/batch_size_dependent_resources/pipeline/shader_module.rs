use bevy::render::renderer::RenderDevice;
use wgpu::ShaderModule;

pub fn create_collision_shader_module(
    num_colliders: u32,
    max_num_results: u32,
    workgroup_size: u32,
    device: &RenderDevice,
    wgsl_file: &str,
) -> ShaderModule {
    let wgsl_file = wgsl_file.replace(
        "const ARRAY_SIZE: u32 = 5;",
        &format!("const ARRAY_SIZE: u32 = {};", num_colliders),
    );
    let wgsl_file = wgsl_file.replace(
        "const MAX_ARRAY_SIZE: u32 = 5;",
        &format!("const MAX_ARRAY_SIZE: u32 = {};", max_num_results),
    );
    let wgsl_file = wgsl_file.replace(
        "const WORKGROUP_SIZE: u32 = 64;",
        &format!("const WORKGROUP_SIZE: u32 = {};", workgroup_size),
    );
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Collision Detection Shader"),
        source: wgpu::ShaderSource::Wgsl(wgsl_file.into()),
    })
}
