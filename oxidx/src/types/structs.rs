use crate::pso::{BlobInterface, RootSignatureInterface};

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

/// Describes the arguments (parameters) of a command signature.
///
/// For more information: [`D3D12_COMMAND_SIGNATURE_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_command_signature_desc)
#[derive(Debug)]
pub struct CommandSignatureDesc<'a> {
    /// Specifies the size of each command in the drawing buffer, in bytes.
    pub byte_stride: u32,

    /// An array of [`IndirectArgumentDesc`] enumeration, containing details of the arguments, including whether the argument is a vertex buffer, constant, constant buffer view, shader resource view, or unordered access view.
    pub argument_descs: &'a [IndirectArgumentDesc],

    /// For single GPU operation, set this to zero.
    /// If there are multiple GPU nodes, set bits to identify the nodes (the device's physical adapters) for which the command signature is to apply.
    /// Each bit in the mask corresponds to a single node.
    pub node_mask: u32,
}

/// Describes a compute pipeline state object.
///
/// For more information: [`D3D12_COMPUTE_PIPELINE_STATE_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_compute_pipeline_state_desc)
#[derive(Debug)]
pub struct ComputePipelineStateDesc<'a, RS: RootSignatureInterface, B: BlobInterface> {
    /// A reference to the [`RootSignatureInterface`] object.
    pub root_signature: &'a RS,

    /// /// A reference to the [`BlobInterface`] object that contains compute shader.
    pub cs: &'a B,

    /// For single GPU operation, set this to zero.
    /// If there are multiple GPU nodes, set bits to identify the nodes (the device's physical adapters) for which the compute pipeline state is to apply.
    /// Each bit in the mask corresponds to a single node.
    pub node_mask: u32,

    /// A cached pipeline state object, as a [`CachedPipelineState`] structure. `cached_blob` and `cached_blob_size_in_bytes` may be set to None and 0 respectively.
    pub cached_pso: Option<&'a B>,

    /// A [`PipelineStateFlags`] enumeration constant such as for "tool debug".
    pub flags: PipelineStateFlags,
}

/// Describes a constant buffer to view.
///
/// For more information: [`D3D12_CONSTANT_BUFFER_VIEW_DESC structure `](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_constant_buffer_view_desc)
#[derive(Clone, Copy, Debug)]
pub struct ConstantBufferViewDesc {
    // GPU virtual address
    pub buffer_location: u64,

    /// The size in bytes of the constant buffer.
    pub size_in_bytes: u32,
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

/// Describes the subresources of a texture that are accessible from a depth-stencil view.
///
/// For more information: [`D3D12_DEPTH_STENCIL_VIEW_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_depth_stencil_view_desc)
#[derive(Clone, Copy, Debug)]
pub struct DepthStencilViewDesc {
    pub format: Format,
    pub view_dimension: DsvDimension,
    pub flags: DsvFlags,
}

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

/// Describes a resource, such as a texture. This structure is used extensively.
///
/// For more information: [`D3D12_RESOURCE_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_resource_desc)
#[derive(Clone, Debug)]
pub struct ResourceDesc {
    /// One member of [`ResourceDimension`], specifying the dimensions of the resource.
    pub dimension: ResourceDimension,

    /// Specifies the alignment.
    pub alignment: u64,

    /// Specifies the width of the resource.
    pub width: u64,

    /// Specifies the height of the resource.
    pub height: u32,

    /// Specifies the depth of the resource, if it is 3D, or the array size if it is an array of 1D or 2D resources.
    pub depth_or_array_size: u16,

    /// Specifies the number of MIP levels.
    pub mip_levels: u16,

    /// Specifies a [`SampleDesc`] structure.
    pub sample_desc: SampleDesc,

    /// Specifies one member of [`Format`].
    pub format: Format,

    /// Specifies one member of [`TextureLayout`].
    pub layout: TextureLayout,

    /// Bitwise-OR'd flags, as [`ResourceFlags`] enumeration constants.
    pub flags: ResourceFlags,
}

/// Describes multi-sampling parameters for a resource.
///
/// For more information: [`DXGI_SAMPLE_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/dxgicommon/ns-dxgicommon-dxgi_sample_desc)
#[derive(Debug, Default, Clone, Copy)]
pub struct SampleDesc {
    /// The number of multisamples per pixel.
    pub count: u32,

    /// The image quality level. The higher the quality, the lower the performance.
    pub quality: u32,
}

/// Describes the size of a tiled region.
///
/// For more information: [`D3D12_TILE_REGION_SIZE structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_tile_region_size)
#[derive(Clone, Copy, Debug)]
pub struct TileRegionSize {
    /// The number of tiles in the tiled region.
    pub num_tiles: u32,

    /// Specifies whether the runtime uses the **width**, **height**, and **depth** members to define the region.
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
    pub subresource: u32,
}
