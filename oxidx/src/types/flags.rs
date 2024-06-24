use windows::Win32::Graphics::Direct3D12::*;

bitflags::bitflags! {
    /// Specifies flags to be used when creating a command queue.
    ///
    /// Empty flag - Indicates a default command queue.
    ///
    /// For more information: [`D3D12_COMMAND_QUEUE_FLAGS enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_command_queue_flags)
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct CommandQueueFlags: i32 {
        /// Indicates that the GPU timeout should be disabled for this command queue.
        const DisableGpuTimeout = D3D12_COMMAND_QUEUE_FLAG_DISABLE_GPU_TIMEOUT.0;
    }
}

bitflags::bitflags! {
    /// Describes the level of GPU-based validation to perform at runtime.
    ///
    /// Empty flag - Default behavior; resource states, descriptors, and descriptor tables are all validated.
    ///
    /// For more information: [`D3D12_GPU_BASED_VALIDATION_FLAGS enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/ne-d3d12sdklayers-d3d12_gpu_based_validation_flags)
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct GpuBasedValidationFlags: i32 {
        /// Indicates that the GPU timeout should be disabled for this command queue.
        const DisableStateTracking = D3D12_GPU_BASED_VALIDATION_FLAGS_DISABLE_STATE_TRACKING.0;
    }
}

bitflags::bitflags! {
    /// Specifies a range of tile mappings.
    ///
    /// Empty flag - No tile-mapping flags are specified.
    ///
    /// For more information: [`D3D12_TILE_RANGE_FLAGS enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_tile_range_flags)
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct TileRangeFlags: i32 {
        /// The tile range is NULL.
        const Null = D3D12_TILE_RANGE_FLAG_NULL.0;

        /// Skip the tile range.
        const Skip = D3D12_TILE_RANGE_FLAG_SKIP.0;

        /// Reuse a single tile in the tile range.
        const ReuseSingleTile = D3D12_TILE_RANGE_FLAG_REUSE_SINGLE_TILE.0;
    }
}
