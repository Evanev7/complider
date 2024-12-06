#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
    pub position: [f32; 3],
    pub angle: f32,
    pub colour: [f32; 3],
    pub _padding2: f32,
}
