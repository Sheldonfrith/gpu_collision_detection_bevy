#[repr(C)]
#[derive(Copy, Debug, Eq, Hash, PartialEq, Clone, bytemuck::Pod, bytemuck::Zeroable)]

/// Smaller number is always index 0
pub struct WgslCollisionResult(pub [u32; 2]);

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
