use bevy::render::renderer::RenderDevice;
use wgpu::ShaderModule;

pub fn create_collision_shader_module(
    num_colliders: u32,
    max_num_results: u32,
    workgroup_sizes: (u32, u32, u32),
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
        "const WORKGROUP_SIZE_X: u32 = 64;",
        &format!("const WORKGROUP_SIZE_X: u32 = {};", workgroup_sizes.0),
    );
    let wgsl_file = wgsl_file.replace(
        "const WORKGROUP_SIZE_Y: u32 = 1;",
        &format!("const WORKGROUP_SIZE_Y: u32 = {};", workgroup_sizes.1),
    );
    let wgsl_file = wgsl_file.replace(
        "const WORKGROUP_SIZE_Z: u32 = 1;",
        &format!("const WORKGROUP_SIZE_Z: u32 = {};", workgroup_sizes.2),
    );
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Collision Detection Shader"),
        source: wgpu::ShaderSource::Wgsl(wgsl_file.into()),
    })
}
