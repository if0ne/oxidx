use smallvec::SmallVec;
use windows::Win32::Graphics::Direct3D12::*;

use crate::conv_enum;

use super::*;

conv_enum!(AddressMode to D3D12_TEXTURE_ADDRESS_MODE);
conv_enum!(Blend to D3D12_BLEND);
conv_enum!(BlendOp to D3D12_BLEND_OP);
conv_enum!(BorderColor to D3D12_STATIC_BORDER_COLOR);
conv_enum!(CommandListType to D3D12_COMMAND_LIST_TYPE);
conv_enum!(CpuPageProperty to D3D12_CPU_PAGE_PROPERTY);
conv_enum!(ComparisonFunc to D3D12_COMPARISON_FUNC);
conv_enum!(ConservativeRaster to D3D12_CONSERVATIVE_RASTERIZATION_MODE);
conv_enum!(ConservativeRasterizationTier to D3D12_CONSERVATIVE_RASTERIZATION_TIER);
conv_enum!(CrossNodeSharingTier to D3D12_CROSS_NODE_SHARING_TIER);
conv_enum!(CullMode to D3D12_CULL_MODE);
conv_enum!(DescriptorHeapType to D3D12_DESCRIPTOR_HEAP_TYPE);
conv_enum!(DescriptorRangeType to D3D12_DESCRIPTOR_RANGE_TYPE);
conv_enum!(FeatureType to D3D12_FEATURE);
conv_enum!(FeatureLevel to D3D_FEATURE_LEVEL);
conv_enum!(FillMode to D3D12_FILL_MODE);
conv_enum!(Filter to D3D12_FILTER);
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
conv_enum!(ShaderVisibility to D3D12_SHADER_VISIBILITY);
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
            RenderTargetBlendDesc::None => D3D12_RENDER_TARGET_BLEND_DESC {
                RenderTargetWriteMask: ColorWriteEnable::all().as_raw().0 as u8,
                ..Default::default()
            },
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


impl<'a> RootParameterType<'a> {
    #[inline]
    pub(crate) fn as_type_raw(&self) -> D3D12_ROOT_PARAMETER_TYPE {
        match self {
            RootParameterType::DescriptorTable { .. } => D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE,
            RootParameterType::Constants32Bit { .. } => D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS,
            RootParameterType::Cbv { .. } => D3D12_ROOT_PARAMETER_TYPE_CBV,
            RootParameterType::Srv { .. } => D3D12_ROOT_PARAMETER_TYPE_SRV,
            RootParameterType::Uav { .. } => D3D12_ROOT_PARAMETER_TYPE_UAV,
        }
    }

    #[inline(always)]
    pub(crate) fn as_raw(&self) -> D3D12_ROOT_PARAMETER_0 {
        let ranges = if let RootParameterType::DescriptorTable { ranges } = self {
            ranges
                .iter()
                .map(|r| r.as_raw())
                .collect::<SmallVec<[_; 16]>>()
        } else {
            SmallVec::new()
        };

        match self {
            RootParameterType::Cbv {
                shader_register,
                register_space,
            } => D3D12_ROOT_PARAMETER_0 {
                Descriptor: D3D12_ROOT_DESCRIPTOR {
                    ShaderRegister: *shader_register,
                    RegisterSpace: *register_space,
                },
            },
            RootParameterType::Srv {
                shader_register,
                register_space,
            } => D3D12_ROOT_PARAMETER_0 {
                Descriptor: D3D12_ROOT_DESCRIPTOR {
                    ShaderRegister: *shader_register,
                    RegisterSpace: *register_space,
                },
            },
            RootParameterType::Uav {
                shader_register,
                register_space,
            } => D3D12_ROOT_PARAMETER_0 {
                Descriptor: D3D12_ROOT_DESCRIPTOR {
                    ShaderRegister: *shader_register,
                    RegisterSpace: *register_space,
                },
            },
            RootParameterType::DescriptorTable { .. } => D3D12_ROOT_PARAMETER_0 {
                DescriptorTable: D3D12_ROOT_DESCRIPTOR_TABLE {
                    NumDescriptorRanges: ranges.len() as u32,
                    pDescriptorRanges: ranges.as_ptr() as *const _,
                },
            },
            RootParameterType::Constants32Bit {
                shader_register,
                register_space,
                num_32bit_values,
            } => D3D12_ROOT_PARAMETER_0 {
                Constants: D3D12_ROOT_CONSTANTS {
                    ShaderRegister: *shader_register,
                    RegisterSpace: *register_space,
                    Num32BitValues: *num_32bit_values,
                },
            },
        }
    }
}

impl RtvDimension {
    pub(crate) fn as_type_raw(&self) -> D3D12_RTV_DIMENSION {
        match self {
            RtvDimension::Buffer { .. } => D3D12_RTV_DIMENSION_BUFFER,
            RtvDimension::Tex1D { .. } => D3D12_RTV_DIMENSION_TEXTURE1D,
            RtvDimension::Tex2D { .. } => D3D12_RTV_DIMENSION_TEXTURE2D,
            RtvDimension::Tex3D { .. } => D3D12_RTV_DIMENSION_TEXTURE3D,
            RtvDimension::ArrayTex1D { .. } => D3D12_RTV_DIMENSION_TEXTURE1DARRAY,
            RtvDimension::ArrayTex2D { .. } => D3D12_RTV_DIMENSION_TEXTURE2DARRAY,
            RtvDimension::Tex2DMs => D3D12_RTV_DIMENSION_TEXTURE2DMS,
            RtvDimension::Array2DMs { .. } => D3D12_RTV_DIMENSION_TEXTURE2DMSARRAY,
        }
    }

    pub(crate) fn as_raw(&self) -> D3D12_RENDER_TARGET_VIEW_DESC_0 {
        match self {
            RtvDimension::Buffer {
                first_element,
                num_elements,
            } => D3D12_RENDER_TARGET_VIEW_DESC_0 {
                Buffer: D3D12_BUFFER_RTV {
                    FirstElement: *first_element,
                    NumElements: *num_elements,
                },
            },
            RtvDimension::Tex1D { mip_slice } => D3D12_RENDER_TARGET_VIEW_DESC_0 {
                Texture1D: D3D12_TEX1D_RTV {
                    MipSlice: *mip_slice,
                },
            },
            RtvDimension::Tex2D {
                mip_slice,
                plane_slice,
            } => D3D12_RENDER_TARGET_VIEW_DESC_0 {
                Texture2D: D3D12_TEX2D_RTV {
                    MipSlice: *mip_slice,
                    PlaneSlice: *plane_slice,
                },
            },
            RtvDimension::Tex3D {
                mip_slice,
                first_w_slice,
                w_size,
            } => D3D12_RENDER_TARGET_VIEW_DESC_0 {
                Texture3D: D3D12_TEX3D_RTV {
                    MipSlice: *mip_slice,
                    FirstWSlice: *first_w_slice,
                    WSize: *w_size,
                },
            },
            RtvDimension::ArrayTex1D {
                mip_slice,
                first_array_slice,
                array_size,
            } => D3D12_RENDER_TARGET_VIEW_DESC_0 {
                Texture1DArray: D3D12_TEX1D_ARRAY_RTV {
                    MipSlice: *mip_slice,
                    FirstArraySlice: *first_array_slice,
                    ArraySize: *array_size,
                },
            },
            RtvDimension::ArrayTex2D {
                mip_slice,
                plane_slice,
                first_array_slice,
                array_size,
            } => D3D12_RENDER_TARGET_VIEW_DESC_0 {
                Texture2DArray: D3D12_TEX2D_ARRAY_RTV {
                    MipSlice: *mip_slice,
                    PlaneSlice: *plane_slice,
                    FirstArraySlice: *first_array_slice,
                    ArraySize: *array_size,
                },
            },
            RtvDimension::Tex2DMs => D3D12_RENDER_TARGET_VIEW_DESC_0 {
                Texture2DMS: D3D12_TEX2DMS_RTV {
                    UnusedField_NothingToDefine: 0,
                },
            },
            RtvDimension::Array2DMs {
                first_array_slice,
                array_size,
            } => D3D12_RENDER_TARGET_VIEW_DESC_0 {
                Texture2DMSArray: D3D12_TEX2DMS_ARRAY_RTV {
                    FirstArraySlice: *first_array_slice,
                    ArraySize: *array_size,
                },
            },
        }
    }
}