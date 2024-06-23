use windows::Win32::Graphics::Direct3D12::*;

use crate::types::CommandQueueFlags;

use super::TileRangeFlags;

impl CommandQueueFlags {
    pub(crate) fn as_raw(&self) -> D3D12_COMMAND_QUEUE_FLAGS {
        D3D12_COMMAND_QUEUE_FLAGS(self.bits())
    }
}

impl From<D3D12_COMMAND_QUEUE_FLAGS> for CommandQueueFlags {
    fn from(value: D3D12_COMMAND_QUEUE_FLAGS) -> Self {
        match value {
            D3D12_COMMAND_QUEUE_FLAG_DISABLE_GPU_TIMEOUT => CommandQueueFlags::DisableGpuTimeout,
            _ => unreachable!(),
        }
    }
}

impl TileRangeFlags {
    pub(crate) fn as_raw(&self) -> D3D12_TILE_RANGE_FLAGS {
        D3D12_TILE_RANGE_FLAGS(self.bits())
    }
}
