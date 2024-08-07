use smallvec::SmallVec;
use windows::Win32::Graphics::Direct3D12::*;

use super::*;

impl From<DXGI_ADAPTER_DESC1> for AdapterDesc1 {
    fn from(value: DXGI_ADAPTER_DESC1) -> Self {
        AdapterDesc1 {
            description: CompactString::from_utf16_lossy(value.Description),
            vendor_id: value.VendorId,
            device_id: value.DeviceId,
            sub_sys_id: value.SubSysId,
            revision: value.Revision,
            dedicated_video_memory: value.DedicatedVideoMemory,
            dedicated_system_memory: value.SharedSystemMemory,
            shared_system_memory: value.SharedSystemMemory,
            adapter_luid: Luid {
                low_part: value.AdapterLuid.LowPart,
                high_part: value.AdapterLuid.HighPart,
            },
            flags: AdapterFlags::from_bits(value.Flags as i32).unwrap(),
        }
    }
}

impl BlendDesc {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_BLEND_DESC {
        let mut render_target = [D3D12_RENDER_TARGET_BLEND_DESC::default(); 8];

        for (i, desc) in self.render_targets.iter().enumerate() {
            render_target[i] = desc.as_raw();
        }

        D3D12_BLEND_DESC {
            AlphaToCoverageEnable: self.alpha_to_coverage_enable.into(),
            IndependentBlendEnable: self.independent_blend_enable.into(),
            RenderTarget: render_target,
        }
    }
}

impl Box {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_BOX {
        D3D12_BOX {
            left: self.left,
            top: self.top,
            front: self.front,
            right: self.right,
            bottom: self.bottom,
            back: self.back,
        }
    }
}

impl CommandQueueDesc {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_COMMAND_QUEUE_DESC {
        D3D12_COMMAND_QUEUE_DESC {
            Type: self.r#type.as_raw(),
            Priority: self.priority.as_raw(),
            Flags: self.flags.as_raw(),
            NodeMask: self.node_mask,
        }
    }
}

impl From<D3D12_COMMAND_QUEUE_DESC> for CommandQueueDesc {
    #[inline]
    fn from(value: D3D12_COMMAND_QUEUE_DESC) -> Self {
        Self {
            r#type: value.Type.into(),
            priority: value.Priority.into(),
            flags: value.Flags.into(),
            node_mask: value.NodeMask,
        }
    }
}

impl<'a> CommandSignatureDesc<'a> {
    #[inline]
    pub(crate) fn as_raw(
        &self,
        arguments: &[D3D12_INDIRECT_ARGUMENT_DESC],
    ) -> D3D12_COMMAND_SIGNATURE_DESC {
        D3D12_COMMAND_SIGNATURE_DESC {
            ByteStride: self.byte_stride,
            NumArgumentDescs: arguments.len() as u32,
            pArgumentDescs: arguments.as_ptr(),
            NodeMask: self.node_mask,
        }
    }
}

impl<'a> ComputePipelineStateDesc<'a> {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_COMPUTE_PIPELINE_STATE_DESC {
        unsafe {
            D3D12_COMPUTE_PIPELINE_STATE_DESC {
                pRootSignature: std::mem::transmute_copy(self.root_signature.as_raw()),
                CS: self.cs.as_shader_bytecode(),
                NodeMask: self.node_mask,
                CachedPSO: self
                    .cached_pso
                    .map(|pso| pso.as_cached_pipeline_state())
                    .unwrap_or_default(),
                Flags: self.flags.as_raw(),
            }
        }
    }
}

impl ConstantBufferViewDesc {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_CONSTANT_BUFFER_VIEW_DESC {
        D3D12_CONSTANT_BUFFER_VIEW_DESC {
            BufferLocation: self.buffer_location,
            SizeInBytes: self.size_in_bytes,
        }
    }
}

impl CpuDescriptorHandle {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_CPU_DESCRIPTOR_HANDLE {
        D3D12_CPU_DESCRIPTOR_HANDLE { ptr: self.0 }
    }
}

impl From<D3D12_CPU_DESCRIPTOR_HANDLE> for CpuDescriptorHandle {
    #[inline]
    fn from(value: D3D12_CPU_DESCRIPTOR_HANDLE) -> Self {
        Self(value.ptr)
    }
}

impl DeclarationEntry {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_SO_DECLARATION_ENTRY {
        let semantic_name = PCSTR::from_raw(self.semantic_name.as_ref().as_ptr() as *const _);

        D3D12_SO_DECLARATION_ENTRY {
            Stream: self.stream,
            SemanticName: semantic_name,
            SemanticIndex: self.semantic_index,
            StartComponent: self.start_component,
            ComponentCount: self.component_count,
            OutputSlot: self.output_slot,
        }
    }
}

impl DepthStencilDesc {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_DEPTH_STENCIL_DESC {
        D3D12_DEPTH_STENCIL_DESC {
            DepthEnable: self.depth_enable.into(),
            DepthWriteMask: self.depth_write_mask.as_raw(),
            DepthFunc: self.depth_func.as_raw(),
            StencilEnable: self.stencil_enable.into(),
            StencilReadMask: self.stencil_read_mask,
            StencilWriteMask: self.stencil_write_mask,
            FrontFace: self.front_face.as_raw(),
            BackFace: self.back_face.as_raw(),
        }
    }
}

impl DepthStencilOpDesc {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_DEPTH_STENCILOP_DESC {
        D3D12_DEPTH_STENCILOP_DESC {
            StencilFailOp: self.stencil_fail_op.as_raw(),
            StencilDepthFailOp: self.stencil_depth_fail_op.as_raw(),
            StencilPassOp: self.stencil_pass_op.as_raw(),
            StencilFunc: self.stencil_func.as_raw(),
        }
    }
}

impl DepthStencilViewDesc {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_DEPTH_STENCIL_VIEW_DESC {
        D3D12_DEPTH_STENCIL_VIEW_DESC {
            Format: self.format.as_raw(),
            ViewDimension: self.view_dimension.as_raw(),
            Flags: self.flags.as_raw(),
            Anonymous: match self.view_dimension {
                DsvDimension::Tex1D { mip_slice } => D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
                    Texture1D: D3D12_TEX1D_DSV {
                        MipSlice: mip_slice,
                    },
                },
                DsvDimension::ArrayTex1D {
                    mip_slice,
                    first_array_slice,
                    array_size,
                } => D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
                    Texture1DArray: D3D12_TEX1D_ARRAY_DSV {
                        MipSlice: mip_slice,
                        FirstArraySlice: first_array_slice,
                        ArraySize: array_size,
                    },
                },
                DsvDimension::Tex2D { mip_slice } => D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
                    Texture2D: D3D12_TEX2D_DSV {
                        MipSlice: mip_slice,
                    },
                },
                DsvDimension::ArrayTex2D {
                    mip_slice,
                    first_array_slice,
                    array_size,
                } => D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
                    Texture2DArray: D3D12_TEX2D_ARRAY_DSV {
                        MipSlice: mip_slice,
                        FirstArraySlice: first_array_slice,
                        ArraySize: array_size,
                    },
                },
                DsvDimension::Tex2DMs => D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
                    Texture2DMS: D3D12_TEX2DMS_DSV::default(),
                },
                DsvDimension::ArrayTex2DMs {
                    first_array_slice,
                    array_size,
                } => D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
                    Texture2DMSArray: D3D12_TEX2DMS_ARRAY_DSV {
                        FirstArraySlice: first_array_slice,
                        ArraySize: array_size,
                    },
                },
            },
        }
    }
}

impl DescriptorHeapDesc {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_DESCRIPTOR_HEAP_DESC {
        D3D12_DESCRIPTOR_HEAP_DESC {
            Type: self.r#type.as_raw(),
            NumDescriptors: self.num,
            Flags: self.flags.as_raw(),
            NodeMask: self.node_mask,
        }
    }
}

impl From<D3D12_DESCRIPTOR_HEAP_DESC> for DescriptorHeapDesc {
    #[inline]
    fn from(value: D3D12_DESCRIPTOR_HEAP_DESC) -> Self {
        Self {
            r#type: value.Type.into(),
            num: value.NumDescriptors,
            flags: value.Flags.into(),
            node_mask: value.NodeMask,
        }
    }
}

impl DescriptorRange {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_DESCRIPTOR_RANGE {
        D3D12_DESCRIPTOR_RANGE {
            RangeType: self.r#type.as_raw(),
            NumDescriptors: self.num,
            BaseShaderRegister: self.base_shader_register,
            RegisterSpace: self.register_space,
            OffsetInDescriptorsFromTableStart: self.offset_in_descriptors_from_table_start,
        }
    }
}

impl<'a> DiscardRegion<'a> {
    #[inline]
    pub(crate) fn as_raw(&self, rects: &[RECT]) -> D3D12_DISCARD_REGION {
        D3D12_DISCARD_REGION {
            NumRects: rects.len() as u32,
            pRects: rects.as_ptr(),
            FirstSubresource: self.first_subresource,
            NumSubresources: self.num_subresource,
        }
    }
}

impl GpuDescriptorHandle {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_GPU_DESCRIPTOR_HANDLE {
        D3D12_GPU_DESCRIPTOR_HANDLE { ptr: self.0 as u64 }
    }
}

impl From<D3D12_GPU_DESCRIPTOR_HANDLE> for GpuDescriptorHandle {
    #[inline]
    fn from(value: D3D12_GPU_DESCRIPTOR_HANDLE) -> Self {
        Self(value.ptr as usize)
    }
}

impl<'a> GraphicsPipelineDesc<'a> {
    #[inline]
    pub(crate) fn as_raw(
        &self,
        input_layouts: &[D3D12_INPUT_ELEMENT_DESC],
        so_entries: &[D3D12_SO_DECLARATION_ENTRY],
    ) -> D3D12_GRAPHICS_PIPELINE_STATE_DESC {
        let mut rtv_formats = [DXGI_FORMAT::default(); 8];

        for (i, format) in self.rtv_formats.iter().enumerate() {
            rtv_formats[i] = format.as_raw();
        }

        D3D12_GRAPHICS_PIPELINE_STATE_DESC {
            pRootSignature: unsafe { std::mem::transmute_copy(self.root_signature.as_raw()) },
            VS: self.vs.as_shader_bytecode(),
            PS: self
                .ps
                .map(|ps| ps.as_shader_bytecode())
                .unwrap_or_default(),
            DS: self
                .ds
                .map(|ds| ds.as_shader_bytecode())
                .unwrap_or_default(),
            HS: self
                .hs
                .map(|hs| hs.as_shader_bytecode())
                .unwrap_or_default(),
            GS: self
                .gs
                .map(|gs| gs.as_shader_bytecode())
                .unwrap_or_default(),
            StreamOutput: self
                .stream_output
                .as_ref()
                .map(|so| so.as_raw(&so_entries))
                .unwrap_or_default(),
            BlendState: self.blend_state.as_raw(),
            SampleMask: self.sample_mask,
            RasterizerState: self.rasterizer_state.as_raw(),
            DepthStencilState: self
                .depth_stencil
                .as_ref()
                .map(|ds| ds.as_raw())
                .unwrap_or_default(),
            InputLayout: D3D12_INPUT_LAYOUT_DESC {
                pInputElementDescs: input_layouts.as_ptr() as *const _,
                NumElements: input_layouts.len() as u32,
            },
            IBStripCutValue: self
                .ib_strip_cut_value
                .map(|ib| ib.as_raw())
                .unwrap_or_default(),
            PrimitiveTopologyType: self.primitive_topology.as_raw(),
            NumRenderTargets: self.num_render_targets,
            RTVFormats: rtv_formats,
            DSVFormat: self.dsv_format.map(|f| f.as_raw()).unwrap_or_default(),
            SampleDesc: self.sampler_desc.as_raw(),
            NodeMask: self.node_mask,
            CachedPSO: self
                .cached_pso
                .map(|pso| pso.as_cached_pipeline_state())
                .unwrap_or_default(),
            Flags: self.flags.as_raw(),
        }
    }
}

impl HeapDesc {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_HEAP_DESC {
        D3D12_HEAP_DESC {
            SizeInBytes: self.size,
            Properties: self.props.as_raw(),
            Alignment: self.alignment.as_raw(),
            Flags: self.flags.as_raw(),
        }
    }
}

impl From<D3D12_HEAP_DESC> for HeapDesc {
    #[inline]
    fn from(value: D3D12_HEAP_DESC) -> Self {
        Self {
            size: value.SizeInBytes,
            props: value.Properties.into(),
            alignment: value.Alignment.into(),
            flags: value.Flags.into(),
        }
    }
}

impl HeapProperties {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_HEAP_PROPERTIES {
        D3D12_HEAP_PROPERTIES {
            Type: self.r#type.as_raw(),
            CPUPageProperty: self.cpu_page_propery.as_raw(),
            MemoryPoolPreference: self.memory_pool_preference.as_raw(),
            CreationNodeMask: self.creation_node_mask,
            VisibleNodeMask: self.visible_node_mask,
        }
    }
}

impl From<D3D12_HEAP_PROPERTIES> for HeapProperties {
    #[inline]
    fn from(value: D3D12_HEAP_PROPERTIES) -> Self {
        Self {
            r#type: value.Type.into(),
            cpu_page_propery: value.CPUPageProperty.into(),
            memory_pool_preference: value.MemoryPoolPreference.into(),
            creation_node_mask: value.CreationNodeMask,
            visible_node_mask: value.VisibleNodeMask,
        }
    }
}

impl IndexBufferView {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_INDEX_BUFFER_VIEW {
        D3D12_INDEX_BUFFER_VIEW {
            BufferLocation: self.buffer_location,
            SizeInBytes: self.size_in_bytes,
            Format: self.format.as_raw(),
        }
    }
}

impl InputElementDesc {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_INPUT_ELEMENT_DESC {
        let semantic_name = PCSTR::from_raw(self.semantic_name.as_ref().as_ptr() as *const _);

        D3D12_INPUT_ELEMENT_DESC {
            SemanticName: semantic_name,
            SemanticIndex: self.semantic_index,
            Format: self.format.as_raw(),
            InputSlot: self.input_slot,
            AlignedByteOffset: self.offset,
            InputSlotClass: self.slot_class.as_raw(),
            InstanceDataStepRate: self.slot_class.step_rate(),
        }
    }
}

impl Luid {
    #[inline]
    pub(crate) fn as_raw(&self) -> LUID {
        LUID {
            LowPart: self.low_part,
            HighPart: self.high_part,
        }
    }
}

impl From<LUID> for Luid {
    fn from(value: LUID) -> Self {
        Self {
            low_part: value.LowPart,
            high_part: value.HighPart,
        }
    }
}

impl From<D3D12_PACKED_MIP_INFO> for PackedMipDesc {
    #[inline]
    fn from(value: D3D12_PACKED_MIP_INFO) -> Self {
        Self {
            num_standard_mips: value.NumStandardMips,
            num_packed_mips: value.NumPackedMips,
            num_tiles_for_packed_mips: value.NumTilesForPackedMips,
            start_tile_index_in_overall_resource: value.StartTileIndexInOverallResource,
        }
    }
}

impl PlacedSubresourceFootprint {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_PLACED_SUBRESOURCE_FOOTPRINT {
        D3D12_PLACED_SUBRESOURCE_FOOTPRINT {
            Offset: self.offset,
            Footprint: self.footprint.as_raw(),
        }
    }
}

impl From<D3D12_PLACED_SUBRESOURCE_FOOTPRINT> for PlacedSubresourceFootprint {
    #[inline]
    fn from(value: D3D12_PLACED_SUBRESOURCE_FOOTPRINT) -> Self {
        Self {
            offset: value.Offset,
            footprint: value.Footprint.into(),
        }
    }
}

impl QueryHeapDesc {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_QUERY_HEAP_DESC {
        D3D12_QUERY_HEAP_DESC {
            Type: self.r#type.as_raw(),
            Count: self.count,
            NodeMask: self.node_mask,
        }
    }
}

impl RasterizerDesc {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_RASTERIZER_DESC {
        D3D12_RASTERIZER_DESC {
            FillMode: self.fill_mode.as_raw(),
            CullMode: self.cull_mode.as_raw(),
            FrontCounterClockwise: self.front_counter_clockwise.into(),
            DepthBias: self.depth_bias,
            DepthBiasClamp: self.depth_bias_clamp,
            SlopeScaledDepthBias: self.slope_scaled_depth_bias,
            DepthClipEnable: self.depth_clip_enable.into(),
            MultisampleEnable: self.multisample_enable.into(),
            AntialiasedLineEnable: self.antialiased_line_enable.into(),
            ForcedSampleCount: self.forced_sample_count,
            ConservativeRaster: self.conservative_raster.as_raw(),
        }
    }
}

impl Rational {
    pub(crate) fn as_raw(&self) -> DXGI_RATIONAL {
        DXGI_RATIONAL {
            Numerator: self.numerator,
            Denominator: self.denominator,
        }
    }
}

impl Rect {
    pub(crate) fn as_raw(&self) -> RECT {
        RECT {
            left: self.left,
            top: self.top,
            right: self.right,
            bottom: self.bottom,
        }
    }
}

impl RenderTargetViewDesc {
    pub(crate) fn as_raw(&self) -> D3D12_RENDER_TARGET_VIEW_DESC {
        D3D12_RENDER_TARGET_VIEW_DESC {
            Format: self.format.as_raw(),
            ViewDimension: self.dimension.as_type_raw(),
            Anonymous: self.dimension.as_raw(),
        }
    }
}

impl From<D3D12_RESOURCE_ALLOCATION_INFO> for ResourceAllocationInfo {
    #[inline]
    fn from(value: D3D12_RESOURCE_ALLOCATION_INFO) -> Self {
        Self {
            size_in_bytes: value.SizeInBytes,
            alignment: value.Alignment,
        }
    }
}

impl<'a> ResourceBarrier<'a> {
    pub(crate) fn as_raw(&self) -> D3D12_RESOURCE_BARRIER {
        D3D12_RESOURCE_BARRIER {
            Type: self.r#type.as_type_raw(),
            Flags: D3D12_RESOURCE_BARRIER_FLAGS(self.flags.bits()),
            Anonymous: self.r#type.as_raw(),
        }
    }
}

impl ResourceDesc {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_RESOURCE_DESC {
        D3D12_RESOURCE_DESC {
            Dimension: self.dimension.as_raw(),
            Alignment: self.alignment.as_raw(),
            Width: self.width,
            Height: self.height,
            DepthOrArraySize: self.depth_or_array_size,
            MipLevels: self.mip_levels,
            Format: self.format.as_raw(),
            SampleDesc: self.sample_desc.as_raw(),
            Layout: self.layout.as_raw(),
            Flags: self.flags.as_raw(),
        }
    }
}

impl From<D3D12_RESOURCE_DESC> for ResourceDesc {
    #[inline]
    fn from(value: D3D12_RESOURCE_DESC) -> Self {
        Self {
            dimension: value.Dimension.into(),
            alignment: value.Alignment.into(),
            width: value.Width,
            height: value.Height,
            depth_or_array_size: value.DepthOrArraySize,
            mip_levels: value.MipLevels,
            sample_desc: value.SampleDesc.into(),
            format: value.Format.into(),
            layout: value.Layout.into(),
            flags: value.Flags.into(),
        }
    }
}

impl<'a> RootParameter<'a> {
    #[inline]
    pub(crate) fn as_raw<const N: usize>(
        &self,
        param: &[SmallVec<[D3D12_DESCRIPTOR_RANGE; N]>],
    ) -> D3D12_ROOT_PARAMETER {
        D3D12_ROOT_PARAMETER {
            ParameterType: self.r#type.as_type_raw(),
            Anonymous: self.r#type.as_raw(param),
            ShaderVisibility: self.visibility.as_raw(),
        }
    }
}

impl<'a> RootSignatureDesc<'a> {
    #[inline]
    pub(crate) fn as_raw<const N: usize>(
        &self,
        params: &[SmallVec<[D3D12_DESCRIPTOR_RANGE; N]>],
    ) -> D3D12_ROOT_SIGNATURE_DESC {
        let parameters = self
            .parameters
            .iter()
            .map(|param| param.as_raw(params))
            .collect::<SmallVec<[_; 16]>>();
        let sampler = self
            .samplers
            .iter()
            .map(|sampler| sampler.as_raw())
            .collect::<SmallVec<[_; 16]>>();

        D3D12_ROOT_SIGNATURE_DESC {
            NumParameters: self.parameters.len() as u32,
            pParameters: parameters.as_ptr(),
            NumStaticSamplers: self.samplers.len() as u32,
            pStaticSamplers: sampler.as_ptr(),
            Flags: self.flags.as_raw(),
        }
    }
}

impl SampleDesc {
    #[inline]
    pub(crate) fn as_raw(&self) -> DXGI_SAMPLE_DESC {
        DXGI_SAMPLE_DESC {
            Count: self.count,
            Quality: self.quality,
        }
    }
}

impl From<DXGI_SAMPLE_DESC> for SampleDesc {
    fn from(value: DXGI_SAMPLE_DESC) -> Self {
        SampleDesc {
            count: value.Count,
            quality: value.Quality,
        }
    }
}

impl SamplerDesc {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_SAMPLER_DESC {
        D3D12_SAMPLER_DESC {
            Filter: self.filter.as_raw(),
            AddressU: self.address_u.as_raw(),
            AddressV: self.address_v.as_raw(),
            AddressW: self.address_w.as_raw(),
            MipLODBias: self.mip_lod_bias,
            MaxAnisotropy: self.max_anisotropy,
            ComparisonFunc: self.comparison_func.as_raw(),
            BorderColor: self.border_color,
            MinLOD: self.min_lod,
            MaxLOD: self.max_lod,
        }
    }
}

impl ShaderResourceViewDesc {
    pub(crate) fn as_raw(&self) -> D3D12_SHADER_RESOURCE_VIEW_DESC {
        D3D12_SHADER_RESOURCE_VIEW_DESC {
            Format: self.format.as_raw(),
            ViewDimension: self.dimension.as_type_raw(),
            Anonymous: self.dimension.as_raw(),
            // TODO: Make it in shader resource view desc
            Shader4ComponentMapping: 0x7,
        }
    }
}

impl StaticSamplerDesc {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_STATIC_SAMPLER_DESC {
        D3D12_STATIC_SAMPLER_DESC {
            Filter: self.filter.as_raw(),
            AddressU: self.address_u.as_raw(),
            AddressV: self.address_v.as_raw(),
            AddressW: self.address_w.as_raw(),
            MipLODBias: self.mip_lod_bias,
            MaxAnisotropy: self.max_anisotropy,
            ComparisonFunc: self.comparison_func.as_raw(),
            BorderColor: self.border_color.as_raw(),
            MinLOD: self.min_lod,
            MaxLOD: self.max_lod,
            ShaderRegister: self.shader_register,
            RegisterSpace: self.register_space,
            ShaderVisibility: self.visibility.as_raw(),
        }
    }
}

impl StreamOutputBufferView {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_STREAM_OUTPUT_BUFFER_VIEW {
        D3D12_STREAM_OUTPUT_BUFFER_VIEW {
            BufferLocation: self.buffer_location,
            SizeInBytes: self.size_in_bytes,
            BufferFilledSizeLocation: self.buffer_filled_size_location,
        }
    }
}

impl<'a> StreamOutputDesc<'a> {
    #[inline]
    pub(crate) fn as_raw(
        &self,
        entries: &[D3D12_SO_DECLARATION_ENTRY],
    ) -> D3D12_STREAM_OUTPUT_DESC {
        D3D12_STREAM_OUTPUT_DESC {
            pSODeclaration: entries.as_ptr(),
            NumEntries: entries.len() as u32,
            pBufferStrides: self.buffer_strides.as_ptr(),
            NumStrides: self.buffer_strides.len() as u32,
            RasterizedStream: self.rasterized_stream,
        }
    }
}

impl SubresourceFootprint {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_SUBRESOURCE_FOOTPRINT {
        D3D12_SUBRESOURCE_FOOTPRINT {
            Format: self.format.as_raw(),
            Width: self.width,
            Height: self.height,
            Depth: self.depth,
            RowPitch: self.row_pitch,
        }
    }
}

impl From<D3D12_SUBRESOURCE_FOOTPRINT> for SubresourceFootprint {
    #[inline]
    fn from(value: D3D12_SUBRESOURCE_FOOTPRINT) -> Self {
        Self {
            format: value.Format.into(),
            width: value.Width,
            height: value.Height,
            depth: value.Depth,
            row_pitch: value.RowPitch,
        }
    }
}

impl From<D3D12_SUBRESOURCE_TILING> for SubresourceTiling {
    #[inline]
    fn from(value: D3D12_SUBRESOURCE_TILING) -> Self {
        Self {
            width_in_tiles: value.WidthInTiles,
            height_in_tiles: value.HeightInTiles,
            depth_in_tiles: value.DepthInTiles,
            start_tile_index_in_overall_resource: value.StartTileIndexInOverallResource,
        }
    }
}

impl SwapchainDesc1 {
    #[inline]
    pub(crate) fn as_raw(&self) -> DXGI_SWAP_CHAIN_DESC1 {
        DXGI_SWAP_CHAIN_DESC1 {
            Width: self.width,
            Height: self.height,
            Format: self.format.as_raw(),
            Stereo: self.stereo.into(),
            SampleDesc: self.sample_desc.as_raw(),
            BufferUsage: self.usage.as_raw(),
            BufferCount: self.buffer_count,
            Scaling: self.scaling.as_raw(),
            SwapEffect: self.swap_effect.as_raw(),
            AlphaMode: self.alpha_mode.as_raw(),
            Flags: self.flags.bits() as u32,
        }
    }
}

impl SwapchainFullscreenDesc {
    pub(crate) fn as_raw(&self) -> DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
        DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
            RefreshRate: self.rational.as_raw(),
            ScanlineOrdering: self.scanline_ordering.as_raw(),
            Scaling: self.scaling.as_raw(),
            Windowed: self.windowed.into(),
        }
    }
}

impl<'a> TextureCopyLocation<'a> {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_TEXTURE_COPY_LOCATION {
        unsafe {
            D3D12_TEXTURE_COPY_LOCATION {
                pResource: std::mem::transmute_copy(self.resource.as_raw()),
                Type: self.r#type.as_raw_type(),
                Anonymous: self.r#type.as_raw(),
            }
        }
    }
}

impl TileRegionSize {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_TILE_REGION_SIZE {
        D3D12_TILE_REGION_SIZE {
            NumTiles: self.num_tiles,
            UseBox: self.use_box.into(),
            Width: self.width,
            Height: self.height,
            Depth: self.depth,
        }
    }
}

impl From<D3D12_TILE_SHAPE> for TileShape {
    #[inline]
    fn from(value: D3D12_TILE_SHAPE) -> Self {
        Self {
            width_in_texels: value.WidthInTexels,
            height_in_texels: value.HeightInTexels,
            depth_in_texels: value.DepthInTexels,
        }
    }
}

impl TiledResourceCoordinate {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_TILED_RESOURCE_COORDINATE {
        D3D12_TILED_RESOURCE_COORDINATE {
            X: self.x,
            Y: self.y,
            Z: self.z,
            Subresource: self.subresource,
        }
    }
}

impl UnorderedAccessViewDesc {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_UNORDERED_ACCESS_VIEW_DESC {
        D3D12_UNORDERED_ACCESS_VIEW_DESC {
            Format: self.format.as_raw(),
            ViewDimension: self.dimension.as_type_raw(),
            Anonymous: self.dimension.as_raw(),
        }
    }
}

impl VertexBufferView {
    pub(crate) fn as_raw(&self) -> D3D12_VERTEX_BUFFER_VIEW {
        D3D12_VERTEX_BUFFER_VIEW {
            BufferLocation: self.buffer_location,
            SizeInBytes: self.size_in_bytes,
            StrideInBytes: self.stride_in_bytes,
        }
    }
}

impl Viewport {
    pub(crate) fn as_raw(&self) -> D3D12_VIEWPORT {
        D3D12_VIEWPORT {
            TopLeftX: self.x,
            TopLeftY: self.y,
            Width: self.width,
            Height: self.height,
            MinDepth: self.min_depth,
            MaxDepth: self.max_depth,
        }
    }
}
