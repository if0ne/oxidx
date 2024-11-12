use windows::Win32::Graphics::Direct3D12::*;

use crate::conv_enum;

use super::*;

conv_enum!(AddressMode to D3D12_TEXTURE_ADDRESS_MODE);
conv_enum!(AlphaMode to DXGI_ALPHA_MODE);
conv_enum!(Blend to D3D12_BLEND);
conv_enum!(BlendOp to D3D12_BLEND_OP);
conv_enum!(BorderColor to D3D12_STATIC_BORDER_COLOR);
conv_enum!(CbufferType to D3D_CBUFFER_TYPE);
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
conv_enum!(MessageCategory to D3D12_MESSAGE_CATEGORY);
conv_enum!(MessageId to D3D12_MESSAGE_ID);
conv_enum!(MessageSeverity to D3D12_MESSAGE_SEVERITY);
conv_enum!(MinPrecision to D3D_MIN_PRECISION);
conv_enum!(MinPrecisionSupport to D3D12_SHADER_MIN_PRECISION_SUPPORT);
conv_enum!(PipelinePrimitiveTopology to D3D12_PRIMITIVE_TOPOLOGY_TYPE);
conv_enum!(PredicationOp to D3D12_PREDICATION_OP);
conv_enum!(Primitive to D3D_PRIMITIVE);
conv_enum!(PrimitiveTopology to D3D_PRIMITIVE_TOPOLOGY);
conv_enum!(ProgrammableSamplePositionsTier to D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER);
conv_enum!(QueryHeapType to D3D12_QUERY_HEAP_TYPE);
conv_enum!(QueryType to D3D12_QUERY_TYPE);
conv_enum!(RaytracingTier to D3D12_RAYTRACING_TIER);
conv_enum!(RegisterComponentType to D3D_REGISTER_COMPONENT_TYPE);
conv_enum!(RenderPassTier to D3D12_RENDER_PASS_TIER);
conv_enum!(ResourceBindingTier to D3D12_RESOURCE_BINDING_TIER);
conv_enum!(ResourceDimension to D3D12_RESOURCE_DIMENSION);
conv_enum!(ResourceHeapTier to D3D12_RESOURCE_HEAP_TIER);
conv_enum!(ResourceReturnType to D3D_RESOURCE_RETURN_TYPE);
conv_enum!(RootSignatureVersion to D3D_ROOT_SIGNATURE_VERSION);
conv_enum!(RotationMode to DXGI_MODE_ROTATION);
conv_enum!(SamplerFeedbackTier to D3D12_SAMPLER_FEEDBACK_TIER);
conv_enum!(Scaling to DXGI_SCALING);
conv_enum!(ScalingMode to DXGI_MODE_SCALING);
conv_enum!(ScanlineOrdering to DXGI_MODE_SCANLINE_ORDER);
conv_enum!(ShaderInputType to D3D_SHADER_INPUT_TYPE);
conv_enum!(ShaderModel to D3D_SHADER_MODEL);
conv_enum!(ShaderVarName to D3D_NAME);
conv_enum!(ShaderVisibility to D3D12_SHADER_VISIBILITY);
conv_enum!(SharedResourceCompatibilityTier to D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER);
conv_enum!(StencilOp to D3D12_STENCIL_OP);
conv_enum!(SrvDimension to D3D_SRV_DIMENSION);
conv_enum!(SwapEffect to DXGI_SWAP_EFFECT);
conv_enum!(TessellatorDomain to D3D_TESSELLATOR_DOMAIN);
conv_enum!(TessellatorOutputPrimitive to D3D_TESSELLATOR_OUTPUT_PRIMITIVE);
conv_enum!(TessellatorPartitioning to D3D_TESSELLATOR_PARTITIONING);
conv_enum!(TextureLayout to D3D12_TEXTURE_LAYOUT);
conv_enum!(TiledResourcesTier to D3D12_TILED_RESOURCES_TIER);
conv_enum!(VariableShadingRateTier to D3D12_VARIABLE_SHADING_RATE_TIER);
conv_enum!(ViewInstancingTier to D3D12_VIEW_INSTANCING_TIER);
conv_enum!(WaveMmaTier to D3D12_WAVE_MMA_TIER);

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

impl From<DXGI_MODE_SCALING> for Scaling {
    fn from(value: DXGI_MODE_SCALING) -> Self {
        match value {
            DXGI_MODE_SCALING_UNSPECIFIED => Self::None,
            DXGI_MODE_SCALING_CENTERED => Self::None,
            DXGI_MODE_SCALING_STRETCHED => Self::Stretch,
            _ => unreachable!(),
        }
    }
}
