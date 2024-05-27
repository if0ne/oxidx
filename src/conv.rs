use compact_str::CompactString;
use windows::Win32::Graphics::{
    Direct3D::D3D_FEATURE_LEVEL,
    Direct3D12::{
        D3D12_COMMAND_LIST_TYPE, D3D12_COMMAND_QUEUE_DESC, D3D12_COMMAND_QUEUE_FLAGS,
        D3D12_FENCE_FLAGS,
    },
    Dxgi::{
        Common::{
            DXGI_ALPHA_MODE, DXGI_FORMAT, DXGI_MODE_SCALING, DXGI_MODE_SCANLINE_ORDER,
            DXGI_RATIONAL, DXGI_SAMPLE_DESC,
        },
        DXGI_ADAPTER_DESC1, DXGI_SCALING, DXGI_SWAP_CHAIN_DESC1, DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        DXGI_SWAP_EFFECT, DXGI_USAGE,
    },
};

use crate::{
    adapter::{AdapterDesc, AdapterFlags, Luid},
    command_queue::CommandQueueDesc,
    factory::FeatureLevel,
    misc::{
        AlphaMode, CommandListType, Format, FrameBufferUsage, Scaling, ScalingMode,
        ScanlineOrdering, SwapEffect,
    },
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
