use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Debug, Eq, Hash, PartialEq, Clone, bytemuck::Pod, bytemuck::Zeroable)]

/// Smaller number is always index 0
pub struct WgslCollisionResult(pub [u32; 2]);

#[repr(C)]
#[derive(Clone, Debug)]
pub struct WgslDynamicCollisionResults {
    pub results: Vec<WgslCollisionResult>,
}

type Position = [f32; 2];
#[repr(C)]
#[derive(Clone, Debug)]
pub struct WgslDynamicPositions {
    pub positions: Vec<Position>,
}
impl Default for WgslDynamicPositions {
    fn default() -> Self {
        Self {
            positions: Vec::new(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct WgslRadius {
    pub val: f32,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct WgslDynamicRadii {
    pub radii: Vec<WgslRadius>,
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
