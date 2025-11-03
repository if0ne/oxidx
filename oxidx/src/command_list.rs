use std::ops::Range;

use windows::Win32::Graphics::Direct3D12::*;

use crate::{
    create_type,
    descriptor_heap::DescriptorHeap,
    dx::{
        CommandAllocator, CommandSignature, Device, PipelineState, QueryHeap, Resource,
        RootSignature,
    },
    error::DxError,
    ext::memcpy_subresource,
    impl_interface,
    types::*,
};

create_type! { GraphicsCommandList wrap ID3D12GraphicsCommandList }

impl_interface! {
    GraphicsCommandList;
    /// Gets the type of the command list, such as direct, bundle, compute, or copy.
    ///
    /// For more information: [`ID3D12CommandList::GetType method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12commandlist-gettype)
    pub fn get_type(&self) -> CommandListType {
        unsafe {
            self.0.GetType().into()
        }
    }
}

impl_interface! {
    GraphicsCommandList;

    /// Marks the start of a user-defined region of work.
    #[cfg(feature = "pix")]
    pub fn begin_event(&self, color: impl Into<u64>, label: impl AsRef<std::ffi::CStr>) {
        unsafe {
            let color = color.into();
            let label = windows::core::PCSTR::from_raw(label.as_ref().as_ptr() as *const _);

            (crate::pix::WIN_PIX_EVENT_RUNTIME.begin_event_cmd_list)(std::mem::transmute_copy(&self.0), color, label);
        }
    }

    /// Starts a query running.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::BeginQuery method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-beginquery)
    pub fn begin_query(&self, query_heap: impl AsRef<QueryHeap>, r#type: QueryType, index: u32) {
        unsafe {
            self.0.BeginQuery(
                &query_heap.as_ref().0,
                r#type.as_raw(),
                index
            )
        }
    }

    /// Clears the depth-stencil resource.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::ClearDepthStencilView method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-cleardepthstencilview)
    pub fn clear_depth_stencil_view(
        &self,
        depth_stencil_view: CpuDescriptorHandle,
        clear_flags: ClearFlags,
        depth: f32,
        stencil: u8,
        rects: Option<&[Rect]>,
    ) {
        unsafe {
            let rects = rects.map(|rects| std::slice::from_raw_parts(rects.as_ptr() as *const _, rects.len()));

            self.0.ClearDepthStencilView(
                depth_stencil_view.0,
                clear_flags.as_raw(),
                depth,
                stencil,
                rects
            );
        }
    }

    /// Sets all the elements in a render target to one value.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::ClearRenderTargetView method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-clearrendertargetview)
    pub fn clear_render_target_view(
        &self,
        rtv_handle: CpuDescriptorHandle,
        color: impl Into<[f32; 4]>,
        rects: &[Rect],
    ) {
        unsafe {
            let rects = std::slice::from_raw_parts(rects.as_ptr() as *const _, rects.len());

            let rects = if !rects.is_empty() {
                Some(rects)
            } else {
                None
            };

            let color = color.into();

            self.0.ClearRenderTargetView(rtv_handle.0, &color, rects);
        }
    }

    /// Resets the state of a direct command list back to the state it was in when the command list was created.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::ClearState method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-clearstate)
    pub fn clear_state<'a>(&self, pso: impl Into<Option<&'a PipelineState>>,) {
        unsafe {
            if let Some(pipeline_state) = pso.into() {
                self.0.ClearState(&pipeline_state.0);
            } else {
                self.0.ClearState(None);
            }
        }
    }

    /// Sets all of the elements in an unordered-access view (UAV) to the specified f32 values.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::ClearUnorderedAccessViewFloat method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-clearunorderedaccessviewfloat)
    pub fn clear_unordered_access_view_f32(
        &self,
        view_gpu_handle_in_current_heap: GpuDescriptorHandle,
        view_cpu_handle: CpuDescriptorHandle,
        resource: impl AsRef<Resource>,
        values: impl Into<[f32; 4]>,
        rects: &[Rect],
    ) {
        unsafe {
            let rects = std::slice::from_raw_parts(rects.as_ptr() as *const _, rects.len());

            self.0.ClearUnorderedAccessViewFloat(
                view_gpu_handle_in_current_heap.0,
                view_cpu_handle.0,
                &resource.as_ref().0,
                &values.into(),
                rects
            );
        }
    }

    /// Sets all of the elements in an unordered-access view (UAV) to the specified u32 values.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::ClearUnorderedAccessViewUint method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-clearunorderedaccessviewuint)
    pub fn clear_unordered_access_view_u32(
        &self,
        view_gpu_handle_in_current_heap: GpuDescriptorHandle,
        view_cpu_handle: CpuDescriptorHandle,
        resource: impl AsRef<Resource>,
        values: impl Into<[u32; 4]>,
        rects: &[Rect],
    ) {
        unsafe {
            let rects = std::slice::from_raw_parts(rects.as_ptr() as *const _, rects.len());

            self.0.ClearUnorderedAccessViewUint(
                view_gpu_handle_in_current_heap.0,
                view_cpu_handle.0,
                &resource.as_ref().0,
                &values.into(),
                rects
            );
        }
    }

    /// Indicates that recording to the command list has finished.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::Close method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-close)
    pub fn close(&self) -> Result<(), DxError> {
        unsafe {
            self.0.Close().map_err(DxError::from)
        }
    }

    /// Copies a region of a buffer from one resource to another.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::CopyBufferRegion method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-copybufferregion)
    pub fn copy_buffer_region(
        &self,
        dst_buffer: impl AsRef<Resource>,
        dst_offset: u64,
        src_buffer: impl AsRef<Resource>,
        src_offset: u64,
        num_bytes: u64,
    ) {
        unsafe {
            self.0.CopyBufferRegion(
                &dst_buffer.as_ref().0,
                dst_offset,
                &src_buffer.as_ref().0,
                src_offset,
                num_bytes
            );
        }
    }

    /// Copies the entire contents of the source resource to the destination resource.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::CopyResource method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-copyresource)
    pub fn copy_resource(&self, dst_resource: impl AsRef<Resource>, src_resource: impl AsRef<Resource>) {
        unsafe {
            self.0.CopyResource(
                &dst_resource.as_ref().0,
                &src_resource.as_ref().0,
            );
        }
    }

    /// This method uses the GPU to copy texture data between two locations. Both the source and the destination may reference texture data located within either a buffer resource or a texture resource.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::CopyTextureRegion method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-copytextureregion)
    pub fn copy_texture_region(
        &self,
        dst: &TextureCopyLocation<'_>,
        dst_x: u32,
        dst_y: u32,
        dst_z: u32,
        src: &TextureCopyLocation<'_>,
        src_box: Option<&DxBox>,
    ) {
        unsafe {
            let src_box = src_box.map(|s| &s.0 as *const _);

            self.0.CopyTextureRegion(
                &dst.0,
                dst_x,
                dst_y,
                dst_z,
                &src.0,
                src_box,
            );
        }
    }

    /// Copies tiles from buffer to tiled resource or vice versa.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::CopyTiles method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-copytiles)
    pub fn copy_tiles(
        &self,
        tiled_resource: impl AsRef<Resource>,
        tile_region_start_coordinate: &TiledResourceCoordinate,
        tile_region_size: &TileRegionSize,
        buffer: impl AsRef<Resource>,
        buffer_start_offset: u64,
        flags: TileCopyFlags,
    ) {
        unsafe {
            self.0.CopyTiles(
                &tiled_resource.as_ref().0,
                &tile_region_start_coordinate.0,
                &tile_region_size.0,
                &buffer.as_ref().0,
                buffer_start_offset,
                flags.as_raw(),
            );
        }
    }

    /// Discards a resource.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::DiscardResource method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-discardresource)
    pub fn discard_resource(&self, resource: impl AsRef<Resource>, region: Option<&DiscardRegion<'_>>) {
        unsafe {
            let region = region.map(|r| &r.0 as *const _);

            self.0.DiscardResource(&resource.as_ref().0, region);
        }
    }

    /// Executes a compute shader on a thread group.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::Dispatch method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-dispatch)
    pub fn dispatch(
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

    /// Draws indexed, instanced primitives.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::DrawIndexedInstanced method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-drawindexedinstanced)
    pub fn draw_indexed_instanced(
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

    /// Draws non-indexed, instanced primitives.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::DrawInstanced method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-drawinstanced)
    pub fn draw_instanced(
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

    /// Marks the end of a user-defined region of work.
    #[cfg(feature = "pix")]
    pub fn end_event(&self) {
        unsafe {
            (crate::pix::WIN_PIX_EVENT_RUNTIME.end_event_cmd_list)(std::mem::transmute_copy(&self.0));
        }
    }

    /// Ends a running query.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::EndQuery method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-endquery)
    pub fn end_query(&self, query_heap: impl AsRef<QueryHeap>, r#type: QueryType, index: u32) {
        unsafe {
            self.0.EndQuery(
                &query_heap.as_ref().0,
                r#type.as_raw(),
                index
            )
        }
    }

    /// Executes a bundle.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::ExecuteBundle method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-executebundle)
    pub fn execute_bundle(&self, command_list: impl AsRef<GraphicsCommandList>) {
        unsafe {
            self.0.ExecuteBundle(&command_list.as_ref().0);
        }
    }

    /// Apps perform indirect draws/dispatches using the ExecuteIndirect method.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::ExecuteIndirect method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-executeindirect)
    pub fn execute_indirect<'a>(
        &self,
        command_signature: impl AsRef<CommandSignature>,
        max_command_count: u32,
        argument_buffer: impl AsRef<Resource>,
        argument_buffer_offset: u64,
        count_buffer: impl Into<Option<&'a Resource>>,
        count_buffer_offset: u64,
    ) {
        unsafe {
            if let Some(count_buffer) = count_buffer.into() {
                self.0.ExecuteIndirect(
                    &command_signature.as_ref().0,
                    max_command_count,
                    &argument_buffer.as_ref().0,
                    argument_buffer_offset,
                    &count_buffer.0,
                    count_buffer_offset
                );
            } else {
                self.0.ExecuteIndirect(
                    &command_signature.as_ref().0,
                    max_command_count,
                    &argument_buffer.as_ref().0,
                    argument_buffer_offset,
                    None,
                    count_buffer_offset
                );
            }
        }
    }

    /// Sets the view for the index buffer.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::IASetIndexBuffer method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-iasetindexbuffer)
    pub fn ia_set_index_buffer(&self, view: Option<&IndexBufferView>) {
        unsafe {
            let view = view.map(|view| &view.0 as *const _);

            self.0.IASetIndexBuffer(view);
        }
    }

    /// Bind information about the primitive type, and data order that describes input data for the input assembler stage.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::IASetPrimitiveTopology method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-iasetprimitivetopology)
    pub fn ia_set_primitive_topology(&self, topology: PrimitiveTopology) {
        unsafe {
            self.0.IASetPrimitiveTopology(topology.as_raw());
        }
    }

    /// Sets a CPU descriptor handle for the vertex buffers.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::IASetVertexBuffers method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-iasetvertexbuffers)
    pub fn ia_set_vertex_buffers(
        &self,
        slot: u32,
        buffers: &[VertexBufferView],
    ) {
        unsafe {
            let buffers = std::slice::from_raw_parts(
                buffers.as_ptr() as *const _,
                buffers.len()
            );

            let buffers = if !buffers.is_empty() {
                Some(buffers)
            } else {
                None
            };

            self.0.IASetVertexBuffers(slot, buffers);
        }
    }

    /// Sets the blend factor that modulate values for a pixel shader, render target, or both.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::OMSetBlendFactor method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-omsetblendfactor)
    pub fn om_set_blend_factor(&self, blend_factor: Option<[f32; 4]>) {
        unsafe {
            self.0.OMSetBlendFactor(blend_factor.as_ref());
        }
    }

    /// Sets CPU descriptor handles for the render targets and depth stencil.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::OMSetRenderTargets method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-omsetrendertargets)
    pub fn om_set_render_targets(
        &self,
        render_targets: &[CpuDescriptorHandle],
        rts_single_handle_to_descriptor_range: bool,
        depth_stencil: Option<CpuDescriptorHandle>,
    ) {
        unsafe {
            let render_targets_raw = if !render_targets.is_empty() {
                Some(render_targets.as_ptr() as *const _)
            } else {
                None
            };

            let depth_stencil = depth_stencil.map(|ds| ds.0);
            let depth_stencil = depth_stencil.as_ref().map(|ds| ds as *const _);

            self.0.OMSetRenderTargets(
                render_targets.len() as u32,
                render_targets_raw,
                rts_single_handle_to_descriptor_range,
                depth_stencil,
            );
        }
    }

    /// Sets the reference value for depth stencil tests.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::OMSetStencilRef method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-omsetstencilref)
    pub fn om_set_stencil_ref(&self, stencil_ref: u32) {
        unsafe {
            self.0.OMSetStencilRef(stencil_ref);
        }
    }

    /// Resets a command list back to its initial state as if a new command list was just created.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::Reset method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-reset)
    pub fn reset<'a>(
        &self,
        command_allocator: &impl AsRef<CommandAllocator>,
        pso: impl Into<Option<&'a PipelineState>>,
    ) -> Result<(), DxError> {
        unsafe {
            if let Some(pso) = pso.into() {
                self.0.Reset(
                    &command_allocator.as_ref().0,
                    &pso.0
                )
                .map_err(DxError::from)
            } else {
                self.0.Reset(
                     &command_allocator.as_ref().0,
                    None
                )
                .map_err(DxError::from)
            }
        }
    }

    /// Extracts data from a query. ResolveQueryData works with all heap types (default, upload, and readback). ResolveQueryData works with all heap types (default, upload, and readback).
    ///
    /// For more information: [`ID3D12GraphicsCommandList::ResolveQueryData method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-resolvequerydata)
    pub fn resolve_query_data(
        &self,
        query_heap: impl AsRef<QueryHeap>,
        r#type: QueryType,
        range: Range<u32>,
        dst_buffer: impl AsRef<Resource>,
        aligned_dst_buffer_offset: u64,
    ) {
        unsafe {
            self.0.ResolveQueryData(
                &query_heap.as_ref().0,
                r#type.as_raw(),
                range.start,
                range.count() as u32,
                &dst_buffer.as_ref().0,
                aligned_dst_buffer_offset
            );
        }
    }

    /// Copy a multi-sampled resource into a non-multi-sampled resource.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::ResolveSubresource method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-resolvesubresource)
    pub fn resolve_subresource(
        &self,
        dst_resource: impl AsRef<Resource>,
        dst_subresource: u32,
        src_resource: impl AsRef<Resource>,
        src_subresource: u32,
        format: Format,
    ) {
        unsafe {
            self.0.ResolveSubresource(
                &dst_resource.as_ref().0,
                dst_subresource,
                &src_resource.as_ref().0,
                src_subresource,
                format.as_raw()
            );
        }
    }

    /// Notifies the driver that it needs to synchronize multiple accesses to resources.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::ResourceBarrier method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-resourcebarrier)
    pub fn resource_barrier(&self, barriers: &[ResourceBarrier<'_>]) {
        unsafe {
            let barriers = std::slice::from_raw_parts(
                barriers.as_ptr() as *const _,
                barriers.len()
            );
            self.0.ResourceBarrier(barriers);
        }
    }

    /// Binds an array of scissor rectangles to the rasterizer stage.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::RSSetScissorRects method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-rssetscissorrects)
    pub fn rs_set_scissor_rects(&self, rects: &[Rect]) {
        unsafe {
            let rects = std::slice::from_raw_parts(rects.as_ptr() as *const _, rects.len());

            self.0.RSSetScissorRects(rects);
        }
    }

    /// Bind an array of viewports to the rasterizer stage of the pipeline.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::RSSetViewports method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-rssetviewports)
    pub fn rs_set_viewports(&self, viewport: &[Viewport]) {
        unsafe {
            let viewports = std::slice::from_raw_parts(
                viewport.as_ptr() as *const _,
                viewport.len()
            );

            self.0.RSSetViewports(viewports);
        }
    }

    /// Sets a constant in the compute root signature.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SetComputeRoot32BitConstant method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-setcomputeroot32bitconstant)
    pub fn set_compute_root_32bit_constant<T: Copy>(
        &self,
        root_parameter_index: u32,
        src_data: T,
        dest_offset_in_32bit_values: u32,
    ) {
        const { assert!(size_of::<T>() == 4) }

        let bits = unsafe {
            std::mem::transmute_copy(&src_data)
        };

        unsafe {
            self.0.SetComputeRoot32BitConstant(
                root_parameter_index,
                bits,
                dest_offset_in_32bit_values
            );
        }
    }

    /// Sets a group of constants in the compute root signature.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SetComputeRoot32BitConstants method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-setcomputeroot32bitconstants)
    pub fn set_compute_root_32bit_constants<T: Copy>(
        &self,
        root_parameter_index: u32,
        src_data: &[T],
        dest_offset_in_32bit_values: u32,
    ) {
        const { assert!(size_of::<T>() == 4) }

        unsafe {
            self.0.SetComputeRoot32BitConstants(
                root_parameter_index,
                src_data.len() as u32,
                src_data.as_ptr() as *const _,
                dest_offset_in_32bit_values
            );
        }
    }

    /// Sets a CPU descriptor handle for the constant buffer in the compute root signature.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SetComputeRootConstantBufferView method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-setcomputerootconstantbufferview)
    pub fn set_compute_root_constant_buffer_view(
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

    /// Sets a descriptor table into the compute root signature.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SetComputeRootDescriptorTable method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12graphicscommandlist)
    pub fn set_compute_root_descriptor_table(
        &self,
        root_parameter_index: u32,
        base_descriptor: GpuDescriptorHandle,
    ) {
        unsafe {
            self.0.SetComputeRootDescriptorTable(
                root_parameter_index,
                base_descriptor.0
            );
        }
    }

    /// Sets a CPU descriptor handle for the shader resource in the compute root signature.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SetComputeRootShaderResourceView method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-setcomputerootshaderresourceview)
    pub fn set_compute_root_shader_resource_view(
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

    /// Sets the layout of the compute root signature.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SetComputeRootSignature method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-setcomputerootsignature)
    pub fn set_compute_root_signature<'a>(&self, root_signature: impl Into<Option<&'a RootSignature>>) {
        unsafe {
            if let Some(root_signature) = root_signature.into() {
                self.0.SetComputeRootSignature(&root_signature.0);
            } else {
                self.0.SetComputeRootSignature(None);
            }
        }
    }

    /// Sets a CPU descriptor handle for the unordered-access-view resource in the compute root signature.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SetComputeRootUnorderedAccessView method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-setcomputerootunorderedaccessview)
    pub fn set_compute_root_unordered_access_view(
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

    /// Changes the currently bound descriptor heaps that are associated with a command list.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SetDescriptorHeaps method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-setdescriptorheaps)
    pub fn set_descriptor_heaps(
        &self,
        descriptor_heaps: &[Option<DescriptorHeap>],
    ) {
        unsafe {
            let descriptor_heaps = std::slice::from_raw_parts(descriptor_heaps.as_ptr() as *const _, descriptor_heaps.len());

            self.0.SetDescriptorHeaps(
                descriptor_heaps
            );
        }
    }

    /// Sets a constant in the graphics root signature.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SetGraphicsRoot32BitConstant method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-setgraphicsroot32bitconstant)
    pub fn set_graphics_root_32bit_constant<T: Copy>(
        &self,
        root_parameter_index: u32,
        src_data: T,
        dest_offset_in_32bit_values: u32,
    ) {
        const { assert!(size_of::<T>() == 4) }

        let bits = unsafe {
            std::mem::transmute_copy(&src_data)
        };

        unsafe {
            self.0.SetGraphicsRoot32BitConstant(
                root_parameter_index,
                bits,
                dest_offset_in_32bit_values
            );
        }
    }

    /// Sets a group of constants in the graphics root signature.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SetGraphicsRoot32BitConstants method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-setgraphicsroot32bitconstants)
    pub fn set_graphics_root_32bit_constants<T: Copy>(
        &self,
        root_parameter_index: u32,
        src_data: &[T],
        dest_offset_in_32bit_values: u32,
    ) {
        const { assert!(size_of::<T>() == 4) }

        unsafe {
            self.0.SetGraphicsRoot32BitConstants(
                root_parameter_index,
                src_data.len() as u32,
                src_data.as_ptr() as *const _,
                dest_offset_in_32bit_values
            );
        }
    }

    /// Sets a CPU descriptor handle for the constant buffer in the graphics root signature.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SetGraphicsRootConstantBufferView method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-setgraphicsrootconstantbufferview)
    pub fn set_graphics_root_constant_buffer_view(
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

    /// Sets a descriptor table into the graphics root signature.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SetGraphicsRootDescriptorTable method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-setgraphicsrootdescriptortable)
    pub fn set_graphics_root_descriptor_table(
        &self,
        root_parameter_index: u32,
        base_descriptor: GpuDescriptorHandle,
    ) {
        unsafe {
            self.0.SetGraphicsRootDescriptorTable(
                root_parameter_index,
                base_descriptor.0,
            );
        }
    }

    /// Sets a CPU descriptor handle for the shader resource in the graphics root signature.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SetGraphicsRootShaderResourceView method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-setgraphicsrootshaderresourceview)
    pub fn set_graphics_root_shader_resource_view(
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

    /// Sets the layout of the graphics root signature.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SetGraphicsRootSignature method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-setgraphicsrootsignature)
    pub fn set_graphics_root_signature<'a>(&self, root_signature: impl Into<Option<&'a RootSignature>>) {
        unsafe {
            if let Some(root_signature) = root_signature.into() {
                self.0.SetGraphicsRootSignature(&root_signature.0);
            } else {
                self.0.SetGraphicsRootSignature(None);
            }
        }
    }

    /// Sets a CPU descriptor handle for the unordered-access-view resource in the graphics root signature.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SetGraphicsRootUnorderedAccessView method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-setgraphicsrootunorderedaccessview)
    pub fn set_graphics_root_unordered_access_view(
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

    /// Inserts a user-defined marker into timeline.
    #[cfg(feature = "pix")]
    pub fn set_marker(&self, color: impl Into<u64>, label: impl AsRef<std::ffi::CStr>) {
        unsafe {
            let color = color.into();
            let label = windows::core::PCSTR::from_raw(label.as_ref().as_ptr() as *const _);

            (crate::pix::WIN_PIX_EVENT_RUNTIME.set_marker_cmd_list)(std::mem::transmute_copy(&self.0), color, label);
        }
    }

    /// Sets all shaders and programs most of the fixed-function state of the graphics processing unit (GPU) pipeline.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SetPipelineState method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-setpipelinestate)
    pub fn set_pipeline_state(&self, pso: impl AsRef<PipelineState>) {
        unsafe {
            self.0.SetPipelineState(&pso.as_ref().0);
        }
    }

    /// Sets a rendering predicate.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SetPredication method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-setpredication)
    pub fn set_predication<'a>(
        &self,
        buffer: impl Into<Option<&'a Resource>>,
        aligned_buffer_offset: u64,
        operation: PredicationOp,
    ) {
        unsafe {
            if let Some(buffer) = buffer.into() {
                self.0.SetPredication(
                    &buffer.0,
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

    /// Sets the stream output buffer views.
    ///
    /// For more information: [`ID3D12GraphicsCommandList::SOSetTargets method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12graphicscommandlist-sosettargets)
    pub fn so_set_targets(
        &self,
        start_slot: u32,
        views: Option<&[StreamOutputBufferView]>,
    ) {
        unsafe {
            let views = views.map(|views| std::slice::from_raw_parts(views.as_ptr() as *const _, views.len()));

            self.0.SOSetTargets(
                start_slot,
                views
            )
        }
    }
}

impl_interface! {
    GraphicsCommandList;

    pub fn update_subresources_raw<T: Clone>(
        &self,
        dst_resource: impl AsRef<Resource>,
        intermediate: impl AsRef<Resource>,
        subresources: Range<u32>,
        required_size: usize,
        layouts: &[PlacedSubresourceFootprint],
        num_rows: &[u32],
        row_sizes: &[u64],
        src_data: &[SubresourceData<'_, T>],
    ) -> usize {
        let start = subresources.start as usize;
        let num = subresources.count();

        let dst_resource = dst_resource.as_ref();
        let intermediate = intermediate.as_ref();

        let intermediate_desc = intermediate.get_desc();
        let dst_desc = dst_resource.get_desc();

        if intermediate_desc.dimension() != ResourceDimension::Buffer
            || (intermediate_desc.width() as usize) < (required_size + layouts[0].offset() as usize)
            || (dst_desc.dimension() == ResourceDimension::Buffer && (start != 0 && num != 1)) {
                return 0;
            }

        let Ok(data) = intermediate.map::<T>(0, None) else {
            return 0;
        };

        for i in 0..num {
            let num_slices = layouts[i].footprint().depth();
            let slice_pitch = layouts[i].footprint().row_pitch() * num_rows[i];
            let total_size = num_slices * slice_pitch;

            let mut count = total_size as usize / size_of::<T>();

            if total_size as usize % size_of::<T>() > 0 {
                count += 1;
            }

            let data = unsafe {
                std::slice::from_raw_parts_mut(data.add(layouts[i].offset() as usize).as_mut(), count)
            };

            let mut dst_data = MemcpyDest::new(data)
                .with_row_pitch(layouts[i].footprint().row_pitch() as usize)
                .with_slice_pitch(slice_pitch as usize);

            memcpy_subresource(
                &mut dst_data,
                &src_data[i],
                row_sizes[i] as usize / size_of::<T>(),
                num_rows[i] as usize,
                num_slices as usize,
            );
        }

        intermediate.unmap(0, None);

        if dst_desc.dimension() == ResourceDimension::Buffer {
            self.copy_buffer_region(
                dst_resource,
                0,
                intermediate,
                layouts[0].offset(),
                layouts[0].footprint().width() as u64
            );
        } else {
            for (i, layout) in layouts.iter().enumerate().take(num).skip(start) {
                let dst = TextureCopyLocation::subresource(dst_resource, i as u32);
                let src = TextureCopyLocation::placed_footprint(intermediate, *layout);

                self.copy_texture_region(&dst, 0, 0, 0, &src, None);
            }
        }

        required_size
    }

    pub fn update_subresources_fixed<const MAX_SUBRESOURCES: usize, T: Clone>(
        &self,
        dst_resource: impl AsRef<Resource>,
        intermediate: impl AsRef<Resource>,
        intermediate_offset: u64,
        subresources: Range<u32>,
        src_data: &[SubresourceData<'_, T>],
    ) -> usize {
        let mut layouts = [unsafe { std::mem::zeroed() }; MAX_SUBRESOURCES];
        let mut num_rows = [0; MAX_SUBRESOURCES];
        let mut row_sizes = [0; MAX_SUBRESOURCES];

        let dst_resource = dst_resource.as_ref();
        let intermediate = intermediate.as_ref();

        let desc = dst_resource.get_desc();
        let device: Device = dst_resource.get_device().unwrap();

        let required_size = device.get_copyable_footprints(
            &desc,
            subresources.clone(),
            intermediate_offset,
            Some(&mut layouts),
            Some(&mut num_rows),
            Some(&mut row_sizes)
        );

        self.update_subresources_raw(
            dst_resource,
            intermediate,
            subresources,
            required_size as usize,
            &layouts,
            &num_rows,
            &row_sizes,
            src_data
        )
    }

    pub fn update_subresources<T: Clone>(
        &self,
        dst_resource: impl AsRef<Resource>,
        intermediate: impl AsRef<Resource>,
        intermediate_offset: u64,
        subresources: Range<u32>,
        src_data: &[SubresourceData<'_, T>],
    ) -> usize {
        let count = subresources.clone().count();
        let mut layouts = vec![unsafe { std::mem::zeroed() }; count];
        let mut num_rows = vec![0; count];
        let mut row_sizes = vec![0; count];

        let dst_resource = dst_resource.as_ref();
        let intermediate = intermediate.as_ref();

        let desc = dst_resource.get_desc();
        let device: Device = dst_resource.get_device().unwrap();

        let required_size = device.get_copyable_footprints(
            &desc,
            subresources.clone(),
            intermediate_offset,
            Some(&mut layouts),
            Some(&mut num_rows),
            Some(&mut row_sizes)
        );

        self.update_subresources_raw(
            dst_resource,
            intermediate,
            subresources,
            required_size as usize,
            &layouts,
            &num_rows,
            &row_sizes,
            src_data
        )
    }
}
