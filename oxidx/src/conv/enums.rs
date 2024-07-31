use std::mem::ManuallyDrop;

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
conv_enum!(QueryHeapType to D3D12_QUERY_HEAP_TYPE);
conv_enum!(QueryType to D3D12_QUERY_TYPE);
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
conv_enum!(PredicationOp to D3D12_PREDICATION_OP);
conv_enum!(Scaling to DXGI_SCALING);
conv_enum!(ScalingMode to DXGI_MODE_SCALING);
conv_enum!(ScanlineOrdering to DXGI_MODE_SCANLINE_ORDER);
conv_enum!(SwapEffect to DXGI_SWAP_EFFECT);
conv_enum!(AlphaMode to DXGI_ALPHA_MODE);

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

    #[inline]
    pub(crate) fn as_raw<const N: usize>(
        &self,
        ctx: &[SmallVec<[D3D12_DESCRIPTOR_RANGE; N]>],
    ) -> D3D12_ROOT_PARAMETER_0 {
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
                    NumDescriptorRanges: ctx.len() as u32,
                    pDescriptorRanges: ctx.as_ptr() as *const _,
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
    #[inline]
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

    #[inline]
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

impl SrvDimension {
    #[inline]
    pub(crate) fn as_type_raw(&self) -> D3D12_SRV_DIMENSION {
        match self {
            SrvDimension::Buffer { .. } => D3D12_SRV_DIMENSION_BUFFER,
            SrvDimension::Tex1D { .. } => D3D12_SRV_DIMENSION_TEXTURE1D,
            SrvDimension::ArrayTex1D { .. } => D3D12_SRV_DIMENSION_TEXTURE1DARRAY,
            SrvDimension::Tex2D { .. } => D3D12_SRV_DIMENSION_TEXTURE2D,
            SrvDimension::ArrayTex2D { .. } => D3D12_SRV_DIMENSION_TEXTURE2DARRAY,
            SrvDimension::Tex2DMs => D3D12_SRV_DIMENSION_TEXTURE2DMS,
            SrvDimension::Array2DMs { .. } => D3D12_SRV_DIMENSION_TEXTURE2DMSARRAY,
            SrvDimension::Tex3D { .. } => D3D12_SRV_DIMENSION_TEXTURE3D,
            SrvDimension::TexCube { .. } => D3D12_SRV_DIMENSION_TEXTURECUBE,
            SrvDimension::ArrayCube { .. } => D3D12_SRV_DIMENSION_TEXTURECUBEARRAY,
            SrvDimension::RaytracingAccelerationStructure { .. } => {
                D3D12_SRV_DIMENSION_RAYTRACING_ACCELERATION_STRUCTURE
            }
        }
    }

    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
        match self {
            SrvDimension::Buffer {
                first_element,
                num_elements,
                structure_byte_stride,
                flags,
            } => D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                Buffer: D3D12_BUFFER_SRV {
                    FirstElement: *first_element,
                    NumElements: *num_elements,
                    StructureByteStride: *structure_byte_stride,
                    Flags: flags.as_raw(),
                },
            },
            SrvDimension::Tex1D {
                most_detailed_mip,
                mip_levels,
                resource_min_lod_clamp,
            } => D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                Texture1D: D3D12_TEX1D_SRV {
                    MostDetailedMip: *most_detailed_mip,
                    MipLevels: *mip_levels,
                    ResourceMinLODClamp: *resource_min_lod_clamp,
                },
            },
            SrvDimension::Tex2D {
                most_detailed_mip,
                mip_levels,
                resource_min_lod_clamp,
                plane_slice,
            } => D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                Texture2D: D3D12_TEX2D_SRV {
                    MostDetailedMip: *most_detailed_mip,
                    MipLevels: *mip_levels,
                    ResourceMinLODClamp: *resource_min_lod_clamp,
                    PlaneSlice: *plane_slice,
                },
            },
            SrvDimension::Tex3D {
                most_detailed_mip,
                mip_levels,
                resource_min_lod_clamp,
            } => D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                Texture3D: D3D12_TEX3D_SRV {
                    MostDetailedMip: *most_detailed_mip,
                    MipLevels: *mip_levels,
                    ResourceMinLODClamp: *resource_min_lod_clamp,
                },
            },
            SrvDimension::ArrayTex1D {
                most_detailed_mip,
                mip_levels,
                resource_min_lod_clamp,
                first_array_slice,
                array_size,
            } => D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                Texture1DArray: D3D12_TEX1D_ARRAY_SRV {
                    MostDetailedMip: *most_detailed_mip,
                    MipLevels: *mip_levels,
                    ResourceMinLODClamp: *resource_min_lod_clamp,
                    FirstArraySlice: *first_array_slice,
                    ArraySize: *array_size,
                },
            },
            SrvDimension::ArrayTex2D {
                most_detailed_mip,
                mip_levels,
                resource_min_lod_clamp,
                plane_slice,
                first_array_slice,
                array_size,
            } => D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                Texture2DArray: D3D12_TEX2D_ARRAY_SRV {
                    MostDetailedMip: *most_detailed_mip,
                    MipLevels: *mip_levels,
                    ResourceMinLODClamp: *resource_min_lod_clamp,
                    FirstArraySlice: *first_array_slice,
                    ArraySize: *array_size,
                    PlaneSlice: *plane_slice,
                },
            },
            SrvDimension::Tex2DMs => D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                Texture2DMS: D3D12_TEX2DMS_SRV {
                    UnusedField_NothingToDefine: 0,
                },
            },
            SrvDimension::Array2DMs {
                first_array_slice,
                array_size,
            } => D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                Texture2DMSArray: D3D12_TEX2DMS_ARRAY_SRV {
                    FirstArraySlice: *first_array_slice,
                    ArraySize: *array_size,
                },
            },
            SrvDimension::TexCube {
                most_detailed_mip,
                mip_levels,
                resource_min_lod_clamp,
            } => D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                TextureCube: D3D12_TEXCUBE_SRV {
                    MostDetailedMip: *most_detailed_mip,
                    MipLevels: *mip_levels,
                    ResourceMinLODClamp: *resource_min_lod_clamp,
                },
            },
            SrvDimension::ArrayCube {
                most_detailed_mip,
                mip_levels,
                resource_min_lod_clamp,
                first_2d_array_face,
                num_cubes,
            } => D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                TextureCubeArray: D3D12_TEXCUBE_ARRAY_SRV {
                    MostDetailedMip: *most_detailed_mip,
                    MipLevels: *mip_levels,
                    ResourceMinLODClamp: *resource_min_lod_clamp,
                    First2DArrayFace: *first_2d_array_face,
                    NumCubes: *num_cubes,
                },
            },
            SrvDimension::RaytracingAccelerationStructure { location } => {
                D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                    RaytracingAccelerationStructure: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_SRV {
                        Location: *location,
                    },
                }
            }
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
                PlacedFootprint: footprint.as_raw(),
            },
        }
    }
}

impl<'a> BarrierType<'a> {
    pub(crate) fn as_raw(&self) -> D3D12_RESOURCE_BARRIER_0 {
        match self {
            BarrierType::Transition {
                resource,
                subresource,
                before,
                after,
            } => D3D12_RESOURCE_BARRIER_0 {
                Transition: ManuallyDrop::new(D3D12_RESOURCE_TRANSITION_BARRIER {
                    pResource: unsafe { std::mem::transmute_copy(resource.as_raw()) },
                    Subresource: *subresource,
                    StateBefore: before.as_raw(),
                    StateAfter: after.as_raw(),
                }),
            },
            BarrierType::Aliasing { before, after } => D3D12_RESOURCE_BARRIER_0 {
                Aliasing: ManuallyDrop::new(D3D12_RESOURCE_ALIASING_BARRIER {
                    pResourceBefore: unsafe { std::mem::transmute_copy(before.as_raw()) },
                    pResourceAfter: unsafe { std::mem::transmute_copy(after.as_raw()) },
                }),
            },
            BarrierType::Uav { resource } => D3D12_RESOURCE_BARRIER_0 {
                UAV: ManuallyDrop::new(D3D12_RESOURCE_UAV_BARRIER {
                    pResource: unsafe { std::mem::transmute_copy(resource.as_raw_ref()) },
                }),
            },
        }
    }

    pub(crate) fn as_type_raw(&self) -> D3D12_RESOURCE_BARRIER_TYPE {
        match self {
            BarrierType::Transition { .. } => D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
            BarrierType::Aliasing { .. } => D3D12_RESOURCE_BARRIER_TYPE_ALIASING,
            BarrierType::Uav { .. } => D3D12_RESOURCE_BARRIER_TYPE_UAV,
        }
    }
}
