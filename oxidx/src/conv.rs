mod enums;
mod flags;
mod structs;

use std::mem::ManuallyDrop;

use compact_str::CompactString;
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::*,
        Graphics::{
            Direct3D::*,
            Direct3D12::*,
            Dxgi::{Common::*, *},
        },
    },
};

use crate::{
    adapter::{AdapterDesc, AdapterFlags},
    error::DxError,
    resources::{BarrierType, ResourceBarrier, VertexBufferView},
    swapchain::{Rational, SwapchainDesc, SwapchainFullscreenDesc},
    types::*,
    HasInterface,
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
            flags: AdapterFlags::from_bits(value.Flags).unwrap_or(AdapterFlags::empty()),
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

impl VertexBufferView {
    pub(crate) fn as_raw(&self) -> D3D12_VERTEX_BUFFER_VIEW {
        D3D12_VERTEX_BUFFER_VIEW {
            BufferLocation: self.buffer_location,
            SizeInBytes: self.size_in_bytes,
            StrideInBytes: self.stride_in_bytes,
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

impl From<windows::core::Error> for DxError {
    fn from(value: windows::core::Error) -> Self {
        match value.code() {
            D3D12_ERROR_ADAPTER_NOT_FOUND => DxError::AdapterNotFound,
            D3D12_ERROR_DRIVER_VERSION_MISMATCH => DxError::DriverVersionMismatch,
            E_FAIL => DxError::Fail(value.message()),
            E_INVALIDARG => DxError::InvalidArgs,
            E_OUTOFMEMORY => DxError::Oom,
            E_NOTIMPL => DxError::NotImpl,
            _ => DxError::Dxgi(value.message()),
        }
    }
}
