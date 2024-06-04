use std::ops::Range;

use windows::{
    core::{Interface, Param},
    Win32::Graphics::Direct3D12::*,
};

use crate::{
    create_type, error::DxError, impl_trait, misc::Format, prelude::SampleDesc, HasInterface,
};

pub trait ResourceInterface:
    for<'a> HasInterface<Raw: Interface, RawRef<'a>: Param<ID3D12Resource>>
{
    //TODO: Type for pointer
    fn map(&self, subresource: u32, read_range: Option<Range<usize>>) -> Result<*mut (), DxError>;
    fn unmap(&self, subresource: u32, written_range: Option<Range<usize>>);
    fn get_gpu_virtual_address(&self) -> u64;
}

create_type! { Resource wrap ID3D12Resource }

impl_trait! {
    impl ResourceInterface =>
    Resource;

    fn map(&self, subresource: u32, read_range: Option<Range<usize>>) -> Result<*mut (), DxError> {
        let mut ptr = std::ptr::null_mut();
        let range = read_range.map(|r| D3D12_RANGE {
            Begin: r.start,
            End: r.end,
        });

        unsafe {
            self.0
                .Map(
                    subresource,
                    range.as_ref().map(|r| r as *const _),
                    Some(&mut ptr),
                )
                .map_err(|_| DxError::Dummy)?;
        }

        Ok(ptr as *mut ())
    }

    fn unmap(&self, subresource: u32, written_range: Option<Range<usize>>) {
        let range = written_range.map(|r| D3D12_RANGE {
            Begin: r.start,
            End: r.end,
        });

        unsafe {
            self.0
                .Unmap(subresource, range.as_ref().map(|r| r as *const _));
        }
    }

    fn get_gpu_virtual_address(&self) -> u64 {
        unsafe {
            self.0.GetGPUVirtualAddress()
        }
    }
}

#[derive(Clone, Debug)]
pub struct Barrier<'a> {
    pub r#type: BarrierType<'a>,
    pub flags: BarrierFlags,
}

#[derive(Clone, Debug)]
pub enum BarrierType<'a> {
    Transition {
        resource: &'a Resource,
        subresource: u32,
        before: ResourceState,
        after: ResourceState,
    },
    Aliasing {
        before: &'a Resource,
        after: &'a Resource,
    },
    Uav {
        resource: &'a Resource,
    },
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct BarrierFlags: i32 {
        const BeginOnly = D3D12_RESOURCE_BARRIER_FLAG_BEGIN_ONLY.0;
        const EndOnly = D3D12_RESOURCE_BARRIER_FLAG_END_ONLY.0;
    }
}

#[derive(Debug, Clone)]
pub struct RenderTargetViewDesc {
    pub format: Format,
    pub dimension: ViewDimension,
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct ResourceState: i32 {
        const Present = D3D12_RESOURCE_STATE_PRESENT.0;
        const RenderTarget = D3D12_RESOURCE_STATE_RENDER_TARGET.0;
        const GenericRead = D3D12_RESOURCE_STATE_GENERIC_READ.0;
    }
}

#[derive(Debug, Clone)]
pub enum ViewDimension {
    Buffer {
        first_element: u64,
        num_elements: u32,
    },
    Tex1D {
        mip_slice: u32,
    },
    Tex2D {
        mip_slice: u32,
        plane_slice: u32,
    },
    Tex3D {
        mip_slice: u32,
        first_w_slice: u32,
        w_size: u32,
    },
    ArrayTex1D {
        mip_slice: u32,
        first_array_slice: u32,
        array_size: u32,
    },
    ArrayTex2D {
        mip_slice: u32,
        plane_slice: u32,
        first_array_slice: u32,
        array_size: u32,
    },
    Tex2DMs,
    Array2DMs {
        first_array_slice: u32,
        array_size: u32,
    },
}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum ResourceDimension {
    Unknown = D3D12_RESOURCE_DIMENSION_UNKNOWN.0,
    Buffer = D3D12_RESOURCE_DIMENSION_BUFFER.0,
    Texture1D = D3D12_RESOURCE_DIMENSION_TEXTURE1D.0,
    Texture2D = D3D12_RESOURCE_DIMENSION_TEXTURE2D.0,
    Texture3d = D3D12_RESOURCE_DIMENSION_TEXTURE3D.0,
}

#[derive(Clone, Debug)]
pub struct ResourceDesc {
    pub dimension: ResourceDimension,
    pub alignment: u64,
    pub width: u64,
    pub height: u32,
    pub depth_or_array_size: u16,
    pub mip_levels: u16,
    pub sample_desc: SampleDesc,
    pub format: Format,
    pub layout: TextureLayout,
    pub flags: ResourceFlags,
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct ResourceFlags: i32 {
        const AllowRenderTarget = D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET.0;
        const AllowDepthStencil = D3D12_RESOURCE_FLAG_ALLOW_DEPTH_STENCIL.0;
        const AllowUnorderedAccess = D3D12_RESOURCE_FLAG_ALLOW_UNORDERED_ACCESS.0;
        const DenyShaderResource = D3D12_RESOURCE_FLAG_DENY_SHADER_RESOURCE.0;
        const AllowCrossAdapter = D3D12_RESOURCE_FLAG_ALLOW_CROSS_ADAPTER.0;
        const AllowSimultaneousAccess = D3D12_RESOURCE_FLAG_ALLOW_SIMULTANEOUS_ACCESS.0;
        const VideoDecodeReferenceOnly = D3D12_RESOURCE_FLAG_VIDEO_DECODE_REFERENCE_ONLY.0;
        const VideoEncodeReferenceOnly = D3D12_RESOURCE_FLAG_VIDEO_ENCODE_REFERENCE_ONLY.0;
        const RaytracingAccelerationStructure = D3D12_RESOURCE_FLAG_RAYTRACING_ACCELERATION_STRUCTURE.0;
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum TextureLayout {
    Unknown = D3D12_TEXTURE_LAYOUT_UNKNOWN.0,
    RowMajor = D3D12_TEXTURE_LAYOUT_ROW_MAJOR.0,
    UndefinedSwizzle64Kb = D3D12_TEXTURE_LAYOUT_64KB_UNDEFINED_SWIZZLE.0,
    StandardSwizzle64Kb = D3D12_TEXTURE_LAYOUT_64KB_STANDARD_SWIZZLE.0,
}

#[derive(Clone, Debug)]
pub struct VertexBufferView {
    pub buffer_location: u64,
    pub stride_in_bytes: u32,
    pub size_in_bytes: u32,
}
