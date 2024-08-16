use std::ffi::CStr;

use smallvec::SmallVec;
use windows::{
    core::{Interface, Param, PCSTR},
    Win32::{Foundation::BOOL, Graphics::Direct3D12::*},
};

use crate::{
    command_allocator::ICommandAllocator, command_signature::ICommandSignature, create_type,
    descriptor_heap::DescriptorHeap, error::DxError, impl_trait, pix::WIN_PIX_EVENT_RUNTIME,
    pso::IPipelineState, query_heap::IQueryHeap, resources::IResource,
    root_signature::IRootSignature, types::*, HasInterface,
};

/// An interface from which [`IGraphicsCommandList`] inherits.
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

/// Encapsulates a list of graphics commands for rendering. Includes APIs for instrumenting the command list execution, and for setting and clearing the pipeline state.
///
/// For more information: [`ID3D12GraphicsCommandList interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12graphicscommandlist)
pub trait IGraphicsCommandList:
    ICommandList + for<'a> HasInterface<RawRef<'a>: Param<ID3D12GraphicsCommandList>>
{
    /// Marks the start of a user-defined region of work.
    fn begin_event(&self, color: impl Into<u64>, label: impl AsRef<CStr>);

    /// Starts a query running.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::BeginQuery method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-beginquery)
    fn begin_query(&self, query_heap: &impl IQueryHeap, r#type: QueryType, index: u32);

    /// Clears the depth-stencil resource.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::ClearDepthStencilView method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-cleardepthstencilview)
    fn clear_depth_stencil_view(
        &self,
        depth_stencil_view: CpuDescriptorHandle,
        clear_flags: ClearFlags,
        depth: f32,
        stencil: u8,
        rects: impl IntoIterator<Item = Rect>,
    );

    /// Sets all the elements in a render target to one value.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::ClearRenderTargetView method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-clearrendertargetview)
    fn clear_render_target_view(
        &self,
        rtv_handle: CpuDescriptorHandle,
        color: impl Into<[f32; 4]>,
        rects: impl IntoIterator<Item = Rect>,
    );

    /// Resets the state of a direct command list back to the state it was in when the command list was created.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::ClearState method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-clearstate)
    fn clear_state(&self, pipeline_state: Option<&impl IPipelineState>);

    /// Sets all of the elements in an unordered-access view (UAV) to the specified f32 values.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::ClearUnorderedAccessViewFloat method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-clearunorderedaccessviewfloat)
    fn clear_unordered_access_view_f32(
        &self,
        view_gpu_handle_in_current_heap: GpuDescriptorHandle,
        view_cpu_handle: CpuDescriptorHandle,
        resource: &impl IResource,
        values: impl Into<[f32; 4]>,
        rects: impl IntoIterator<Item = Rect>,
    );

    /// Sets all of the elements in an unordered-access view (UAV) to the specified u32 values.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::ClearUnorderedAccessViewUint method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-clearunorderedaccessviewuint)
    fn clear_unordered_access_view_u32(
        &self,
        view_gpu_handle_in_current_heap: GpuDescriptorHandle,
        view_cpu_handle: CpuDescriptorHandle,
        resource: &impl IResource,
        values: impl Into<[u32; 4]>,
        rects: impl IntoIterator<Item = Rect>,
    );

    /// Indicates that recording to the command list has finished.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::Close method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-close)
    fn close(&self) -> Result<(), DxError>;

    /// Copies a region of a buffer from one resource to another.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::CopyBufferRegion method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-copybufferregion)
    fn copy_buffer_region(
        &self,
        dst_buffer: &impl IResource,
        dst_offset: u64,
        src_buffer: &impl IResource,
        src_offset: u64,
        num_bytes: u64,
    );

    /// Copies the entire contents of the source resource to the destination resource.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::CopyResource method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-copyresource)
    fn copy_resource(&self, dst_resource: &impl IResource, src_resource: &impl IResource);

    /// This method uses the GPU to copy texture data between two locations. Both the source and the destination may reference texture data located within either a buffer resource or a texture resource.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::CopyTextureRegion method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-copytextureregion)
    fn copy_texture_region(
        &self,
        dst: &TextureCopyLocation<'_>,
        dst_x: u32,
        dst_y: u32,
        dst_z: u32,
        src: &TextureCopyLocation<'_>,
        src_box: Option<&Box>,
    );

    /// Copies tiles from buffer to tiled resource or vice versa.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::CopyTiles method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-copytiles)
    fn copy_tiles(
        &self,
        tiled_resource: &impl IResource,
        tile_region_start_coordinate: &TiledResourceCoordinate,
        tile_region_size: &TileRegionSize,
        buffer: &impl IResource,
        buffer_start_offset: u64,
        flags: TileCopyFlags,
    );

    /// Discards a resource.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::DiscardResource method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-discardresource)
    fn discard_resource(&self, resource: &impl IResource, region: Option<&DiscardRegion<'_>>);

    /// Executes a compute shader on a thread group.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::Dispatch method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-dispatch)
    fn dispatch(
        &self,
        thread_group_count_x: u32,
        thread_group_count_y: u32,
        thread_group_count_z: u32,
    );

    /// Draws indexed, instanced primitives.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::DrawIndexedInstanced method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-drawindexedinstanced)
    fn draw_indexed_instanced(
        &self,
        index_count_per_instance: u32,
        instance_count: u32,
        start_index_location: u32,
        base_vertex_location: i32,
        start_instance_location: u32,
    );

    /// Draws non-indexed, instanced primitives.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::DrawInstanced method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-drawinstanced)
    fn draw_instanced(
        &self,
        vertex_count_per_instance: u32,
        instance_count: u32,
        start_vertex_location: u32,
        start_instance_location: u32,
    );

    /// Marks the end of a user-defined region of work.
    fn end_event(&self);

    /// Ends a running query.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::EndQuery method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-endquery)
    fn end_query(&self, query_heap: &impl IQueryHeap, r#type: QueryType, index: u32);

    /// Executes a bundle.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::ExecuteBundle method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-executebundle)
    fn execute_bundle(&self, command_list: &impl IGraphicsCommandList);

    /// Apps perform indirect draws/dispatches using the ExecuteIndirect method.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::ExecuteIndirect method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-executeindirect)
    fn execute_indirect(
        &self,
        command_signature: &impl ICommandSignature,
        max_command_count: u32,
        argument_buffer: impl IResource,
        argument_buffer_offset: u64,
        count_buffer: Option<&impl IResource>,
        count_buffer_offset: u64,
    );

    /// Sets the view for the index buffer.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::IASetIndexBuffer method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-iasetindexbuffer)
    fn ia_set_index_buffer(&self, view: Option<&IndexBufferView>);

    /// Bind information about the primitive type, and data order that describes input data for the input assembler stage.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::IASetPrimitiveTopology method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-iasetprimitivetopology)
    fn ia_set_primitive_topology(&self, topology: PrimitiveTopology);

    /// Sets a CPU descriptor handle for the vertex buffers.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::IASetVertexBuffers method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-iasetvertexbuffers)
    fn ia_set_vertex_buffers(&self, slot: u32, buffers: impl IntoIterator<Item = VertexBufferView>);

    /// Sets the blend factor that modulate values for a pixel shader, render target, or both.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::OMSetBlendFactor method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-omsetblendfactor)
    fn om_set_blend_factor(&self, blend_factor: Option<[f32; 4]>);

    /// Sets CPU descriptor handles for the render targets and depth stencil.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::OMSetRenderTargets method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-omsetrendertargets)
    fn om_set_render_targets(
        &self,
        render_targets: impl IntoIterator<Item = CpuDescriptorHandle>,
        rts_single_handle_to_descriptor_range: bool,
        depth_stencil: Option<CpuDescriptorHandle>,
    );

    /// Sets the reference value for depth stencil tests.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::OMSetStencilRef method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-omsetstencilref)
    fn om_set_stencil_ref(&self, stencil_ref: u32);

    /// Resets a command list back to its initial state as if a new command list was just created.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::Reset method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-reset)
    fn reset(
        &self,
        command_allocator: &impl ICommandAllocator,
        pso: Option<&impl IPipelineState>,
    ) -> Result<(), DxError>;

    /// Extracts data from a query. ResolveQueryData works with all heap types (default, upload, and readback). ResolveQueryData works with all heap types (default, upload, and readback).
    ///
    /// For more information: [`ID3D12GraphicsCommandList::ResolveQueryData method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-resolvequerydata)
    fn resolve_query_data(
        &self,
        query_heap: &impl IQueryHeap,
        r#type: QueryType,
        start_index: u32,
        num_queries: u32,
        dst_buffer: &impl IResource,
        aligned_dst_buffer_offset: u64,
    );

    /// Copy a multi-sampled resource into a non-multi-sampled resource.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::ResolveSubresource method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-resolvesubresource)
    fn resolve_subresource(
        &self,
        dst_resource: &impl IResource,
        dst_subresource: u32,
        src_resource: &impl IResource,
        src_subresource: u32,
        format: Format,
    );

    /// Notifies the driver that it needs to synchronize multiple accesses to resources.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::ResourceBarrier method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-resourcebarrier)
    fn resource_barrier<'a>(&self, barriers: impl IntoIterator<Item = ResourceBarrier<'a>>);

    /// Binds an array of scissor rectangles to the rasterizer stage.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::RSSetScissorRects method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-rssetscissorrects)
    fn rs_set_scissor_rects(&self, rects: impl IntoIterator<Item = Rect>);

    /// Bind an array of viewports to the rasterizer stage of the pipeline.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::RSSetViewports method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-rssetviewports)
    fn rs_set_viewports(&self, viewport: impl IntoIterator<Item = Viewport>);

    /// Sets a constant in the compute root signature.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SetComputeRoot32BitConstant method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-setcomputeroot32bitconstant)
    fn set_compute_root_32bit_constant(
        &self,
        root_parameter_index: u32,
        src_data: u32,
        dest_offset_in_32bit_values: u32,
    );

    /// Sets a group of constants in the compute root signature.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SetComputeRoot32BitConstants method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-setcomputeroot32bitconstants)
    fn set_compute_root_32bit_constants<T: Copy>(
        &self,
        root_parameter_index: u32,
        src_data: &[T],
        dest_offset_in_32bit_values: u32,
    );

    /// Sets a CPU descriptor handle for the constant buffer in the compute root signature.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SetComputeRootConstantBufferView method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-setcomputerootconstantbufferview)
    fn set_compute_root_constant_buffer_view(
        &self,
        root_parameter_index: u32,
        buffer_location: GpuVirtualAddress,
    );

    /// Sets a descriptor table into the compute root signature.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SetComputeRootDescriptorTable method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12graphicscommandlist)
    fn set_compute_root_descriptor_table(
        &self,
        root_parameter_index: u32,
        base_descriptor: GpuDescriptorHandle,
    );

    /// Sets a CPU descriptor handle for the shader resource in the compute root signature.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SetComputeRootShaderResourceView method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-setcomputerootshaderresourceview)
    fn set_compute_root_shader_resource_view(
        &self,
        root_parameter_index: u32,
        buffer_location: GpuVirtualAddress,
    );

    /// Sets the layout of the compute root signature.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SetComputeRootSignature method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-setcomputerootsignature)
    fn set_compute_root_signature(&self, root_signature: Option<&impl IRootSignature>);

    /// Sets a CPU descriptor handle for the unordered-access-view resource in the compute root signature.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SetComputeRootUnorderedAccessView method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-setcomputerootunorderedaccessview)
    fn set_compute_root_unordered_access_view(
        &self,
        root_parameter_index: u32,
        buffer_location: GpuVirtualAddress,
    );

    /// Changes the currently bound descriptor heaps that are associated with a command list.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SetDescriptorHeaps method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-setdescriptorheaps)
    fn set_descriptor_heaps<'a>(
        &self,
        descriptor_heaps: impl IntoIterator<Item = &'a DescriptorHeap>,
    );

    /// Sets a constant in the graphics root signature.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SetGraphicsRoot32BitConstant method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-setgraphicsroot32bitconstant)
    fn set_graphics_root_32bit_constant(
        &self,
        root_parameter_index: u32,
        src_data: u32,
        dest_offset_in_32bit_values: u32,
    );

    /// Sets a group of constants in the graphics root signature.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SetGraphicsRoot32BitConstants method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-setgraphicsroot32bitconstants)
    fn set_graphics_root_32bit_constants<T: Copy>(
        &self,
        root_parameter_index: u32,
        src_data: &[T],
        dest_offset_in_32bit_values: u32,
    );

    /// Sets a CPU descriptor handle for the constant buffer in the graphics root signature.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SetGraphicsRootConstantBufferView method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-setgraphicsrootconstantbufferview)
    fn set_graphics_root_constant_buffer_view(
        &self,
        root_parameter_index: u32,
        buffer_location: GpuVirtualAddress,
    );

    /// Sets a descriptor table into the graphics root signature.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SetGraphicsRootDescriptorTable method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-setgraphicsrootdescriptortable)
    fn set_graphics_root_descriptor_table(
        &self,
        root_parameter_index: u32,
        base_descriptor: GpuDescriptorHandle,
    );

    /// Sets a CPU descriptor handle for the shader resource in the graphics root signature.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SetGraphicsRootShaderResourceView method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-setgraphicsrootshaderresourceview)
    fn set_graphics_root_shader_resource_view(
        &self,
        root_parameter_index: u32,
        buffer_location: GpuVirtualAddress,
    );

    /// Sets the layout of the graphics root signature.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SetGraphicsRootSignature method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-setgraphicsrootsignature)
    fn set_graphics_root_signature(&self, root_signature: Option<&impl IRootSignature>);

    /// Sets a CPU descriptor handle for the unordered-access-view resource in the graphics root signature.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SetGraphicsRootUnorderedAccessView method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-setgraphicsrootunorderedaccessview)
    fn set_graphics_root_unordered_access_view(
        &self,
        root_parameter_index: u32,
        buffer_location: GpuVirtualAddress,
    );

    /// Inserts a user-defined marker into timeline.
    fn set_marker(&self, color: impl Into<u64>, label: impl AsRef<CStr>);

    /// Sets all shaders and programs most of the fixed-function state of the graphics processing unit (GPU) pipeline.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SetPipelineState method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-setpipelinestate)
    fn set_pipeline_state(&self, pipeline_state: &impl IPipelineState);

    /// Sets a rendering predicate.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SetPredication method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-setpredication)
    fn set_predication(
        &self,
        buffer: Option<&impl IResource>,
        aligned_buffer_offset: u64,
        operation: PredicationOp,
    );

    /// Sets the stream output buffer views.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SOSetTargets method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-sosettargets)
    fn so_set_targets(
        &self,
        start_slot: u32,
        views: Option<impl IntoIterator<Item = StreamOutputBufferView>>,
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

    fn begin_event(&self, color: impl Into<u64>, label: impl AsRef<CStr>) {
        unsafe {
            let color = color.into();
            let label = PCSTR::from_raw(label.as_ref().as_ptr() as *const _);

            (WIN_PIX_EVENT_RUNTIME.begin_event_cmd_list)(std::mem::transmute_copy(&self.0), color, label);
        }
    }

    fn begin_query(&self, query_heap: &impl IQueryHeap, r#type: QueryType, index: u32) {
        unsafe {
            self.0.BeginQuery(
                query_heap.as_raw_ref(),
                r#type.as_raw(),
                index
            )
        }
    }

    fn clear_depth_stencil_view(
        &self,
        depth_stencil_view: CpuDescriptorHandle,
        clear_flags: ClearFlags,
        depth: f32,
        stencil: u8,
        rects: impl IntoIterator<Item = Rect>,
    ) {
        unsafe {
            let rects = rects
                .into_iter()
                .map(|r| r.as_raw())
                .collect::<SmallVec<[_; 8]>>();

            self.0.ClearDepthStencilView(
                depth_stencil_view.as_raw(),
                clear_flags.as_raw(),
                depth,
                stencil,
                &rects
            );
        }
    }

    fn clear_render_target_view(
        &self,
        rtv_handle: CpuDescriptorHandle,
        color: impl Into<[f32; 4]>,
        rects: impl IntoIterator<Item = Rect>,
    ) {
        unsafe {
            let rects = rects
                .into_iter()
                .map(|r| r.as_raw())
                .collect::<SmallVec<[_; 8]>>();

            let rects = if !rects.is_empty() {
                Some(rects.as_slice())
            } else {
                None
            };

            let color = color.into();

            self.0.ClearRenderTargetView(rtv_handle.as_raw(), &color, rects);
        }
    }

    fn clear_state(&self, pipeline_state: Option<&impl IPipelineState>) {
        unsafe {
            if let Some(pipeline_state) = pipeline_state {
                self.0.ClearState(pipeline_state.as_raw_ref());
            } else {
                self.0.ClearState(None);
            }
        }
    }

    fn clear_unordered_access_view_f32(
        &self,
        view_gpu_handle_in_current_heap: GpuDescriptorHandle,
        view_cpu_handle: CpuDescriptorHandle,
        resource: &impl IResource,
        values: impl Into<[f32; 4]>,
        rects: impl IntoIterator<Item = Rect>,
    ) {
        unsafe {
            let rects = rects
                .into_iter()
                .map(|r| r.as_raw())
                .collect::<SmallVec<[_; 8]>>();

            self.0.ClearUnorderedAccessViewFloat(
                view_gpu_handle_in_current_heap.as_raw(),
                view_cpu_handle.as_raw(),
                resource.as_raw_ref(),
                &values.into(),
                &rects
            );
        }
    }

    fn clear_unordered_access_view_u32(
        &self,
        view_gpu_handle_in_current_heap: GpuDescriptorHandle,
        view_cpu_handle: CpuDescriptorHandle,
        resource: &impl IResource,
        values: impl Into<[u32; 4]>,
        rects: impl IntoIterator<Item = Rect>,
    ) {
        unsafe {
            let rects = rects
                .into_iter()
                .map(|r| r.as_raw())
                .collect::<SmallVec<[_; 8]>>();

            self.0.ClearUnorderedAccessViewUint(
                view_gpu_handle_in_current_heap.as_raw(),
                view_cpu_handle.as_raw(),
                resource.as_raw_ref(),
                &values.into(),
                &rects
            );
        }
    }

    fn close(&self) -> Result<(), DxError> {
        unsafe {
            self.0.Close().map_err(DxError::from)
        }
    }

    fn copy_buffer_region(
        &self,
        dst_buffer: &impl IResource,
        dst_offset: u64,
        src_buffer: &impl IResource,
        src_offset: u64,
        num_bytes: u64,
    ) {
        unsafe {
            self.0.CopyBufferRegion(
                dst_buffer.as_raw_ref(),
                dst_offset,
                src_buffer.as_raw_ref(),
                src_offset,
                num_bytes
            );
        }
    }

    fn copy_resource(&self, dst_resource: &impl IResource, src_resource: &impl IResource) {
        unsafe {
            self.0.CopyResource(
                dst_resource.as_raw_ref(),
                src_resource.as_raw_ref(),
            );
        }
    }

    fn copy_texture_region(
        &self,
        dst: &TextureCopyLocation<'_>,
        dst_x: u32,
        dst_y: u32,
        dst_z: u32,
        src: &TextureCopyLocation<'_>,
        src_box: Option<&Box>,
    ) {
        unsafe {
            let src_box = src_box.map(|s| &s.0 as *const _);

            self.0.CopyTextureRegion(
                &dst.as_raw(),
                dst_x,
                dst_y,
                dst_z,
                &src.as_raw(),
                src_box,
            );
        }
    }

    fn copy_tiles(
        &self,
        tiled_resource: &impl IResource,
        tile_region_start_coordinate: &TiledResourceCoordinate,
        tile_region_size: &TileRegionSize,
        buffer: &impl IResource,
        buffer_start_offset: u64,
        flags: TileCopyFlags,
    ) {
        unsafe {
            self.0.CopyTiles(
                tiled_resource.as_raw_ref(),
                &tile_region_start_coordinate.as_raw(),
                &tile_region_size.as_raw(),
                buffer.as_raw_ref(),
                buffer_start_offset,
                flags.as_raw(),
            );
        }
    }

    fn discard_resource(&self, resource: &impl IResource, region: Option<&DiscardRegion<'_>>) {
        unsafe {
            let rects = if let Some(region) = region {
                region
                    .rects
                    .iter()
                    .map(|r| r.as_raw())
                    .collect::<SmallVec<[_; 16]>>()
            } else {
                SmallVec::new()
            };

            let region = region.map(|r| r.as_raw(&rects));
            let region = region.as_ref().map(|r| r as *const _);

            self.0.DiscardResource(resource.as_raw_ref(), region);
        }
    }

    fn dispatch(
        &self,
        thread_group_count_x: u32,
        thread_group_count_y: u32,
        thread_group_count_z: u32,
    ) {
        unsafe {
            self.0.Dispatch(
                thread_group_count_x,
                thread_group_count_y,
                thread_group_count_z
            )
        }
    }

    fn draw_indexed_instanced(
        &self,
        index_count_per_instance: u32,
        instance_count: u32,
        start_index_location: u32,
        base_vertex_location: i32,
        start_instance_location: u32,
    ) {
        unsafe {
            self.0.DrawIndexedInstanced(
                index_count_per_instance,
                instance_count,
                start_index_location,
                base_vertex_location,
                start_instance_location,
            );
        }
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
            );
        }
    }

    fn end_event(&self) {
        unsafe {
            (WIN_PIX_EVENT_RUNTIME.end_event_cmd_list)(std::mem::transmute_copy(&self.0));
        }
    }

    fn end_query(&self, query_heap: &impl IQueryHeap, r#type: QueryType, index: u32) {
        unsafe {
            self.0.EndQuery(
                query_heap.as_raw_ref(),
                r#type.as_raw(),
                index
            )
        }
    }

    fn execute_bundle(&self, command_list: &impl IGraphicsCommandList) {
        unsafe {
            self.0.ExecuteBundle(command_list.as_raw_ref());
        }
    }

    fn execute_indirect(
        &self,
        command_signature: &impl ICommandSignature,
        max_command_count: u32,
        argument_buffer: impl IResource,
        argument_buffer_offset: u64,
        count_buffer: Option<&impl IResource>,
        count_buffer_offset: u64,
    ) {
        unsafe {
            if let Some(count_buffer) = count_buffer {
                self.0.ExecuteIndirect(
                    command_signature.as_raw_ref(),
                    max_command_count,
                    argument_buffer.as_raw_ref(),
                    argument_buffer_offset,
                    count_buffer.as_raw_ref(),
                    count_buffer_offset
                );
            } else {
                self.0.ExecuteIndirect(
                    command_signature.as_raw_ref(),
                    max_command_count,
                    argument_buffer.as_raw_ref(),
                    argument_buffer_offset,
                    None,
                    count_buffer_offset
                );
            }
        }
    }

    fn ia_set_index_buffer(&self, view: Option<&IndexBufferView>) {
        unsafe {
            let view = view.map(|view| view.as_raw());
            let view = view.as_ref().map(|view| view as *const _);

            self.0.IASetIndexBuffer(view);
        }
    }

    fn ia_set_primitive_topology(&self, topology: PrimitiveTopology) {
        unsafe {
            self.0.IASetPrimitiveTopology(topology.as_raw());
        }
    }

    fn ia_set_vertex_buffers(
        &self,
        slot: u32,
        buffers: impl IntoIterator<Item = VertexBufferView>,
    ) {
        unsafe {
            let buffers = buffers
                .into_iter()
                .map(|r| r.as_raw())
                .collect::<SmallVec<[_; 8]>>();

            let buffers = if !buffers.is_empty() {
                Some(buffers.as_slice())
            } else {
                None
            };

            self.0.IASetVertexBuffers(slot, buffers);
        }
    }

    fn om_set_blend_factor(&self, blend_factor: Option<[f32; 4]>) {
        unsafe {
            self.0.OMSetBlendFactor(blend_factor.as_ref());
        }
    }

    fn om_set_render_targets(
        &self,
        render_targets: impl IntoIterator<Item = CpuDescriptorHandle>,
        rts_single_handle_to_descriptor_range: bool,
        depth_stencil: Option<CpuDescriptorHandle>,
    ) {
        unsafe {
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

            self.0.OMSetRenderTargets(
                render_targets.len() as u32,
                render_targets_raw,
                BOOL(rts_single_handle_to_descriptor_range as i32),
                depth_stencil,
            );
        }
    }

    fn om_set_stencil_ref(&self, stencil_ref: u32) {
        unsafe {
            self.0.OMSetStencilRef(stencil_ref);
        }
    }

    fn reset(
        &self,
        command_allocator: &impl ICommandAllocator,
        pso: Option<&impl IPipelineState>,
    ) -> Result<(), DxError> {
        unsafe {
            if let Some(pso) = pso {
                self.0.Reset(
                        command_allocator.as_raw_ref(),
                        pso.as_raw_ref()
                    )
                    .map_err(DxError::from)
            } else {
                self.0.Reset(
                        command_allocator.as_raw_ref(),
                        None
                    )
                    .map_err(DxError::from)
            }
        }
    }

    fn resolve_query_data(
        &self,
        query_heap: &impl IQueryHeap,
        r#type: QueryType,
        start_index: u32,
        num_queries: u32,
        dst_buffer: &impl IResource,
        aligned_dst_buffer_offset: u64,
    ) {
        unsafe {
            self.0.ResolveQueryData(
                query_heap.as_raw_ref(),
                r#type.as_raw(),
                start_index,
                num_queries,
                dst_buffer.as_raw_ref(),
                aligned_dst_buffer_offset
            );
        }
    }

    fn resolve_subresource(
        &self,
        dst_resource: &impl IResource,
        dst_subresource: u32,
        src_resource: &impl IResource,
        src_subresource: u32,
        format: Format,
    ) {
        unsafe {
            self.0.ResolveSubresource(
                dst_resource.as_raw_ref(),
                dst_subresource,
                src_resource.as_raw_ref(),
                src_subresource,
                format.as_raw()
            );
        }
    }

    fn resource_barrier<'a>(&self, barriers: impl IntoIterator<Item = ResourceBarrier<'a>>) {
        unsafe {
            let barriers = barriers
                .into_iter()
                .map(|r| r.as_raw())
                .collect::<SmallVec<[_; 8]>>();

            self.0.ResourceBarrier(barriers.as_slice());
        }
    }

    fn rs_set_scissor_rects(&self, rects: impl IntoIterator<Item = Rect>) {
        unsafe {
            let rects = rects
                .into_iter()
                .map(|v| v.as_raw())
                .collect::<SmallVec<[_; 8]>>();

            self.0.RSSetScissorRects(&rects);
        }
    }

    fn rs_set_viewports(&self, viewport: impl IntoIterator<Item = Viewport>) {
        unsafe {
            let viewports = viewport
                .into_iter()
                .map(|v| v.as_raw())
                .collect::<SmallVec<[_; 8]>>();

            self.0.RSSetViewports(&viewports);
        }
    }

    fn set_compute_root_32bit_constant(
        &self,
        root_parameter_index: u32,
        src_data: u32,
        dest_offset_in_32bit_values: u32,
    ) {
        unsafe {
            self.0.SetComputeRoot32BitConstant(
                root_parameter_index,
                src_data,
                dest_offset_in_32bit_values
            );
        }
    }

    fn set_compute_root_32bit_constants<T: Copy>(
        &self,
        root_parameter_index: u32,
        src_data: &[T],
        dest_offset_in_32bit_values: u32,
    ) {
        unsafe {
            self.0.SetComputeRoot32BitConstants(
                root_parameter_index,
                src_data.len() as u32,
                src_data.as_ptr() as *const _,
                dest_offset_in_32bit_values
            );
        }
    }

    fn set_compute_root_constant_buffer_view(
        &self,
        root_parameter_index: u32,
        buffer_location: GpuVirtualAddress,
    ) {
        unsafe {
            self.0.SetComputeRootConstantBufferView(
                root_parameter_index,
                buffer_location
            );
        }
    }

    fn set_compute_root_descriptor_table(
        &self,
        root_parameter_index: u32,
        base_descriptor: GpuDescriptorHandle,
    ) {
        unsafe {
            self.0.SetComputeRootDescriptorTable(
                root_parameter_index,
                base_descriptor.as_raw()
            );
        }
    }

    fn set_compute_root_shader_resource_view(
        &self,
        root_parameter_index: u32,
        buffer_location: GpuVirtualAddress,
    ) {
        unsafe {
            self.0.SetComputeRootShaderResourceView(
                root_parameter_index,
                buffer_location
            );
        }
    }

    fn set_compute_root_signature(&self, root_signature: Option<&impl IRootSignature>) {
        unsafe {
            if let Some(root_signature) = root_signature {
                self.0.SetComputeRootSignature(root_signature.as_raw_ref());
            } else {
                self.0.SetComputeRootSignature(None);
            }
        }
    }

    fn set_compute_root_unordered_access_view(
        &self,
        root_parameter_index: u32,
        buffer_location: GpuVirtualAddress,
    ) {
        unsafe {
            self.0.SetComputeRootUnorderedAccessView(
                root_parameter_index,
                buffer_location
            );
        }
    }

    fn set_descriptor_heaps<'a>(
        &self,
        descriptor_heaps: impl IntoIterator<Item = &'a DescriptorHeap>,
    ) {
        unsafe {
            let descriptor_heaps = descriptor_heaps
                .into_iter()
                .map(|dh| Some(dh.as_raw().clone()))
                .collect::<SmallVec<[_; 16]>>();

            self.0.SetDescriptorHeaps(
                descriptor_heaps.as_slice()
            );
        }
    }

    fn set_graphics_root_32bit_constant(
        &self,
        root_parameter_index: u32,
        src_data: u32,
        dest_offset_in_32bit_values: u32,
    ) {
        unsafe {
            self.0.SetGraphicsRoot32BitConstant(
                root_parameter_index,
                src_data,
                dest_offset_in_32bit_values
            );
        }
    }

    fn set_graphics_root_32bit_constants<T: Copy>(
        &self,
        root_parameter_index: u32,
        src_data: &[T],
        dest_offset_in_32bit_values: u32,
    ) {
        unsafe {
            self.0.SetGraphicsRoot32BitConstants(
                root_parameter_index,
                src_data.len() as u32,
                src_data.as_ptr() as *const _,
                dest_offset_in_32bit_values
            );
        }
    }

    fn set_graphics_root_constant_buffer_view(
        &self,
        root_parameter_index: u32,
        buffer_location: GpuVirtualAddress,
    ) {
        unsafe {
            self.0.SetGraphicsRootConstantBufferView(
                root_parameter_index,
                buffer_location,
            );
        }
    }

    fn set_graphics_root_descriptor_table(
        &self,
        root_parameter_index: u32,
        base_descriptor: GpuDescriptorHandle,
    ) {
        unsafe {
            self.0.SetGraphicsRootDescriptorTable(
                root_parameter_index,
                base_descriptor.as_raw(),
            );
        }
    }

    fn set_graphics_root_shader_resource_view(
        &self,
        root_parameter_index: u32,
        buffer_location: GpuVirtualAddress,
    ) {
        unsafe {
            self.0.SetGraphicsRootShaderResourceView(
                root_parameter_index,
                buffer_location,
            );
        }
    }

    fn set_graphics_root_signature(&self, root_signature: Option<&impl IRootSignature>) {
        unsafe {
            if let Some(root_signature) = root_signature {
                self.0.SetGraphicsRootSignature(root_signature.as_raw_ref());
            } else {
                self.0.SetGraphicsRootSignature(None);
            }
        }
    }

    fn set_graphics_root_unordered_access_view(
        &self,
        root_parameter_index: u32,
        buffer_location: GpuVirtualAddress,
    ) {
        unsafe {
            self.0.SetGraphicsRootUnorderedAccessView(
                root_parameter_index,
                buffer_location,
            );
        }
    }

    fn set_marker(&self, color: impl Into<u64>, label: impl AsRef<CStr>) {
        unsafe {
            let color = color.into();
            let label = PCSTR::from_raw(label.as_ref().as_ptr() as *const _);

            (WIN_PIX_EVENT_RUNTIME.set_marker_cmd_list)(std::mem::transmute_copy(&self.0), color, label);
        }
    }

    fn set_pipeline_state(&self, pipeline_state: &impl IPipelineState) {
        unsafe {
            self.0.SetPipelineState(pipeline_state.as_raw_ref());
        }
    }

    fn set_predication(
        &self,
        buffer: Option<&impl IResource>,
        aligned_buffer_offset: u64,
        operation: PredicationOp,
    ) {
        unsafe {
            if let Some(buffer) = buffer {
                self.0.SetPredication(
                    buffer.as_raw_ref(),
                    aligned_buffer_offset,
                    operation.as_raw()
                );
            } else {
                self.0.SetPredication(
                    None,
                    aligned_buffer_offset,
                    operation.as_raw()
                )
            }
        }
    }

    fn so_set_targets(
        &self,
        start_slot: u32,
        views: Option<impl IntoIterator<Item = StreamOutputBufferView>>,
    ) {
        unsafe {
            let views = views
                .map(|views|
                    views
                    .into_iter()
                    .map(|view| view.as_raw())
                    .collect::<SmallVec<[_; 16]>>()
                );

            self.0.SOSetTargets(
                start_slot,
                views.as_ref().map(|v| v.as_slice())
            )
        }
    }
}
