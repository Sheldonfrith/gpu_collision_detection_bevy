use bytemuck::AnyBitPattern;

#[repr(C)]
#[derive(Copy, Debug, Eq, Hash, PartialEq, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct WgslCollisionResult {
    pub entity1: u32,
    pub entity2: u32,
}
// pub struct WgslCollisionResult {
//     max_array_size: u32,
//     array_size: u32,
//     global_id_x: u32,
//     global_id_y: u32,
//     global_id_z: u32,
//     workgroup_id_x: u32,
//     workgroup_size_x: u32,
//     workgroup_id_y: u32,
//     workgroup_size_y: u32,
//     workgroup_id_z: u32,
//     workgroup_size_z: u32,
// }
#[repr(C)]
#[derive(Clone, Debug)]
pub struct WgslDynamicCollisionResults {
    pub results: Vec<WgslCollisionResult>,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct WgslDynamicPositions {
    pub positions: Vec<[f32; 2]>,
}
impl Default for WgslDynamicPositions {
    fn default() -> Self {
        Self {
            positions: Vec::new(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct WgslDynamicRadii {
    pub radii: Vec<f32>,
}
impl Default for WgslDynamicRadii {
    fn default() -> Self {
        Self { radii: Vec::new() }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct WgslCounter {
    pub count: u32,
}
