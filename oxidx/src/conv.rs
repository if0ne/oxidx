use compact_str::CompactString;
use windows::Win32::{
    Foundation::RECT,
    Graphics::{
        Direct3D::D3D_FEATURE_LEVEL,
        Direct3D12::{
            D3D12_BUFFER_RTV, D3D12_COMMAND_LIST_TYPE, D3D12_COMMAND_QUEUE_DESC,
            D3D12_COMMAND_QUEUE_FLAGS, D3D12_CPU_DESCRIPTOR_HANDLE, D3D12_DESCRIPTOR_HEAP_DESC,
            D3D12_DESCRIPTOR_HEAP_FLAGS, D3D12_DESCRIPTOR_HEAP_TYPE,
            D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV, D3D12_DESCRIPTOR_HEAP_TYPE_DSV,
            D3D12_DESCRIPTOR_HEAP_TYPE_RTV, D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER, D3D12_FENCE_FLAGS,
            D3D12_RENDER_TARGET_VIEW_DESC, D3D12_RENDER_TARGET_VIEW_DESC_0, D3D12_ROOT_CONSTANTS,
            D3D12_ROOT_DESCRIPTOR, D3D12_ROOT_DESCRIPTOR_TABLE, D3D12_ROOT_PARAMETER,
            D3D12_ROOT_PARAMETER_0, D3D12_ROOT_PARAMETER_TYPE,
            D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS, D3D12_ROOT_PARAMETER_TYPE_CBV,
            D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE, D3D12_ROOT_PARAMETER_TYPE_SRV,
            D3D12_ROOT_PARAMETER_TYPE_UAV, D3D12_ROOT_SIGNATURE_FLAGS, D3D12_RTV_DIMENSION,
            D3D12_RTV_DIMENSION_BUFFER, D3D12_RTV_DIMENSION_TEXTURE1D,
            D3D12_RTV_DIMENSION_TEXTURE1DARRAY, D3D12_RTV_DIMENSION_TEXTURE2D,
            D3D12_RTV_DIMENSION_TEXTURE2DARRAY, D3D12_RTV_DIMENSION_TEXTURE2DMS,
            D3D12_RTV_DIMENSION_TEXTURE2DMSARRAY, D3D12_RTV_DIMENSION_TEXTURE3D,
            D3D12_SHADER_VISIBILITY, D3D12_STATIC_SAMPLER_DESC, D3D12_TEX1D_ARRAY_RTV,
            D3D12_TEX1D_RTV, D3D12_TEX2DMS_ARRAY_RTV, D3D12_TEX2DMS_RTV, D3D12_TEX2D_ARRAY_RTV,
            D3D12_TEX2D_RTV, D3D12_TEX3D_RTV, D3D12_VIEWPORT, D3D_ROOT_SIGNATURE_VERSION,
        },
        Dxgi::{
            Common::{
                DXGI_ALPHA_MODE, DXGI_FORMAT, DXGI_MODE_SCALING, DXGI_MODE_SCANLINE_ORDER,
                DXGI_RATIONAL, DXGI_SAMPLE_DESC,
            },
            DXGI_ADAPTER_DESC1, DXGI_SCALING, DXGI_SWAP_CHAIN_DESC1,
            DXGI_SWAP_CHAIN_FULLSCREEN_DESC, DXGI_SWAP_EFFECT, DXGI_USAGE,
        },
    },
};

use crate::{
    adapter::{AdapterDesc, AdapterFlags, Luid},
    command_queue::CommandQueueDesc,
    factory::FeatureLevel,
    heap::{CpuDescriptorHandle, DescriptorHeapDesc, DescriptorHeapFlags, DescriptorHeapType},
    misc::{
        AlphaMode, CommandListType, Format, FrameBufferUsage, Rect, Scaling, ScalingMode,
        ScanlineOrdering, SwapEffect, Viewport,
    },
    pso::{
        RootParameter, RootParameterType, RootSignatureFlags, RootSignatureVersion,
        ShaderVisibility, StaticSamplerDesc,
    },
    resources::{RenderTargetViewDesc, ViewDimension},
    swapchain::{Rational, SampleDesc, SwapchainDesc, SwapchainFullscreenDesc},
    sync::FenceFlags,
};

impl SwapchainDesc {
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

impl Format {
    pub(crate) fn as_raw(&self) -> DXGI_FORMAT {
        DXGI_FORMAT(*self as i32)
    }
}

impl SampleDesc {
    pub(crate) fn as_raw(&self) -> DXGI_SAMPLE_DESC {
        DXGI_SAMPLE_DESC {
            Count: self.count,
            Quality: self.quality,
        }
    }
}

impl FrameBufferUsage {
    pub(crate) fn as_raw(&self) -> DXGI_USAGE {
        DXGI_USAGE(self.bits())
    }
}

impl Scaling {
    pub(crate) fn as_raw(&self) -> DXGI_SCALING {
        DXGI_SCALING(*self as i32)
    }
}

impl SwapEffect {
    pub(crate) fn as_raw(&self) -> DXGI_SWAP_EFFECT {
        DXGI_SWAP_EFFECT(*self as i32)
    }
}

impl AlphaMode {
    pub(crate) fn as_raw(&self) -> DXGI_ALPHA_MODE {
        DXGI_ALPHA_MODE(*self as i32)
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

impl Rational {
    pub(crate) fn as_raw(&self) -> DXGI_RATIONAL {
        DXGI_RATIONAL {
            Numerator: self.numerator,
            Denominator: self.denominator,
        }
    }
}

impl ScanlineOrdering {
    pub(crate) fn as_raw(&self) -> DXGI_MODE_SCANLINE_ORDER {
        DXGI_MODE_SCANLINE_ORDER(*self as i32)
    }
}

impl ScalingMode {
    pub(crate) fn as_raw(&self) -> DXGI_MODE_SCALING {
        DXGI_MODE_SCALING(*self as i32)
    }
}

impl FeatureLevel {
    pub(crate) fn as_raw(&self) -> D3D_FEATURE_LEVEL {
        D3D_FEATURE_LEVEL(*self as i32)
    }
}

impl CommandListType {
    pub(crate) fn as_raw(&self) -> D3D12_COMMAND_LIST_TYPE {
        D3D12_COMMAND_LIST_TYPE(*self as i32)
    }
}

impl FenceFlags {
    pub(crate) fn as_raw(&self) -> D3D12_FENCE_FLAGS {
        D3D12_FENCE_FLAGS(self.bits())
    }
}

impl CommandQueueDesc {
    pub(crate) fn as_raw(&self) -> D3D12_COMMAND_QUEUE_DESC {
        D3D12_COMMAND_QUEUE_DESC {
            Type: self.r#type.as_raw(),
            Priority: self.priority,
            Flags: D3D12_COMMAND_QUEUE_FLAGS(self.flags.bits()),
            NodeMask: self.node_mask,
        }
    }
}

impl DescriptorHeapDesc {
    pub(crate) fn as_raw(&self) -> D3D12_DESCRIPTOR_HEAP_DESC {
        D3D12_DESCRIPTOR_HEAP_DESC {
            Type: self.r#type.as_raw(),
            NumDescriptors: self.num,
            Flags: self.flags.as_raw(),
            NodeMask: self.node_mask,
        }
    }
}

impl DescriptorHeapFlags {
    pub(crate) fn as_raw(&self) -> D3D12_DESCRIPTOR_HEAP_FLAGS {
        D3D12_DESCRIPTOR_HEAP_FLAGS(self.bits())
    }
}

impl DescriptorHeapType {
    pub(crate) fn as_raw(&self) -> D3D12_DESCRIPTOR_HEAP_TYPE {
        match self {
            DescriptorHeapType::Rtv => D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
            DescriptorHeapType::Dsv => D3D12_DESCRIPTOR_HEAP_TYPE_DSV,
            DescriptorHeapType::CbvSrvUav => D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV,
            DescriptorHeapType::Sampler => D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER,
        }
    }
}

impl From<DXGI_ADAPTER_DESC1> for AdapterDesc {
    fn from(value: DXGI_ADAPTER_DESC1) -> Self {
        AdapterDesc {
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
            flags: AdapterFlags::from_bits(value.Flags).unwrap_or(AdapterFlags::None),
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

impl CpuDescriptorHandle {
    pub(crate) fn as_raw(&self) -> D3D12_CPU_DESCRIPTOR_HANDLE {
        D3D12_CPU_DESCRIPTOR_HANDLE { ptr: self.0 }
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

impl ViewDimension {
    pub(crate) fn as_type_raw(&self) -> D3D12_RTV_DIMENSION {
        match self {
            ViewDimension::Buffer { .. } => D3D12_RTV_DIMENSION_BUFFER,
            ViewDimension::Tex1D { .. } => D3D12_RTV_DIMENSION_TEXTURE1D,
            ViewDimension::Tex2D { .. } => D3D12_RTV_DIMENSION_TEXTURE2D,
            ViewDimension::Tex3D { .. } => D3D12_RTV_DIMENSION_TEXTURE3D,
            ViewDimension::ArrayTex1D { .. } => D3D12_RTV_DIMENSION_TEXTURE1DARRAY,
            ViewDimension::ArrayTex2D { .. } => D3D12_RTV_DIMENSION_TEXTURE2DARRAY,
            ViewDimension::Tex2DMs => D3D12_RTV_DIMENSION_TEXTURE2DMS,
            ViewDimension::Array2DMs { .. } => D3D12_RTV_DIMENSION_TEXTURE2DMSARRAY,
        }
    }

    pub(crate) fn as_raw(&self) -> D3D12_RENDER_TARGET_VIEW_DESC_0 {
        match self {
            ViewDimension::Buffer {
                first_element,
                num_elements,
            } => D3D12_RENDER_TARGET_VIEW_DESC_0 {
                Buffer: D3D12_BUFFER_RTV {
                    FirstElement: *first_element,
                    NumElements: *num_elements,
                },
            },
            ViewDimension::Tex1D { mip_slice } => D3D12_RENDER_TARGET_VIEW_DESC_0 {
                Texture1D: D3D12_TEX1D_RTV {
                    MipSlice: *mip_slice,
                },
            },
            ViewDimension::Tex2D {
                mip_slice,
                plane_slice,
            } => D3D12_RENDER_TARGET_VIEW_DESC_0 {
                Texture2D: D3D12_TEX2D_RTV {
                    MipSlice: *mip_slice,
                    PlaneSlice: *plane_slice,
                },
            },
            ViewDimension::Tex3D {
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
            ViewDimension::ArrayTex1D {
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
            ViewDimension::ArrayTex2D {
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
            ViewDimension::Tex2DMs => D3D12_RENDER_TARGET_VIEW_DESC_0 {
                Texture2DMS: D3D12_TEX2DMS_RTV {
                    UnusedField_NothingToDefine: 0,
                },
            },
            ViewDimension::Array2DMs {
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

impl RootSignatureVersion {
    pub(crate) fn as_raw(&self) -> D3D_ROOT_SIGNATURE_VERSION {
        D3D_ROOT_SIGNATURE_VERSION(*self as i32)
    }
}

impl RootSignatureFlags {
    pub(crate) fn as_raw(&self) -> D3D12_ROOT_SIGNATURE_FLAGS {
        D3D12_ROOT_SIGNATURE_FLAGS(self.bits())
    }
}

impl StaticSamplerDesc {
    pub(crate) fn as_raw(&self) -> D3D12_STATIC_SAMPLER_DESC {
        todo!()
    }
}

impl<'a> RootParameter<'a> {
    pub(crate) fn as_raw(&self) -> D3D12_ROOT_PARAMETER {
        D3D12_ROOT_PARAMETER {
            ParameterType: self.r#type.as_type_raw(),
            Anonymous: self.r#type.as_raw(),
            ShaderVisibility: self.visibility.as_raw(),
        }
    }
}

impl ShaderVisibility {
    pub(crate) fn as_raw(&self) -> D3D12_SHADER_VISIBILITY {
        D3D12_SHADER_VISIBILITY(*self as i32)
    }
}

impl<'a> RootParameterType<'a> {
    pub(crate) fn as_type_raw(&self) -> D3D12_ROOT_PARAMETER_TYPE {
        match self {
            RootParameterType::DescriptorTable { .. } => D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE,
            RootParameterType::Constants { .. } => D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS,
            RootParameterType::Cbv { .. } => D3D12_ROOT_PARAMETER_TYPE_CBV,
            RootParameterType::Srv { .. } => D3D12_ROOT_PARAMETER_TYPE_SRV,
            RootParameterType::Uav { .. } => D3D12_ROOT_PARAMETER_TYPE_UAV,
        }
    }

    pub(crate) fn as_raw(&self) -> D3D12_ROOT_PARAMETER_0 {
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
            RootParameterType::DescriptorTable { ranges } => D3D12_ROOT_PARAMETER_0 {
                DescriptorTable: D3D12_ROOT_DESCRIPTOR_TABLE {
                    NumDescriptorRanges: ranges.len() as u32,
                    pDescriptorRanges: ranges.as_ptr() as *const _,
                },
            },
            RootParameterType::Constants {
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
