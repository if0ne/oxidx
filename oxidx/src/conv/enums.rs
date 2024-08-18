use windows::Win32::Graphics::Direct3D12::*;

use crate::conv_enum;

use super::*;

conv_enum!(AddressMode to D3D12_TEXTURE_ADDRESS_MODE);
conv_enum!(AlphaMode to DXGI_ALPHA_MODE);
conv_enum!(Blend to D3D12_BLEND);
conv_enum!(BlendOp to D3D12_BLEND_OP);
conv_enum!(BorderColor to D3D12_STATIC_BORDER_COLOR);
conv_enum!(CommandListType to D3D12_COMMAND_LIST_TYPE);
conv_enum!(ComparisonFunc to D3D12_COMPARISON_FUNC);
conv_enum!(ConservativeRaster to D3D12_CONSERVATIVE_RASTERIZATION_MODE);
conv_enum!(ConservativeRasterizationTier to D3D12_CONSERVATIVE_RASTERIZATION_TIER);
conv_enum!(CpuPageProperty to D3D12_CPU_PAGE_PROPERTY);
conv_enum!(CrossNodeSharingTier to D3D12_CROSS_NODE_SHARING_TIER);
conv_enum!(CullMode to D3D12_CULL_MODE);
conv_enum!(DescriptorHeapType to D3D12_DESCRIPTOR_HEAP_TYPE);
conv_enum!(DescriptorRangeType to D3D12_DESCRIPTOR_RANGE_TYPE);
conv_enum!(FeatureLevel to D3D_FEATURE_LEVEL);
conv_enum!(FeatureType to D3D12_FEATURE);
conv_enum!(FillMode to D3D12_FILL_MODE);
conv_enum!(Filter to D3D12_FILTER);
conv_enum!(Format to DXGI_FORMAT);
conv_enum!(GpuPreference to DXGI_GPU_PREFERENCE);
conv_enum!(HeapSerializationTier to D3D12_HEAP_SERIALIZATION_TIER);
conv_enum!(HeapType to D3D12_HEAP_TYPE);
conv_enum!(IndexBufferStripCutValue to D3D12_INDEX_BUFFER_STRIP_CUT_VALUE);
conv_enum!(LogicOp to D3D12_LOGIC_OP);
conv_enum!(MemoryPool to D3D12_MEMORY_POOL);
conv_enum!(MeshShaderTier to D3D12_MESH_SHADER_TIER);
conv_enum!(MinPrecisionSupport to D3D12_SHADER_MIN_PRECISION_SUPPORT);
conv_enum!(PipelinePrimitiveTopology to D3D12_PRIMITIVE_TOPOLOGY_TYPE);
conv_enum!(PredicationOp to D3D12_PREDICATION_OP);
conv_enum!(PrimitiveTopology to D3D_PRIMITIVE_TOPOLOGY);
conv_enum!(ProgrammableSamplePositionsTier to D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER);
conv_enum!(QueryHeapType to D3D12_QUERY_HEAP_TYPE);
conv_enum!(QueryType to D3D12_QUERY_TYPE);
conv_enum!(RaytracingTier to D3D12_RAYTRACING_TIER);
conv_enum!(RenderPassTier to D3D12_RENDER_PASS_TIER);
conv_enum!(ResourceBindingTier to D3D12_RESOURCE_BINDING_TIER);
conv_enum!(ResourceDimension to D3D12_RESOURCE_DIMENSION);
conv_enum!(ResourceHeapTier to D3D12_RESOURCE_HEAP_TIER);
conv_enum!(RootSignatureVersion to D3D_ROOT_SIGNATURE_VERSION);
conv_enum!(SamplerFeedbackTier to D3D12_SAMPLER_FEEDBACK_TIER);
conv_enum!(Scaling to DXGI_SCALING);
conv_enum!(ScalingMode to DXGI_MODE_SCALING);
conv_enum!(ScanlineOrdering to DXGI_MODE_SCANLINE_ORDER);
conv_enum!(ShaderModel to D3D_SHADER_MODEL);
conv_enum!(ShaderVisibility to D3D12_SHADER_VISIBILITY);
conv_enum!(SharedResourceCompatibilityTier to D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER);
conv_enum!(StencilOp to D3D12_STENCIL_OP);
conv_enum!(SwapEffect to DXGI_SWAP_EFFECT);
conv_enum!(TextureLayout to D3D12_TEXTURE_LAYOUT);
conv_enum!(TiledResourcesTier to D3D12_TILED_RESOURCES_TIER);
conv_enum!(VariableShadingRateTier to D3D12_VARIABLE_SHADING_RATE_TIER);
conv_enum!(ViewInstancingTier to D3D12_VIEW_INSTANCING_TIER);
conv_enum!(WaveMmaTier to D3D12_WAVE_MMA_TIER);

impl ClearValue {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_CLEAR_VALUE {
        match *self {
            ClearValue::Color { format, value } => D3D12_CLEAR_VALUE {
                Format: format.as_raw(),
                Anonymous: D3D12_CLEAR_VALUE_0 { Color: value },
            },
            ClearValue::Depth {
                format,
                depth,
                stencil,
            } => D3D12_CLEAR_VALUE {
                Format: format.as_raw(),
                Anonymous: D3D12_CLEAR_VALUE_0 {
                    DepthStencil: D3D12_DEPTH_STENCIL_VALUE {
                        Depth: depth,
                        Stencil: stencil,
                    },
                },
            },
        }
    }
}

impl CommandQueuePriority {
    #[inline]
    pub(crate) fn as_raw(&self) -> i32 {
        *self as i32
    }
}

impl From<i32> for CommandQueuePriority {
    #[inline]
    fn from(value: i32) -> Self {
        let value = D3D12_COMMAND_QUEUE_PRIORITY(value);
        match value {
            D3D12_COMMAND_QUEUE_PRIORITY_NORMAL => CommandQueuePriority::Normal,
            D3D12_COMMAND_QUEUE_PRIORITY_HIGH => CommandQueuePriority::High,
            D3D12_COMMAND_QUEUE_PRIORITY_GLOBAL_REALTIME => CommandQueuePriority::GlobalRealtime,
            _ => unreachable!(),
        }
    }
}

impl HeapAlignment {
    #[inline]
    pub(crate) fn as_raw(&self) -> u64 {
        *self as u64
    }
}

impl From<u64> for HeapAlignment {
    #[inline]
    fn from(value: u64) -> Self {
        match value as u32 {
            0 => HeapAlignment::Default,
            D3D12_DEFAULT_RESOURCE_PLACEMENT_ALIGNMENT => HeapAlignment::ResourcePlacement,
            D3D12_DEFAULT_MSAA_RESOURCE_PLACEMENT_ALIGNMENT => HeapAlignment::MsaaResourcePlacement,
            _ => unreachable!(),
        }
    }
}

impl InputClass {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_INPUT_CLASSIFICATION {
        match self {
            InputClass::PerVertex => D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
            InputClass::InstanceData(_) => D3D12_INPUT_CLASSIFICATION_PER_INSTANCE_DATA,
        }
    }
}

impl TextureCopyType {
    #[inline]
    pub(crate) fn as_raw_type(&self) -> D3D12_TEXTURE_COPY_TYPE {
        match self {
            TextureCopyType::SubresourceIndex(_) => D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX,
            TextureCopyType::PlacedFootprint(_) => D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT,
        }
    }

    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_TEXTURE_COPY_LOCATION_0 {
        match self {
            TextureCopyType::SubresourceIndex(index) => D3D12_TEXTURE_COPY_LOCATION_0 {
                SubresourceIndex: *index,
            },
            TextureCopyType::PlacedFootprint(footprint) => D3D12_TEXTURE_COPY_LOCATION_0 {
                PlacedFootprint: footprint.0,
            },
        }
    }
}

impl UavDimension {
    #[inline]
    pub(crate) fn as_type_raw(&self) -> D3D12_UAV_DIMENSION {
        match self {
            UavDimension::Buffer { .. } => D3D12_UAV_DIMENSION_BUFFER,
            UavDimension::Tex1D { .. } => D3D12_UAV_DIMENSION_TEXTURE1D,
            UavDimension::Tex2D { .. } => D3D12_UAV_DIMENSION_TEXTURE2D,
            UavDimension::Tex3D { .. } => D3D12_UAV_DIMENSION_TEXTURE3D,
            UavDimension::ArrayTex1D { .. } => D3D12_UAV_DIMENSION_TEXTURE1DARRAY,
            UavDimension::ArrayTex2D { .. } => D3D12_UAV_DIMENSION_TEXTURE2DARRAY,
            UavDimension::Tex2DMs => D3D12_UAV_DIMENSION_TEXTURE2DMS,
            UavDimension::Array2DMs { .. } => D3D12_UAV_DIMENSION_TEXTURE2DMSARRAY,
        }
    }

    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {
        match self {
            UavDimension::Buffer {
                first_element,
                num_elements,
                structure_byte_stride,
                counter_offset,
                flags,
            } => D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {
                Buffer: D3D12_BUFFER_UAV {
                    FirstElement: *first_element,
                    NumElements: *num_elements,
                    StructureByteStride: *structure_byte_stride,
                    CounterOffsetInBytes: *counter_offset,
                    Flags: flags.as_raw(),
                },
            },
            UavDimension::Tex1D { mip_slice } => D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {
                Texture1D: D3D12_TEX1D_UAV {
                    MipSlice: *mip_slice,
                },
            },
            UavDimension::Tex2D {
                mip_slice,
                plane_slice,
            } => D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {
                Texture2D: D3D12_TEX2D_UAV {
                    MipSlice: *mip_slice,
                    PlaneSlice: *plane_slice,
                },
            },
            UavDimension::Tex3D {
                mip_slice,
                first_w_slice,
                w_size,
            } => D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {
                Texture3D: D3D12_TEX3D_UAV {
                    MipSlice: *mip_slice,
                    FirstWSlice: *first_w_slice,
                    WSize: *w_size,
                },
            },
            UavDimension::ArrayTex1D {
                mip_slice,
                first_array_slice,
                array_size,
            } => D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {
                Texture1DArray: D3D12_TEX1D_ARRAY_UAV {
                    MipSlice: *mip_slice,
                    FirstArraySlice: *first_array_slice,
                    ArraySize: *array_size,
                },
            },
            UavDimension::ArrayTex2D {
                mip_slice,
                plane_slice,
                first_array_slice,
                array_size,
            } => D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {
                Texture2DArray: D3D12_TEX2D_ARRAY_UAV {
                    MipSlice: *mip_slice,
                    PlaneSlice: *plane_slice,
                    FirstArraySlice: *first_array_slice,
                    ArraySize: *array_size,
                },
            },
            UavDimension::Tex2DMs => D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {
                Texture2DMS: D3D12_TEX2DMS_UAV {
                    UnusedField_NothingToDefine: 0,
                },
            },
            UavDimension::Array2DMs {
                first_array_slice,
                array_size,
            } => D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {
                Texture2DMSArray: D3D12_TEX2DMS_ARRAY_UAV {
                    FirstArraySlice: *first_array_slice,
                    ArraySize: *array_size,
                },
            },
        }
    }
}
