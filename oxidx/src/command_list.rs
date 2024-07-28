use std::ffi::CStr;

use smallvec::SmallVec;
use windows::{
    core::Interface,
    Win32::{Foundation::BOOL, Graphics::Direct3D12::*},
};

use crate::{
    command_allocator::ICommandAllocator,
    create_type,
    error::DxError,
    impl_trait,
    pso::IPipelineState,
    query_heap::IQueryHeap,
    resources::{IResource, ResourceBarrier, VertexBufferView},
    root_signature::IRootSignature,
    types::*,
    HasInterface,
};

/// An interface from which [`GraphicsCommandListInterface`] inherits.
///
/// It represents an ordered set of commands that the GPU executes,
/// while allowing for extension to support other command lists than just those for graphics (such as compute and copy).
///
/// For more information: [`ID3D12CommandList interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12commandlist)
pub trait ICommandList: HasInterface<Raw: Interface> {
    /// Gets the type of the command list, such as direct, bundle, compute, or copy.
    ///
    /// For more information: [`ID3D12CommandList::GetType method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12commandlist-gettype)
    fn get_type(&self) -> CommandListType;
}

pub trait IGraphicsCommandList: ICommandList {
    // TODO: PIX FUNCTIONS

    // fn end_event(&self);
    // fn set_marker<'a>(&self, color: impl Into<u64>, label: &'a str)

    fn begin_event(&self, color: impl Into<u64>, label: impl AsRef<CStr>);
    fn begin_query(&self, query_heap: &impl IQueryHeap, r#type: QueryType, index: u32);

    fn clear_depth_stencil_view(
        &self,
        depth_stencil_view: CpuDescriptorHandle,
        clear_flags: ClearFlags,
        depth: f32,
        stencil: u8,
        rects: impl IntoIterator<Item = Rect>,
    );

    fn clear_render_target_view(
        &self,
        rtv_handle: CpuDescriptorHandle,
        color: impl Into<[f32; 4]>,
        rects: impl IntoIterator<Item = Rect>,
    );

    fn clear_state(&self, pipeline_state: Option<&impl IPipelineState>);

    fn clear_unordered_access_view_float(
        &self,
        view_gpu_handle_in_current_heap: GpuDescriptorHandle,
        view_cpu_handle: CpuDescriptorHandle,
        resource: &impl IResource,
        values: impl Into<[f32; 4]>,
        rects: impl IntoIterator<Item = Rect>,
    );

    fn clear_unordered_access_view_u32(
        &self,
        view_gpu_handle_in_current_heap: GpuDescriptorHandle,
        view_cpu_handle: CpuDescriptorHandle,
        resource: &impl IResource,
        values: impl Into<[u32; 4]>,
        rects: impl IntoIterator<Item = Rect>,
    );

    fn close(&self) -> Result<(), DxError>;

    fn copy_buffer_region(
        &self,
        dst_buffer: &impl IResource,
        dst_offset: u64,
        src_buffer: &impl IResource,
        src_offset: u64,
        num_bytes: u64,
    );

    fn copy_resource(&self, dst_resource: &impl IResource, src_resource: &impl IResource);

    fn copy_texture_region<'a, T: IResource>(
        &self,
        dst: &TextureCopyLocation<'a, T>,
        dst_x: u32,
        dst_y: u32,
        dst_z: u32,
        src: &TextureCopyLocation<'a, T>,
        src_box: Option<&Box>,
    );

    fn reset(
        &self,
        command_allocator: &impl ICommandAllocator,
        pso: &impl IPipelineState,
    ) -> Result<(), DxError>;

    fn set_graphics_root_signature(&self, root_signature: &impl IRootSignature);

    fn rs_set_viewports<'a>(&self, viewport: impl IntoIterator<Item = &'a Viewport>);
    fn rs_set_scissor_rects<'a>(&self, rects: impl IntoIterator<Item = &'a Rect>);

    fn om_set_render_targets<'a>(
        &self,
        render_targets: impl IntoIterator<Item = &'a CpuDescriptorHandle>,
        rts_single_handle_to_descriptor_range: bool,
        depth_stencil: Option<&'a CpuDescriptorHandle>,
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
    impl ICommandList =>
    GraphicsCommandList;

    fn get_type(&self) -> CommandListType {
        unsafe {
            self.0.GetType().into()
        }
    }
}

impl_trait! {
    impl IGraphicsCommandList =>
    GraphicsCommandList;

    fn close(&self) {
        unsafe {
            self.0.Close().unwrap(/*TODO: Error*/);
        }
    }

    fn reset(
        &self,
        command_allocator: &impl ICommandAllocator,
        pso: &impl IPipelineState,
    ) -> Result<(), DxError> {
        unsafe {
            self.0
                .Reset(command_allocator.as_raw_ref(), pso.as_raw_ref())
                .map_err(|_| DxError::Dummy)?
        }
        Ok(())
    }

    fn set_graphics_root_signature(&self, root_signature: &impl IRootSignature) {
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
            self.0.IASetPrimitiveTopology(topology.as_raw());
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
