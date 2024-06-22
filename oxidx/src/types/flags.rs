use windows::Win32::Graphics::Direct3D12::*;

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
