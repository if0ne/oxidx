use windows::Win32::Graphics::Direct3D12::*;

/// Defines priority levels for a command queue.
///
/// For more information: [`D3D12_COMMAND_QUEUE_PRIORITY enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_command_queue_priority)
#[derive(Debug, Default, Clone, Copy)]
#[repr(i32)]
pub enum CommandQueuePriority {
    #[default]
    /// Normal priority.
    Normal = D3D12_COMMAND_QUEUE_PRIORITY_NORMAL.0,

    /// High priority.
    High = D3D12_COMMAND_QUEUE_PRIORITY_HIGH.0,

    /// Global realtime priority.
    GlobalRealtime = D3D12_COMMAND_QUEUE_PRIORITY_GLOBAL_REALTIME.0,
}

/// Specifies the type of a command list.
///
/// For more information: [`D3D12_COMMAND_LIST_TYPE enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_command_list_type)
#[derive(Debug, Default, Clone, Copy)]
#[repr(i32)]
pub enum CommandListType {
    #[default]
    /// Specifies a command buffer that the GPU can execute. A direct command list doesn't inherit any GPU state.
    Direct = D3D12_COMMAND_LIST_TYPE_DIRECT.0,

    /// Specifies a command buffer that can be executed only directly via a direct command list.
    /// A bundle command list inherits all GPU state (except for the currently set pipeline state object and primitive topology).
    Bundle = D3D12_COMMAND_LIST_TYPE_BUNDLE.0,

    /// Specifies a command buffer for computing.
    Compute = D3D12_COMMAND_LIST_TYPE_COMPUTE.0,

    /// Specifies a command buffer for copying.
    Copy = D3D12_COMMAND_LIST_TYPE_COPY.0,

    /// Specifies a command buffer for video decoding.
    VideoDecode = D3D12_COMMAND_LIST_TYPE_VIDEO_DECODE.0,

    /// Specifies a command buffer for video processing.
    VideoProcess = D3D12_COMMAND_LIST_TYPE_VIDEO_PROCESS.0,

    /// Specifies a command buffer for video encoding.
    VideoEncode = D3D12_COMMAND_LIST_TYPE_VIDEO_ENCODE.0,
}

/// Specifies the CPU-page properties for the heap.
///
/// For more information: [`D3D12_CPU_PAGE_PROPERTY enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_cpu_page_property)
#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum CpuPageProperty {
    /// The CPU-page property is unknown.
    Unknown = D3D12_CPU_PAGE_PROPERTY_UNKNOWN.0,

    /// The CPU cannot access the heap, therefore no page properties are available.
    NotAvailable = D3D12_CPU_PAGE_PROPERTY_NOT_AVAILABLE.0,

    /// The CPU-page property is write-combined.
    WriteCombine = D3D12_CPU_PAGE_PROPERTY_WRITE_COMBINE.0,

    /// The CPU-page property is write-back.
    WriteBack = D3D12_CPU_PAGE_PROPERTY_WRITE_BACK.0,
}

/// Specifies a type of descriptor heap.
///
/// For more information: [`D3D12_DESCRIPTOR_HEAP_TYPE enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_descriptor_heap_type)
#[derive(Clone, Copy, Debug, Default)]
#[repr(i32)]
pub enum DescriptorHeapType {
    /// The descriptor heap for the render-target view.
    #[default]
    Rtv = D3D12_DESCRIPTOR_HEAP_TYPE_RTV.0,

    /// The descriptor heap for the depth-stencil view.
    Dsv = D3D12_DESCRIPTOR_HEAP_TYPE_DSV.0,

    /// The descriptor heap for the combination of constant-buffer, shader-resource, and unordered-access views.
    CbvSrvUav = D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV.0,

    /// The descriptor heap for the sampler.
    Sampler = D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER.0,
}

/// Heap alignment variants.
#[derive(Clone, Copy, Debug, Default)]
#[repr(u64)]
pub enum HeapAlignment {
    /// An alias for 64KB.
    #[default]
    Default = 0,

    /// Defined as 64KB.
    ResourcePlacement = D3D12_DEFAULT_RESOURCE_PLACEMENT_ALIGNMENT as u64,

    /// Defined as 4MB. An application must decide whether the heap will contain multi-sample anti-aliasing (MSAA), in which case, the application must choose this
    MsaaResourcePlacement = D3D12_DEFAULT_MSAA_RESOURCE_PLACEMENT_ALIGNMENT as u64,
}

/// Specifies the type of heap. When resident, heaps reside in a particular physical memory pool with certain CPU cache properties.
///
/// For more information: [`D3D12_HEAP_TYPE enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_heap_type)
#[derive(Clone, Copy, Debug, Default)]
#[repr(i32)]
pub enum HeapType {
    /// Specifies the default heap. This heap type experiences the most bandwidth for the GPU, but cannot provide CPU access.
    /// The GPU can read and write to the memory from this pool, and resource transition barriers may be changed.
    /// The majority of heaps and resources are expected to be located here, and are typically populated through resources in upload heaps.
    #[default]
    Default = D3D12_HEAP_TYPE_DEFAULT.0,

    /// Specifies a heap used for uploading. This heap type has CPU access optimized for uploading to the GPU,
    /// but does not experience the maximum amount of bandwidth for the GPU. This heap type is best for CPU-write-once, GPU-read-once data;
    /// but GPU-read-once is stricter than necessary. GPU-read-once-or-from-cache is an acceptable use-case for the data;
    /// but such usages are hard to judge due to differing GPU cache designs and sizes.
    /// If in doubt, stick to the GPU-read-once definition or profile the difference on many GPUs between copying the data to a _DEFAULT heap vs.
    /// reading the data from an _UPLOAD heap.
    Upload = D3D12_HEAP_TYPE_UPLOAD.0,

    /// Specifies a heap used for reading back. This heap type has CPU access optimized for reading data back from the GPU,
    /// but does not experience the maximum amount of bandwidth for the GPU. This heap type is best for GPU-write-once, CPU-readable data.
    /// The CPU cache behavior is write-back, which is conducive for multiple sub-cache-line CPU reads.
    Readback = D3D12_HEAP_TYPE_READBACK.0,

    /// Specifies a custom heap. The application may specify the memory pool and CPU cache properties directly, which can be useful for UMA optimizations,
    /// multi-engine, multi-adapter, or other special cases. To do so, the application is expected to understand the adapter architecture to make the right choice.
    Custom = D3D12_HEAP_TYPE_CUSTOM.0,

    /// TBD
    GpuUpload = D3D12_HEAP_TYPE_GPU_UPLOAD.0,
}

/// Specifies the memory pool for the heap.
///
/// For more information: [`D3D12_MEMORY_POOL enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_memory_pool)
#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum MemoryPool {
    /// The memory pool is unknown.
    Unknown = D3D12_MEMORY_POOL_UNKNOWN.0,

    /// The memory pool is L0.
    ///
    /// L0 is the physical system memory pool.
    ///
    /// When the adapter is discrete/NUMA, this pool has greater bandwidth for the CPU and less bandwidth for the GPU.
    ///
    /// When the adapter is UMA, this pool is the only one which is valid.
    L0 = D3D12_MEMORY_POOL_L0.0,

    /// The memory pool is L1.
    ///
    /// L1 is typically known as the physical video memory pool.
    ///
    /// L1 is only available when the adapter is discrete/NUMA, and has greater bandwidth for the GPU and cannot even be accessed by the CPU.
    ///
    /// When the adapter is UMA, this pool is not available.
    L1 = D3D12_MEMORY_POOL_L1.0,
}
