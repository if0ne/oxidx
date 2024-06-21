use smallvec::SmallVec;
use windows::{
    core::Interface,
    Win32::{Foundation::BOOL, Graphics::Direct3D12::*},
};

use crate::{
    command_allocator::CommandAllocatorInterface,
    create_type,
    heap::CpuDescriptorHandle,
    impl_trait,
    misc::{Rect, Viewport},
    prelude::DxError,
    pso::{PipelineStateInterface, PrimitiveTopology, RootSignatureInterface},
    resources::{ResourceBarrier, VertexBufferView},
    HasInterface,
};

/// An interface from which [`GraphicsCommandListInterface`] inherits.
///
/// It represents an ordered set of commands that the GPU executes,
/// while allowing for extension to support other command lists than just those for graphics (such as compute and copy).
pub trait CommandListInterface: HasInterface<Raw: Interface> {
    /// Gets the type of the command list, such as direct, bundle, compute, or copy.
    fn get_type(&self) -> CommandListType;
}

pub trait GraphicsCommandListInterface: CommandListInterface {
    fn close(&self);

    fn reset(
        &self,
        command_allocator: &impl CommandAllocatorInterface,
        pso: &impl PipelineStateInterface,
    ) -> Result<(), DxError>;

    fn set_graphics_root_signature(&self, root_signature: &impl RootSignatureInterface);

    fn rs_set_viewports<'a>(&self, viewport: impl IntoIterator<Item = &'a Viewport>);
    fn rs_set_scissor_rects<'a>(&self, rects: impl IntoIterator<Item = &'a Rect>);

    fn om_set_render_targets<'a>(
        &self,
        render_targets: impl IntoIterator<Item = &'a CpuDescriptorHandle>,
        rts_single_handle_to_descriptor_range: bool,
        depth_stencil: Option<&'a CpuDescriptorHandle>,
    );
    fn clear_render_target_view<'a>(
        &self,
        rtv_handle: CpuDescriptorHandle,
        color: [f32; 4],
        rects: impl IntoIterator<Item = &'a Rect>,
    );
    fn ia_set_primitive_topology(&self, topology: PrimitiveTopology);
    fn ia_set_vertex_buffers<'a>(
        &self,
        slot: u32,
        buffers: impl IntoIterator<Item = &'a VertexBufferView>,
    );
    fn draw_instanced(
        &self,
        vertex_count_per_instance: u32,
        instance_count: u32,
        start_vertex_location: u32,
        start_instance_location: u32,
    );

    fn resource_barrier<'a>(&self, barriers: impl IntoIterator<Item = &'a ResourceBarrier<'a>>);
}

create_type! { GraphicsCommandList wrap ID3D12GraphicsCommandList }

impl_trait! {
    impl CommandListInterface =>
    GraphicsCommandList;

    fn get_type(&self) -> CommandListType {
        unsafe {
            self.0.GetType().into()
        }
    }
}

impl_trait! {
    impl GraphicsCommandListInterface =>
    GraphicsCommandList;

    fn close(&self) {
        unsafe {
            self.0.Close().unwrap(/*TODO: Error*/);
        }
    }

    fn reset(
        &self,
        command_allocator: &impl CommandAllocatorInterface,
        pso: &impl PipelineStateInterface,
    ) -> Result<(), DxError> {
        unsafe {
            self.0
                .Reset(command_allocator.as_raw_ref(), pso.as_raw_ref())
                .map_err(|_| DxError::Dummy)?
        }
        Ok(())
    }

    fn set_graphics_root_signature(&self, root_signature: &impl RootSignatureInterface) {
        unsafe { self.0.SetGraphicsRootSignature(root_signature.as_raw_ref()) }
    }

    fn rs_set_viewports<'a>(&self, viewport: impl IntoIterator<Item = &'a Viewport>) {
        let viewports = viewport
            .into_iter()
            .map(|v| v.as_raw())
            .collect::<SmallVec<[_; 8]>>();

        unsafe {
            self.0.RSSetViewports(&viewports);
        }
    }

    fn rs_set_scissor_rects<'a>(&self, rects: impl IntoIterator<Item = &'a Rect>) {
        let rects = rects
            .into_iter()
            .map(|v| v.as_raw())
            .collect::<SmallVec<[_; 8]>>();

        unsafe {
            self.0.RSSetScissorRects(&rects);
        }
    }

    fn om_set_render_targets<'a>(
        &self,
        render_targets: impl IntoIterator<Item = &'a CpuDescriptorHandle>,
        rts_single_handle_to_descriptor_range: bool,
        depth_stencil: Option<&'a CpuDescriptorHandle>,
    ) {
        let render_targets = render_targets
            .into_iter()
            .map(|rt| rt.as_raw())
            .collect::<SmallVec<[_; 8]>>();

        let render_targets_raw = if !render_targets.is_empty() {
            Some(render_targets.as_ptr() as *const _)
        } else {
            None
        };

        let depth_stencil = depth_stencil.map(|ds| ds.as_raw());
        let depth_stencil = depth_stencil.as_ref().map(|ds| ds as *const _);

        unsafe {
            self.0.OMSetRenderTargets(
                render_targets.len() as u32,
                render_targets_raw,
                BOOL(rts_single_handle_to_descriptor_range as i32),
                depth_stencil,
            );
        }
    }

    fn clear_render_target_view<'a>(
        &self,
        rtv_handle: CpuDescriptorHandle,
        color: [f32; 4],
        rects: impl IntoIterator<Item = &'a Rect>,
    ) {
        let rects = rects
            .into_iter()
            .map(|r| r.as_raw())
            .collect::<SmallVec<[_; 8]>>();

        let rects = if !rects.is_empty() {
            Some(rects.as_slice())
        } else {
            None
        };

        unsafe {
            self.0
                .ClearRenderTargetView(rtv_handle.as_raw(), &color, rects);
        }
    }

    fn ia_set_primitive_topology(&self, topology: PrimitiveTopology) {
        unsafe {
            self.0.IASetPrimitiveTopology(topology.as_raw_d3d());
        }
    }

    fn ia_set_vertex_buffers<'a>(
        &self,
        slot: u32,
        buffers: impl IntoIterator<Item = &'a VertexBufferView>,
    ) {
        let buffers = buffers
            .into_iter()
            .map(|r| r.as_raw())
            .collect::<SmallVec<[_; 8]>>();

        let buffers = if !buffers.is_empty() {
            Some(buffers.as_slice())
        } else {
            None
        };

        unsafe { self.0.IASetVertexBuffers(slot, buffers) }
    }

    fn draw_instanced(
        &self,
        vertex_count_per_instance: u32,
        instance_count: u32,
        start_vertex_location: u32,
        start_instance_location: u32,
    ) {
        unsafe {
            self.0.DrawInstanced(
                vertex_count_per_instance,
                instance_count,
                start_vertex_location,
                start_instance_location,
            )
        }
    }

    fn resource_barrier<'a>(&self, barriers: impl IntoIterator<Item = &'a ResourceBarrier<'a>>) {
        let barriers = barriers
            .into_iter()
            .map(|r| r.as_raw())
            .collect::<SmallVec<[_; 8]>>();

        unsafe { self.0.ResourceBarrier(barriers.as_slice()) }
    }
}

/// Specifies the type of a command list.
#[derive(Debug, Default, Clone, Copy)]
#[repr(i32)]
pub enum CommandListType {
    #[default]
    /// Specifies a command buffer that the GPU can execute. A direct command list doesn't inherit any GPU state.
    Direct = D3D12_COMMAND_LIST_TYPE_DIRECT.0,

    /// Specifies a command buffer that can be executed only directly via a direct command list.
    /// A bundle command list inherits all GPU state (except for the currently set pipeline state object and primitive topology).
    Bundle = D3D12_COMMAND_LIST_TYPE_BUNDLE.0,

    /// Specifies a command buffer for computing.
    Compute = D3D12_COMMAND_LIST_TYPE_COMPUTE.0,

    /// Specifies a command buffer for copying.
    Copy = D3D12_COMMAND_LIST_TYPE_COPY.0,

    /// Specifies a command buffer for video decoding.
    VideoDecode = D3D12_COMMAND_LIST_TYPE_VIDEO_DECODE.0,

    /// Specifies a command buffer for video processing.
    VideoProcess = D3D12_COMMAND_LIST_TYPE_VIDEO_PROCESS.0,

    /// Specifies a command buffer for video encoding.
    VideoEncode = D3D12_COMMAND_LIST_TYPE_VIDEO_ENCODE.0,
}
