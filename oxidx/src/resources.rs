use std::ops::Range;

use windows::{
    core::{Interface, Param},
    Win32::Graphics::Direct3D12::{
        ID3D12Resource, D3D12_RANGE, D3D12_RESOURCE_BARRIER_FLAG_BEGIN_ONLY,
        D3D12_RESOURCE_BARRIER_FLAG_END_ONLY, D3D12_RESOURCE_STATE_PRESENT, D3D12_RESOURCE_STATE_RENDER_TARGET,
    },
};

use crate::{create_type, error::DxError, impl_trait, misc::Format, HasInterface};

pub trait ResourceInterface:
    for<'a> HasInterface<Raw: Interface, RawRef<'a>: Param<ID3D12Resource>>
{
    //TODO: Type for pointer
    fn map(&self, subresource: u32, read_range: Option<Range<usize>>) -> Result<*mut (), DxError>;
    fn unmap(&self, subresource: u32, written_range: Option<Range<usize>>);
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
