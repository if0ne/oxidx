use windows::Win32::Graphics::Direct3D12::*;

use crate::conv_enum;

use super::*;

conv_enum!(Blend to D3D12_BLEND);
conv_enum!(BlendOp to D3D12_BLEND_OP);
conv_enum!(CommandListType to D3D12_COMMAND_LIST_TYPE);
conv_enum!(CpuPageProperty to D3D12_CPU_PAGE_PROPERTY);
conv_enum!(ComparisonFunc to D3D12_COMPARISON_FUNC);
conv_enum!(ConservativeRaster to D3D12_CONSERVATIVE_RASTERIZATION_MODE);
conv_enum!(ConservativeRasterizationTier to D3D12_CONSERVATIVE_RASTERIZATION_TIER);
conv_enum!(CrossNodeSharingTier to D3D12_CROSS_NODE_SHARING_TIER);
conv_enum!(CullMode to D3D12_CULL_MODE);
conv_enum!(DescriptorHeapType to D3D12_DESCRIPTOR_HEAP_TYPE);
conv_enum!(FeatureType to D3D12_FEATURE);
conv_enum!(FeatureLevel to D3D_FEATURE_LEVEL);
conv_enum!(FillMode to D3D12_FILL_MODE);
conv_enum!(Format to DXGI_FORMAT);
conv_enum!(HeapSerializationTier to D3D12_HEAP_SERIALIZATION_TIER);
conv_enum!(HeapType to D3D12_HEAP_TYPE);
conv_enum!(IndexBufferStripCutValue to D3D12_INDEX_BUFFER_STRIP_CUT_VALUE);
conv_enum!(InputSlotClass to D3D12_INPUT_CLASSIFICATION);
conv_enum!(LogicOp to D3D12_LOGIC_OP);
conv_enum!(MemoryPool to D3D12_MEMORY_POOL);
conv_enum!(MeshShaderTier to D3D12_MESH_SHADER_TIER);
conv_enum!(MinPrecisionSupport to D3D12_SHADER_MIN_PRECISION_SUPPORT);
conv_enum!(PrimitiveTopology to D3D_PRIMITIVE_TOPOLOGY);
conv_enum!(PipelinePrimitiveTopology to D3D12_PRIMITIVE_TOPOLOGY_TYPE);
conv_enum!(ProgrammableSamplePositionsTier to D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER);
conv_enum!(ResourceBindingTier to D3D12_RESOURCE_BINDING_TIER);
conv_enum!(ResourceDimension to D3D12_RESOURCE_DIMENSION);
conv_enum!(ResourceHeapTier to D3D12_RESOURCE_HEAP_TIER);
conv_enum!(RaytracingTier to D3D12_RAYTRACING_TIER);
conv_enum!(RenderPassTier to D3D12_RENDER_PASS_TIER);
conv_enum!(RootSignatureVersion to D3D_ROOT_SIGNATURE_VERSION);
conv_enum!(SamplerFeedbackTier to D3D12_SAMPLER_FEEDBACK_TIER);
conv_enum!(ShaderModel to D3D_SHADER_MODEL);
conv_enum!(SharedResourceCompatibilityTier to D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER);
conv_enum!(StencilOp to D3D12_STENCIL_OP);
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

impl DsvDimension {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_DSV_DIMENSION {
        match self {
            DsvDimension::Tex1D { .. } => D3D12_DSV_DIMENSION_TEXTURE1D,
            DsvDimension::ArrayTex1D { .. } => D3D12_DSV_DIMENSION_TEXTURE1DARRAY,
            DsvDimension::Tex2D { .. } => D3D12_DSV_DIMENSION_TEXTURE2D,
            DsvDimension::ArrayTex2D { .. } => D3D12_DSV_DIMENSION_TEXTURE2DARRAY,
            DsvDimension::Tex2DMs => D3D12_DSV_DIMENSION_TEXTURE2DMS,
            DsvDimension::ArrayTex2DMs { .. } => D3D12_DSV_DIMENSION_TEXTURE2DMSARRAY,
        }
    }
}

impl IndirectArgumentDesc {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_INDIRECT_ARGUMENT_DESC {
        D3D12_INDIRECT_ARGUMENT_DESC {
            Type: self.as_raw_type(),
            Anonymous: match *self {
                IndirectArgumentDesc::Draw => Default::default(),
                IndirectArgumentDesc::DrawIndexed => Default::default(),
                IndirectArgumentDesc::Dispatch => Default::default(),
                IndirectArgumentDesc::VertexBufferView { slot } => D3D12_INDIRECT_ARGUMENT_DESC_0 {
                    VertexBuffer: D3D12_INDIRECT_ARGUMENT_DESC_0_4 { Slot: slot },
                },
                IndirectArgumentDesc::IndexBufferView => Default::default(),
                IndirectArgumentDesc::Constant {
                    root_parameter_index,
                    dest_offset_in32_bit_values,
                    num32_bit_values_to_set,
                } => D3D12_INDIRECT_ARGUMENT_DESC_0 {
                    Constant: D3D12_INDIRECT_ARGUMENT_DESC_0_1 {
                        RootParameterIndex: root_parameter_index,
                        DestOffsetIn32BitValues: dest_offset_in32_bit_values,
                        Num32BitValuesToSet: num32_bit_values_to_set,
                    },
                },
                IndirectArgumentDesc::ConstantBufferView {
                    root_parameter_index,
                } => D3D12_INDIRECT_ARGUMENT_DESC_0 {
                    ConstantBufferView: D3D12_INDIRECT_ARGUMENT_DESC_0_0 {
                        RootParameterIndex: root_parameter_index,
                    },
                },
                IndirectArgumentDesc::ShaderResourceView {
                    root_parameter_index,
                } => D3D12_INDIRECT_ARGUMENT_DESC_0 {
                    ShaderResourceView: D3D12_INDIRECT_ARGUMENT_DESC_0_2 {
                        RootParameterIndex: root_parameter_index,
                    },
                },
                IndirectArgumentDesc::UnorderedAccessView {
                    root_parameter_index,
                } => D3D12_INDIRECT_ARGUMENT_DESC_0 {
                    UnorderedAccessView: D3D12_INDIRECT_ARGUMENT_DESC_0_3 {
                        RootParameterIndex: root_parameter_index,
                    },
                },
            },
        }
    }

    #[inline]
    fn as_raw_type(&self) -> D3D12_INDIRECT_ARGUMENT_TYPE {
        match self {
            IndirectArgumentDesc::Draw => D3D12_INDIRECT_ARGUMENT_TYPE_DRAW,
            IndirectArgumentDesc::DrawIndexed => D3D12_INDIRECT_ARGUMENT_TYPE_DRAW_INDEXED,
            IndirectArgumentDesc::Dispatch => D3D12_INDIRECT_ARGUMENT_TYPE_DISPATCH,
            IndirectArgumentDesc::VertexBufferView { .. } => {
                D3D12_INDIRECT_ARGUMENT_TYPE_VERTEX_BUFFER_VIEW
            }
            IndirectArgumentDesc::IndexBufferView => D3D12_INDIRECT_ARGUMENT_TYPE_INDEX_BUFFER_VIEW,
            IndirectArgumentDesc::Constant { .. } => D3D12_INDIRECT_ARGUMENT_TYPE_CONSTANT,
            IndirectArgumentDesc::ConstantBufferView { .. } => {
                D3D12_INDIRECT_ARGUMENT_TYPE_CONSTANT_BUFFER_VIEW
            }
            IndirectArgumentDesc::ShaderResourceView { .. } => {
                D3D12_INDIRECT_ARGUMENT_TYPE_SHADER_RESOURCE_VIEW
            }
            IndirectArgumentDesc::UnorderedAccessView { .. } => {
                D3D12_INDIRECT_ARGUMENT_TYPE_UNORDERED_ACCESS_VIEW
            }
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

impl RenderTargetBlendDesc {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_RENDER_TARGET_BLEND_DESC {
        match self {
            RenderTargetBlendDesc::None => D3D12_RENDER_TARGET_BLEND_DESC::default(),
            RenderTargetBlendDesc::Blend {
                src_blend,
                dst_blend,
                blend_op,
                src_blend_alpha,
                dst_blend_alpha,
                blend_op_alpha,
                mask,
            } => D3D12_RENDER_TARGET_BLEND_DESC {
                BlendEnable: true.into(),
                SrcBlend: src_blend.as_raw(),
                DestBlend: dst_blend.as_raw(),
                BlendOp: blend_op.as_raw(),
                SrcBlendAlpha: src_blend_alpha.as_raw(),
                DestBlendAlpha: dst_blend_alpha.as_raw(),
                BlendOpAlpha: blend_op_alpha.as_raw(),
                RenderTargetWriteMask: mask.as_raw().0 as u8,
                ..Default::default()
            },
            RenderTargetBlendDesc::Logic { logic_op, mask } => D3D12_RENDER_TARGET_BLEND_DESC {
                LogicOpEnable: true.into(),
                LogicOp: logic_op.as_raw(),
                RenderTargetWriteMask: mask.as_raw().0 as u8,
                ..Default::default()
            },
        }
    }
}
