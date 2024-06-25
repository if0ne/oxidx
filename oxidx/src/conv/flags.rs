use windows::Win32::Graphics::Direct3D12::*;

use crate::types::CommandQueueFlags;

use super::{DescriptorHeapFlags, FenceFlags, GpuBasedValidationFlags, HeapFlags, TileRangeFlags};

impl CommandQueueFlags {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_COMMAND_QUEUE_FLAGS {
        D3D12_COMMAND_QUEUE_FLAGS(self.bits())
    }
}

impl DescriptorHeapFlags {
    pub(crate) fn as_raw(&self) -> D3D12_DESCRIPTOR_HEAP_FLAGS {
        D3D12_DESCRIPTOR_HEAP_FLAGS(self.bits())
    }
}

impl From<D3D12_DESCRIPTOR_HEAP_FLAGS> for DescriptorHeapFlags {
    #[inline]
    fn from(value: D3D12_DESCRIPTOR_HEAP_FLAGS) -> Self {
        Self::from_bits(value.0).unwrap()
    }
}

impl FenceFlags {
    pub(crate) fn as_raw(&self) -> D3D12_FENCE_FLAGS {
        D3D12_FENCE_FLAGS(self.bits())
    }
}

impl From<D3D12_FENCE_FLAGS> for FenceFlags {
    #[inline]
    fn from(value: D3D12_FENCE_FLAGS) -> Self {
        Self::from_bits(value.0).unwrap()
    }
}

impl GpuBasedValidationFlags {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_GPU_BASED_VALIDATION_FLAGS {
        D3D12_GPU_BASED_VALIDATION_FLAGS(self.bits())
    }
}

impl From<D3D12_COMMAND_QUEUE_FLAGS> for CommandQueueFlags {
    #[inline]
    fn from(value: D3D12_COMMAND_QUEUE_FLAGS) -> Self {
        Self::from_bits(value.0).unwrap()
    }
}

impl TileRangeFlags {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_TILE_RANGE_FLAGS {
        D3D12_TILE_RANGE_FLAGS(self.bits())
    }
}

impl HeapFlags {
    pub(crate) fn as_raw(&self) -> D3D12_HEAP_FLAGS {
        D3D12_HEAP_FLAGS(self.bits())
    }
}

impl From<D3D12_HEAP_FLAGS> for HeapFlags {
    #[inline]
    fn from(value: D3D12_HEAP_FLAGS) -> Self {
        Self::from_bits(value.0).unwrap()
    }
}
