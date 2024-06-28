use super::*;

/// Describes a command queue.
///
/// For more information: [`D3D12_COMMAND_QUEUE_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_command_queue_desc)
#[derive(Clone, Copy, Debug, Default)]
pub struct CommandQueueDesc {
    /// Specifies one member of [`CommandListType`].
    pub r#type: CommandListType,

    /// The priority for the command queue, as a [`CommandQueuePriority`] enumeration constant to select normal or high priority.
    pub priority: CommandQueuePriority,

    /// Specifies any flags from the [`CommandQueueFlags`] enumeration.
    pub flags: CommandQueueFlags,

    /// For single GPU operation, set this to zero. If there are multiple GPU nodes, set a bit to identify the node (the device's physical adapter) to which the command queue applies. Each bit in the mask corresponds to a single node. Only 1 bit must be set.
    pub node_mask: u32,
}

/// Describes a CPU descriptor handle.
///
/// For more information: [`D3D12_CPU_DESCRIPTOR_HANDLE structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_cpu_descriptor_handle)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CpuDescriptorHandle(pub(crate) usize);

impl CpuDescriptorHandle {
    pub fn offset(&self, offset: usize) -> Self {
        Self(self.0 + offset)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ConservativeRasterizationTier {}

#[derive(Clone, Copy, Debug, Default)]
pub struct CrossNodeSharingTier {}

/// Describes a GPU descriptor handle.
///
/// For more information: [`D3D12_GPU_DESCRIPTOR_HANDLE structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_gpu_descriptor_handle)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpuDescriptorHandle(pub(crate) usize);

impl GpuDescriptorHandle {
    pub fn offset(&self, offset: usize) -> Self {
        Self(self.0 + offset)
    }
}

/// Describes the descriptor heap.
///
/// For more information: [`D3D12_DESCRIPTOR_HEAP_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_descriptor_heap_desc)
#[derive(Clone, Copy, Debug, Default)]
pub struct DescriptorHeapDesc {
    /// A [`DescriptorHeapType`]-typed value that specifies the types of descriptors in the heap.
    pub r#type: DescriptorHeapType,

    /// The number of descriptors in the heap.
    pub num: u32,

    /// A combination of [`DescriptorHeapFlags]-typed values that are combined by using a bitwise OR operation. The resulting value specifies options for the heap.
    pub flags: DescriptorHeapFlags,

    /// For single-adapter operation, set this to zero. If there are multiple adapter nodes, set a bit to identify the node (one of the device's physical adapters) to which the descriptor heap applies. Each bit in the mask corresponds to a single node. Only one bit must be set.
    pub node_mask: u32,
}

/// Describes a heap.
///
/// For more information: [`D3D12_HEAP_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_heap_desc)
#[derive(Clone, Copy, Debug)]
pub struct HeapDesc {
    /// The size, in bytes, of the heap. To avoid wasting memory, applications should pass size values which are multiples of the effective Alignment;
    /// but non-aligned size is also supported, for convenience.
    /// To find out how large a heap must be to support textures with undefined layouts and adapter-specific sizes, call [`get_resource_allocation_info`](crate::device::DeviceInterface::get_resource_allocation_info)
    pub size: u64,

    /// A [`HeapProperties`] structure that describes the heap properties.
    pub props: HeapProperties,

    /// The alignment value for the heap.
    pub alignment: HeapAlignment,

    /// A combination of [`HeapFlags`]-typed values that are combined by using a bitwise-OR operation.
    /// The resulting value identifies heap options. When creating heaps to support adapters with resource heap tier 1, an application must choose some flags.
    pub flags: HeapFlags,
}

/// Describes heap properties.
///
/// For more information: [`D3D12_HEAP_PROPERTIES structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_heap_properties)
#[derive(Clone, Copy, Debug)]
pub struct HeapProperties {
    /// A [`HeapType`]-typed value that specifies the type of heap.
    pub r#type: HeapType,

    /// A [`CpuPageProperty`]-typed value that specifies the CPU-page properties for the heap.
    pub cpu_page_propery: CpuPageProperty,

    /// A [`MemoryPool`]-typed value that specifies the memory pool for the heap.
    pub memory_pool_preference: MemoryPool,

    /// For multi-adapter operation, this indicates the node where the resource should be created.
    ///
    /// Exactly one bit of this UINT must be set.
    ///
    /// Passing zero is equivalent to passing one, in order to simplify the usage of single-GPU adapters.
    pub creation_node_mask: u32,

    /// For multi-adapter operation, this indicates the set of nodes where the resource is visible.
    ///
    /// VisibleNodeMask must have the same bit set that is set in CreationNodeMask. VisibleNodeMask can also have additional bits set for cross-node resources, but doing so can potentially reduce performance for resource accesses, so you should do so only when needed.
    ///
    /// Passing zero is equivalent to passing one, in order to simplify the usage of single-GPU adapters.
    pub visible_node_mask: u32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MinPrecisionSupport {}

#[derive(Clone, Copy, Debug, Default)]
pub struct ResourceBindingTier {}

#[derive(Clone, Copy, Debug, Default)]
pub struct ResourceHeapTier {}

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

#[derive(Clone, Copy, Debug, Default)]
pub struct TiledResourcesTier {}
