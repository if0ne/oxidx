use std::ffi::CStr;

use smallvec::SmallVec;
use windows::{
    core::Interface,
    Win32::{Foundation::BOOL, Graphics::Direct3D12::*},
};

use crate::{
    command_allocator::ICommandAllocator,
    command_signature::ICommandSignature,
    create_type,
    descriptor_heap::IDescriptorHeap,
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

    fn copy_texture_region<T: IResource>(
        &self,
        dst: &TextureCopyLocation<'_, T>,
        dst_x: u32,
        dst_y: u32,
        dst_z: u32,
        src: &TextureCopyLocation<'_, T>,
        src_box: Option<&Box>,
    );

    fn copy_tiles(
        &self,
        tiled_resource: &impl IResource,
        tile_region_start_coordinate: &TiledResourceCoordinate,
        tile_region_size: &TileRegionSize,
        buffer: &impl IResource,
        buffer_start_offset: u64,
        flags: TileCopyFlags,
    );

    fn discard_resource(&self, resource: &impl IResource, region: &DiscardRegion<'_>);

    fn dispatch(
        &self,
        thread_group_count_x: u32,
        thread_group_count_y: u32,
        thread_group_count_z: u32,
    );

    fn draw_indexed_instanced(
        &self,
        index_count_per_instance: u32,
        instance_count: u32,
        start_index_location: u32,
        base_vertex_location: i32,
        start_instance_location: u32,
    );

    fn draw_instanced(
        &self,
        vertex_count_per_instance: u32,
        instance_count: u32,
        start_vertex_location: u32,
        start_instance_location: u32,
    );

    fn end_event(&self);

    fn end_query(&self, query_heap: &impl IQueryHeap, r#type: QueryType, index: u32);

    fn execute_bundle(&self, command_list: &impl IGraphicsCommandList);

    fn execute_indirect<'a>(
        &self,
        command_signature: &impl ICommandSignature,
        max_command_count: u32,
        argument_buffer: impl IntoIterator<Item = &'a (impl IResource + 'a)>,
        argument_buffer_offset: u64,
        count_buffer: Option<&impl IResource>,
        count_buffer_offset: u64,
    );

    fn ia_set_index_buffer(&self, view: Option<&IndexBufferView>);

    fn ia_set_primitive_topology(&self, topology: PrimitiveTopology);

    fn ia_set_vertex_buffers<'a>(
        &self,
        slot: u32,
        buffers: impl IntoIterator<Item = VertexBufferView>,
    );

    fn om_set_blend_factor(&self, blend_factor: Option<[f32; 4]>);

    fn om_set_render_targets<'a>(
        &self,
        render_targets: impl IntoIterator<Item = CpuDescriptorHandle>,
        rts_single_handle_to_descriptor_range: bool,
        depth_stencil: Option<CpuDescriptorHandle>,
    );

    fn om_set_stencil_ref(&self, stencil_ref: u32);

    fn reset(
        &self,
        command_allocator: &impl ICommandAllocator,
        pso: Option<&impl IPipelineState>,
    ) -> Result<(), DxError>;

    fn resolve_query_data(
        &self,
        query_heap: &impl IQueryHeap,
        r#type: QueryType,
        start_index: u32,
        num_queries: u32,
        dst_buffer: &impl IResource,
        aligned_dst_buffer_offset: u64,
    );

    fn resolve_subresource(
        &self,
        dst_resource: &impl IResource,
        dst_subresource: u32,
        src_resource: &impl IResource,
        src_subresource: u32,
        format: Format,
    );

    fn resource_barrier<'a>(&self, barriers: impl IntoIterator<Item = ResourceBarrier<'a>>);

    fn rs_set_scissor_rects(&self, rects: impl IntoIterator<Item = Rect>);

    fn rs_set_viewports(&self, viewport: impl IntoIterator<Item = Viewport>);

    fn set_compute_root_32bit_constant(
        &self,
        root_parameter_index: u32,
        src_data: u32,
        dest_offset_in_32bit_values: u32,
    );

    fn set_compute_root_32bit_constants<T: Copy>(
        &self,
        root_parameter_index: u32,
        src_data: &T,
        dest_offset_in_32bit_values: u32,
    );

    fn set_compute_root_constant_buffer_view(
        &self,
        root_parameter_index: u32,
        buffer_location: u64,
    );

    fn set_compute_root_descriptor_table(
        &self,
        root_parameter_index: u32,
        base_descriptor: GpuDescriptorHandle,
    );

    fn set_compute_root_shader_resource_view(
        &self,
        root_parameter_index: u32,
        buffer_location: u64,
    );

    fn set_compute_root_signature(&self, root_signature: Option<&impl IRootSignature>);

    fn set_compute_root_unordered_access_view(
        &self,
        root_parameter_index: u32,
        buffer_location: u64,
    );

    fn set_descriptor_heaps<'a>(
        &self,
        descriptor_heaps: impl IntoIterator<Item = &'a (impl IDescriptorHeap + 'a)>,
    );

    fn set_graphics_root_32bit_constant(
        &self,
        root_parameter_index: u32,
        src_data: u32,
        dest_offset_in_32bit_values: u32,
    );

    fn set_graphics_root_32bit_constants<T: Copy>(
        &self,
        root_parameter_index: u32,
        src_data: &T,
        dest_offset_in_32bit_values: u32,
    );

    fn set_graphics_root_constant_buffer_view(
        &self,
        root_parameter_index: u32,
        buffer_location: u64,
    );

    fn set_graphics_root_descriptor_table(
        &self,
        root_parameter_index: u32,
        base_descriptor: GpuDescriptorHandle,
    );

    fn set_graphics_root_shader_resource_view(
        &self,
        root_parameter_index: u32,
        buffer_location: u64,
    );

    fn set_graphics_root_signature(&self, root_signature: Option<&impl IRootSignature>);

    fn set_graphics_root_unordered_access_view(
        &self,
        root_parameter_index: u32,
        buffer_location: u64,
    );

    fn set_marker(&self, color: impl Into<u64>, label: impl AsRef<CStr>);

    fn set_pipeline_state(&self, pipeline_state: &impl IPipelineState);

    fn set_predication(
        &self,
        buffer: Option<&impl IResource>,
        aligned_buffer_offset: u64,
        operation: PredicationOp,
    );

    fn so_set_targets(
        &self,
        start_slot: u32,
        views: impl IntoIterator<Item = StreamOutputBufferView>,
    );
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
        pso: Option<&impl IPipelineState>,
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
