use common::{upload_buffer::UploadBuffer, utils::ConstantBufferData};
use glam::{Mat4, Vec2, Vec3, Vec4};
use oxidx::dx::*;

#[derive(Debug)]
pub struct FrameResource {
    pub cmd_list_alloc: CommandAllocator,
    pub pass_cb: UploadBuffer<ConstantBufferData<PassConstants>>,
    pub object_cb: UploadBuffer<ConstantBufferData<ObjectConstants>>,
    pub fence: u64,
}

impl FrameResource {
    pub fn new(device: &Device, pass_count: usize, object_count: usize) -> Self {
        let cmd_list_alloc = device
            .create_command_allocator(CommandListType::Direct)
            .unwrap();
        let pass_cb = UploadBuffer::new(device, pass_count);
        let object_cb = UploadBuffer::new(device, object_count);

        Self {
            cmd_list_alloc,
            pass_cb,
            object_cb,
            fence: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct ObjectConstants {
    pub world: Mat4,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct PassConstants {
    pub view: Mat4,
    pub inv_view: Mat4,
    pub proj: Mat4,
    pub inv_proj: Mat4,
    pub view_proj: Mat4,
    pub inv_view_proj: Mat4,
    pub eye_pos: Vec3,
    pub cb_per_object_pad1: f32,
    pub render_target_size: Vec2,
    pub inv_render_target_size: Vec2,
    pub near_z: f32,
    pub far_z: f32,
    pub total_time: f32,
    pub delta_time: f32,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Vertex {
    pub pos: Vec3,
    pub color: Vec4,
}
