/// Describes the size of a tiled region.
///
/// For more information: [`D3D12_TILE_REGION_SIZE structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_tile_region_size)
#[derive(Clone, Copy, Debug)]
pub struct TileRegionSize {
    /// The number of tiles in the tiled region.
    pub num_tiles: u32,

    /// Specifies whether the runtime uses the **width**, **height**, and **depth** members to define the region.
    ///
    /// If **true**, the runtime uses the **width**, **height**, and **depth** members to define the region.
    /// In this case, **num_tiles** should be equal to **width * height * depth**.
    ///
    /// If **false**, the runtime ignores the **width**, **height**, and **depth** members and uses the **num_tiles** member to
    /// traverse tiles in the resource linearly across x, then y, then z (as applicable) and then spills over mipmaps/arrays in subresource order.
    /// For example, use this technique to map an entire resource at once.
    ///
    /// Regardless of whether you specify **true** or **false** for **use_box**, you use a [`TiledResourceCoordinate`] structure to specify
    /// the starting location for the region within the resource as a separate parameter outside of this structure by using x, y, and z coordinates.
    ///
    /// When the region includes mipmaps that are packed with nonstandard tiling, **use_box** must be **false** because
    /// tile dimensions are not standard and the app only knows a count of how many tiles are consumed by the packed area,
    /// which is per array slice. The corresponding (separate) starting location parameter uses x to offset into the flat range of tiles in this case,
    /// and y and z coordinates must each be 0.
    pub use_box: bool,

    /// The width of the tiled region, in tiles. Used for buffer and 1D, 2D, and 3D textures.
    pub width: u32,

    /// The height of the tiled region, in tiles. Used for 2D and 3D textures.
    pub height: u16,

    /// The depth of the tiled region, in tiles. Used for 3D textures or arrays.
    /// For arrays, used for advancing in depth jumps to next slice of same mipmap size, which isn't contiguous in the subresource counting space
    /// if there are multiple mipmaps.
    pub depth: u16,
}

/// Describes the coordinates of a tiled resource.
///
/// For more information: [`D3D12_TILED_RESOURCE_COORDINATE structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_tiled_resource_coordinate)
#[derive(Clone, Copy, Debug)]
pub struct TiledResourceCoordinate {
    /// The x-coordinate of the tiled resource.
    pub x: u32,

    /// The y-coordinate of the tiled resource.
    pub y: u32,

    /// The z-coordinate of the tiled resource.
    pub z: u32,

    /// The index of the subresource for the tiled resource.
    ///
    /// For mipmaps that use nonstandard tiling, or are packed, or both use nonstandard tiling and are packed,
    /// any subresource value that indicates any of the packed mipmaps all refer to the same tile.
    /// Additionally, the X coordinate is used to indicate a tile within the packed mip region, rather than a logical region of a single subresource.
    /// The Y and Z coordinates must be zero.
    pub subresource: u32,
}
