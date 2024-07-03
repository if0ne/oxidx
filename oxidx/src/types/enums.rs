use strum::FromRepr;
use windows::Win32::Graphics::{Direct3D::*, Direct3D12::*};

#[allow(unused_imports)]
use super::*;

/// Defines priority levels for a command queue.
///
/// For more information: [`D3D12_COMMAND_QUEUE_PRIORITY enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_command_queue_priority)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
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
#[derive(Clone, Copy, Debug, Default, FromRepr)]
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

/// Identifies the tier level of conservative rasterization.
///
/// For more information: [`D3D12_CONSERVATIVE_RASTERIZATION_TIER enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_conservative_rasterization_tier)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum ConservativeRasterizationTier {
    /// Conservative rasterization is not supported.
    #[default]
    NotSupported = D3D12_CONSERVATIVE_RASTERIZATION_TIER_NOT_SUPPORTED.0,

    /// Tier 1 enforces a maximum 1/2 pixel uncertainty region and does not support post-snap degenerates.
    /// This is good for tiled rendering, a texture atlas, light map generation and sub-pixel shadow maps.
    Tier1 = D3D12_CONSERVATIVE_RASTERIZATION_TIER_1.0,

    /// Tier 2 reduces the maximum uncertainty region to 1/256 and requires post-snap degenerates not be culled.
    /// This tier is helpful for CPU-based algorithm acceleration (such as voxelization).
    Tier2 = D3D12_CONSERVATIVE_RASTERIZATION_TIER_2.0,

    /// Tier 3 maintains a maximum 1/256 uncertainty region and adds support for inner input coverage. Inner input coverage adds the new value `SV_InnerCoverage` to
    /// High Level Shading Language (HLSL). This is a 32-bit scalar integer that can be specified on input to a pixel shader, and represents the underestimated conservative
    /// rasterization information (that is, whether a pixel is guaranteed-to-be-fully covered). This tier is helpful for occlusion culling.
    Tier3 = D3D12_CONSERVATIVE_RASTERIZATION_TIER_3.0,
}

/// Specifies the CPU-page properties for the heap.
///
/// For more information: [`D3D12_CPU_PAGE_PROPERTY enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_cpu_page_property)
#[derive(Clone, Copy, Debug, FromRepr)]
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

/// Specifies the level of sharing across nodes of an adapter, such as Tier 1 Emulated, Tier 1, or Tier 2.
///
/// For more information: [`D3D12_CROSS_NODE_SHARING_TIER enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_cross_node_sharing_tier)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum CrossNodeSharingTier {
    /// If an adapter has only 1 node, then cross-node sharing doesn't apply.
    #[default]
    NotSupported = D3D12_CROSS_NODE_SHARING_TIER_NOT_SUPPORTED.0,

    /// Tier 1 Emulated. Devices that set the [`CrossNodeSharingTier`] member of the [`Options`] structure to [`CrossNodeSharingTier::Tier1Emulated`] have Tier 1 support.
    ///
    /// However, drivers stage these copy operations through a driver-internal system memory allocation. This will cause these copy operations to consume time on the destination GPU as well as the source.
    Tier1Emulated = D3D12_CROSS_NODE_SHARING_TIER_1_EMULATED.0,

    /// Tier 1. Devices that set the [`CrossNodeSharingTier`] member of the [`Options`] structure to [`CrossNodeSharingTier::Tier1`] only support the following cross-node copy operations:
    /// * [GraphicsCommandList::copy_buffer_region](crate::command_list::GraphicsCommandList::copy_buffer_region)
    /// * [GraphicsCommandList::copy_texture_region](crate::command_list::GraphicsCommandList::copy_texture_region)
    /// * [GraphicsCommandList::copy_resource](crate::command_list::GraphicsCommandList::copy_resource)
    Tier1 = D3D12_CROSS_NODE_SHARING_TIER_1.0,

    /// Tier 2. Devices that set the [`CrossNodeSharingTier`] member of the [`Options`] structure to D3D12_CROSS_NODE_SHARING_TIER_2 support all operations across nodes, except for the following:
    /// * Render target views.
    /// * Depth stencil views.
    /// * UAV atomic operations. Similar to CPU/GPU interop, shaders may perform UAV atomic operations; however, no atomicity across adapters is guaranteed.
    Tier2 = D3D12_CROSS_NODE_SHARING_TIER_2.0,

    /// Indicates support for [`HeapFlags`] on heaps that are visible to multiple nodes.
    Tier3 = D3D12_CROSS_NODE_SHARING_TIER_3.0,
}

/// Specifies a type of descriptor heap.
///
/// For more information: [`D3D12_DESCRIPTOR_HEAP_TYPE enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_descriptor_heap_type)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
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

/// Defines constants that specify a Direct3D 12 feature or feature set to query about.
///
/// For more information: [`D3D12_FEATURE enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_feature)
#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(i32)]
pub enum FeatureType {
    /// Indicates a query for the level of support for basic Direct3D 12 feature options.
    Options = D3D12_FEATURE_D3D12_OPTIONS.0,

    /// Indicates a query for the adapter's architectural details, so that your application can better optimize for certain adapter properties.
    Architecture = D3D12_FEATURE_ARCHITECTURE.0,

    /// Indicates a query for info about the feature levels supported.
    FeatureLevels = D3D12_FEATURE_FEATURE_LEVELS.0,

    /// Indicates a query for the resources supported by the current graphics driver for a given format.
    FormatSupport = D3D12_FEATURE_FORMAT_SUPPORT.0,

    /// Indicates a query for the image quality levels for a given format and sample count.
    MultisampleQualityLevels = D3D12_FEATURE_MULTISAMPLE_QUALITY_LEVELS.0,

    /// Indicates a query for the DXGI data format.
    FormatInfo = D3D12_FEATURE_FORMAT_INFO.0,

    /// Indicates a query for the GPU's virtual address space limitations.
    GpuVirtualAddressSupport = D3D12_FEATURE_GPU_VIRTUAL_ADDRESS_SUPPORT.0,

    /// Indicates a query for the supported shader model.
    ShaderModel = D3D12_FEATURE_SHADER_MODEL.0,

    /// Indicates a query for the level of support for HLSL 6.0 wave operations.
    Options1 = D3D12_FEATURE_D3D12_OPTIONS1.0,

    /// Indicates a query for the level of support for protected resource sessions.
    ProtectedResourceSessionSupport = D3D12_FEATURE_PROTECTED_RESOURCE_SESSION_SUPPORT.0,

    /// Indicates a query for root signature version support.
    RootSignature = D3D12_FEATURE_ROOT_SIGNATURE.0,

    /// Indicates a query for each adapter's architectural details, so that your application can better optimize for certain adapter properties.
    Architecture1 = D3D12_FEATURE_ARCHITECTURE1.0,

    /// Indicates a query for the level of support for depth-bounds tests and programmable sample positions.
    Options2 = D3D12_FEATURE_D3D12_OPTIONS2.0,

    /// Indicates a query for the level of support for shader caching.
    ShaderCache = D3D12_FEATURE_SHADER_CACHE.0,

    /// Indicates a query for the adapter's support for prioritization of different command queue types.
    CommandQueuePriority = D3D12_FEATURE_COMMAND_QUEUE_PRIORITY.0,

    /// Indicates a query for the level of support for timestamp queries, format-casting, immediate write, view instancing, and barycentrics.
    Options3 = D3D12_FEATURE_D3D12_OPTIONS3.0,

    /// Indicates a query for whether or not the adapter supports creating heaps from existing system memory.
    ExistingHeaps = D3D12_FEATURE_EXISTING_HEAPS.0,

    /// Indicates a query for the level of support for 64KB-aligned MSAA textures, cross-API sharing, and native 16-bit shader operations.
    Options4 = D3D12_FEATURE_D3D12_OPTIONS4.0,

    /// Indicates a query for the level of support for heap serialization.
    Serialization = D3D12_FEATURE_SERIALIZATION.0,

    /// Indicates a query for the level of support for the sharing of resources between different adapters—for example, multiple GPUs.
    CrossNode = D3D12_FEATURE_CROSS_NODE.0,

    /// Starting with Windows 10, version 1809 (10.0; Build 17763), indicates a query for the level of support for render passes, ray tracing, and shader-resource view tier 3 tiled resources.
    Options5 = D3D12_FEATURE_D3D12_OPTIONS5.0,

    /// Starting with Windows 11 (Build 10.0.22000.194).
    Displayable = D3D12_FEATURE_DISPLAYABLE.0,

    /// Starting with Windows 10, version 1903 (10.0; Build 18362), indicates a query for the level of support for variable-rate shading (VRS), and indicates whether or not background processing is supported.
    Options6 = D3D12_FEATURE_D3D12_OPTIONS6.0,

    /// Indicates a query for the level of support for metacommands.
    QueryMetaCommand = D3D12_FEATURE_QUERY_META_COMMAND.0,

    /// Starting with Windows 10, version 2004 (10.0; Build 19041), indicates a query for the level of support for mesh and amplification shaders, and for sampler feedback.
    Options7 = D3D12_FEATURE_D3D12_OPTIONS7.0,

    /// Starting with Windows 10, version 2004 (10.0; Build 19041), indicates a query to retrieve the count of protected resource session types.
    ProtectedResourceSessionTypeCount = D3D12_FEATURE_PROTECTED_RESOURCE_SESSION_TYPE_COUNT.0,

    /// Starting with Windows 10, version 2004 (10.0; Build 19041), indicates a query to retrieve the list of protected resource session types.
    ProtectedResourceSessionTypes = D3D12_FEATURE_PROTECTED_RESOURCE_SESSION_TYPES.0,

    /// Starting with Windows 11 (Build 10.0.22000.194), indicates whether or not unaligned block-compressed textures are supported.
    Options8 = D3D12_FEATURE_D3D12_OPTIONS8.0,

    /// Starting with Windows 11 (Build 10.0.22000.194), indicates whether or not support exists for mesh shaders, values of SV_RenderTargetArrayIndex
    /// that are 8 or greater, typed resource 64-bit integer atomics, derivative and derivative-dependent texture sample operations, and the level of
    /// support for WaveMMA (wave_matrix) operations.
    Options9 = D3D12_FEATURE_D3D12_OPTIONS9.0,

    /// Starting with Windows 11 (Build 10.0.22000.194), indicates whether or not the SUM combiner can be used, and whether or not SV_ShadingRate can be set from a mesh shader.
    Options10 = D3D12_FEATURE_D3D12_OPTIONS10.0,

    /// Starting with Windows 11 (Build 10.0.22000.194), indicates whether or not 64-bit integer atomics on resources in descriptor heaps are supported.
    Options11 = D3D12_FEATURE_D3D12_OPTIONS11.0,

    /// TBD
    Options12 = D3D12_FEATURE_D3D12_OPTIONS12.0,

    /// TBD
    Options13 = D3D12_FEATURE_D3D12_OPTIONS13.0,

    /// TBD
    Options14 = D3D12_FEATURE_D3D12_OPTIONS14.0,

    /// TBD
    Options15 = D3D12_FEATURE_D3D12_OPTIONS15.0,

    /// TBD
    Options16 = D3D12_FEATURE_D3D12_OPTIONS16.0,

    /// TBD
    Options17 = D3D12_FEATURE_D3D12_OPTIONS17.0,

    /// TBD
    Options18 = D3D12_FEATURE_D3D12_OPTIONS18.0,

    /// TBD
    Options19 = D3D12_FEATURE_D3D12_OPTIONS19.0,

    /// TBD
    Options20 = D3D12_FEATURE_D3D12_OPTIONS20.0,

    /// TBD
    Predication = D3D12_FEATURE_PREDICATION.0,

    /// TBD
    PlacedResourceSupportInfo = D3D12_FEATURE_PLACED_RESOURCE_SUPPORT_INFO.0,

    /// TBD
    HardwareCopy = D3D12_FEATURE_HARDWARE_COPY.0,
}

/// Describes the set of features targeted by a Direct3D device.
///
/// For more information: [`D3D_FEATURE_LEVEL enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_feature_level)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum FeatureLevel {
    /// Targets features supported by Direct3D 11.0, including shader model 5.
    #[default]
    Level11 = D3D_FEATURE_LEVEL_11_0.0,

    /// Targets features supported by Direct3D 11.1, including shader model 5 and logical blend operations.
    /// This feature level requires a display driver that is at least implemented to WDDM for Windows 8 (WDDM 1.2).
    Level11_1 = D3D_FEATURE_LEVEL_11_1.0,

    /// Targets features supported by Direct3D 12.0, including shader model 5.
    Level12 = D3D_FEATURE_LEVEL_12_0.0,

    /// Targets features supported by Direct3D 12.1, including shader model 5.
    Level12_1 = D3D_FEATURE_LEVEL_12_1.0,

    /// Targets features supported by Direct3D 12.2, including shader model 6.5.
    Level12_2 = D3D_FEATURE_LEVEL_12_2.0,
}

/// Heap alignment variants.
#[derive(Clone, Copy, Debug, Default, FromRepr)]
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

/// Defines constants that specify heap serialization support.
///
/// For more information: [`D3D12_HEAP_SERIALIZATION_TIER enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_heap_serialization_tier)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum HeapSerializationTier {
    /// Indicates that heap serialization is not supported.
    #[default]
    Tier0 = D3D12_HEAP_SERIALIZATION_TIER_0.0,

    /// Indicates that heap serialization is supported. Your application can serialize resource data in heaps through copying APIs such as CopyResource, 
    /// without necessarily requiring an explicit state transition of resources on those heaps.
    Tier10 = D3D12_HEAP_SERIALIZATION_TIER_10.0,
}

/// Specifies the type of heap. When resident, heaps reside in a particular physical memory pool with certain CPU cache properties.
///
/// For more information: [`D3D12_HEAP_TYPE enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_heap_type)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
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
#[derive(Clone, Copy, Debug, FromRepr)]
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

/// Describes minimum precision support options for shaders in the current graphics driver.
///
/// For more information: [`D3D12_SHADER_MIN_PRECISION_SUPPORT enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_shader_min_precision_support)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum MinPrecisionSupport {
    /// The driver supports only full 32-bit precision for all shader stages.
    #[default]
    None = D3D12_SHADER_MIN_PRECISION_SUPPORT_NONE.0,

    /// The driver supports 10-bit precision.
    Support10Bit = D3D12_SHADER_MIN_PRECISION_SUPPORT_10_BIT.0,

    /// The driver supports 16-bit precision.
    Support16Bit = D3D12_SHADER_MIN_PRECISION_SUPPORT_16_BIT.0,
}

/// Specifies the level of support for programmable sample positions that's offered by the adapter.
///
/// For more information: [`D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_programmable_sample_positions_tier)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum ProgrammableSamplePositionsTier {
    /// Indicates that there's no support for programmable sample positions.
    #[default]
    NotSupported = D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER_NOT_SUPPORTED.0,

    /// Indicates that there's tier 1 support for programmable sample positions.
    /// In tier 1, a single sample pattern can be specified to repeat for every pixel (SetSamplePosition parameter NumPixels = 1) and ResolveSubResource is supported.
    Tier1 = D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER_1.0,

    /// Indicates that there's tier 2 support for programmable sample positions.
    /// In tier 2, four separate sample patterns can be specified for each pixel in a 2x2 grid (SetSamplePosition parameter NumPixels = 1) that
    /// repeats over the render-target or viewport, aligned on even coordinates.
    Tier2 = D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER_2.0,
}

/// Specifies the level of ray tracing support on the graphics device.
///
/// For more information: [`D3D12_RAYTRACING_TIER enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_raytracing_tier)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum RaytracingTier {
    /// No support for ray tracing on the device. Attempts to create any ray tracing-related object will fail, and using ray tracing-related APIs on command lists results in undefined behavior.
    #[default]
    NotSupported = D3D12_RAYTRACING_TIER_NOT_SUPPORTED.0,

    /// The device supports tier 1 ray tracing functionality. In the current release, this tier represents all available ray tracing features.
    Tier1_0 = D3D12_RAYTRACING_TIER_1_0.0,

    /// TBD
    Tier1_1 = D3D12_RAYTRACING_TIER_1_1.0,
}

/// Specifies the level of support for render passes on a graphics device.
///
/// For more information: [`D3D12_RENDER_PASS_TIER enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_render_pass_tier)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum RenderPassTier {
    /// The user-mode display driver hasn't implemented render passes, and so the feature is provided only via software emulation.
    /// Render passes might not provide a performance advantage at this level of support.
    #[default]
    Tier0 = D3D12_RENDER_PASS_TIER_0.0,

    /// The render passes feature is implemented by the user-mode display driver, and render target/depth buffer writes may be accelerated. 
    /// Unordered access view (UAV) writes are not efficiently supported within the render pass.
    Tier1 = D3D12_RENDER_PASS_TIER_1.0,

    /// The render passes feature is implemented by the user-mode display driver, render target/depth buffer writes may be accelerated, 
    /// and unordered access view (UAV) writes (provided that writes in a render pass are not read until a subsequent render pass) are likely to be more efficient than 
    /// issuing the same work without using a render pass.
    Tier2 = D3D12_RENDER_PASS_TIER_2.0,
}

/// Identifies the tier of resource binding being used.
///
/// For more information: [`D3D12_RESOURCE_BINDING_TIER enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_resource_binding_tier)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum ResourceBindingTier {
    /// Tier 1
    #[default]
    Tier1 = D3D12_RESOURCE_BINDING_TIER_1.0,

    /// Tier 2
    Tier2 = D3D12_RESOURCE_BINDING_TIER_2.0,

    /// Tier 3
    Tier3 = D3D12_RESOURCE_BINDING_TIER_3.0,
}

/// Specifies which resource heap tier the hardware and driver support.
///
/// For more information: [`D3D12_RESOURCE_HEAP_TIER enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_resource_heap_tier)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum ResourceHeapTier {
    /// Indicates that heaps can only support resources from a single resource category.
    #[default]
    Tier1 = D3D12_RESOURCE_HEAP_TIER_1.0,

    /// Indicates that heaps can support resources from all three categories.
    Tier2 = D3D12_RESOURCE_HEAP_TIER_2.0,
}

/// Specifies the version of root signature layout.
///
/// For more information: [`D3D_ROOT_SIGNATURE_VERSION enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d_root_signature_version)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum RootSignatureVersion {
    /// Version one of root signature layout.
    #[default]
    V1_0 = D3D_ROOT_SIGNATURE_VERSION_1_0.0,

    /// Version 1.1 of root signature layout.
    V1_1 = D3D_ROOT_SIGNATURE_VERSION_1_1.0,

    /// TBD
    V1_2 = D3D_ROOT_SIGNATURE_VERSION_1_2.0,
}

/// Specifies a shader model.
///
/// For more information: [`D3D_SHADER_MODEL enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d_shader_model)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum ShaderModel {
    /// TBD
    #[default]
    None = 0,

    /// Indicates shader model 5.1.
    Model5_1 = D3D_SHADER_MODEL_5_1.0,

    /// Indicates shader model 6.0. Compiling a shader model 6.0 shader requires using the DXC compiler, and is not supported by legacy FXC.
    Model6_0 = D3D_SHADER_MODEL_6_0.0,

    /// Indicates shader model 6.1.
    Model6_1 = D3D_SHADER_MODEL_6_1.0,

    /// Indicates shader model 6.2.
    Model6_2 = D3D_SHADER_MODEL_6_2.0,

    /// Indicates shader model 6.3.
    Model6_3 = D3D_SHADER_MODEL_6_3.0,

    /// Shader model 6.4 support was added in Windows 10, Version 1903, and is required for DirectX Raytracing (DXR).
    Model6_4 = D3D_SHADER_MODEL_6_4.0,

    /// Shader model 6.5 support was added in Windows 10, Version 2004, and is required for Direct Machine Learning.
    Model6_5 = D3D_SHADER_MODEL_6_5.0,

    /// Shader model 6.6 support was added in Windows 11 and the DirectX 12 Agility SDK.
    Model6_6 = D3D_SHADER_MODEL_6_6.0,

    /// Shader model 6.7 support was added in the DirectX 12 Agility SDK v1.6
    Model6_7 = D3D_SHADER_MODEL_6_7.0,

    /// TBD
    Model6_8 = D3D_SHADER_MODEL_6_8.0,
}

/// Defines constants that specify a cross-API sharing support tier.
///
/// For more information: [`D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_shared_resource_compatibility_tier)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum SharedResourceCompatibilityTier {
    /// Specifies that the most basic level of cross-API sharing is supported.
    #[default]
    Tier0 = D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER_0.0,

    /// Specifies that cross-API sharing functionality of [`SharedResourceCompatibilityTier::Tier0`] is supported, plus the other formats.
    Tier1 = D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER_1.0,

    /// Specifies that cross-API sharing functionality of [`SharedResourceCompatibilityTier::Tier1`] is supported, plus the other formats.
    Tier2 = D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER_2.0,
}

/// Identifies the tier level at which tiled resources are supported.
///
/// For more information: [`D3D12_TILED_RESOURCES_TIER enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_tiled_resources_tier)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum TiledResourcesTier {
    /// Indicates that textures cannot be created with the [`TextureLayout64kbUndefinedSwizzle`] layout.
    #[default]
    NotSupported = D3D12_TILED_RESOURCES_TIER_NOT_SUPPORTED.0,

    /// Indicates that 2D textures can be created with the [`TextureLayout64kbUndefinedSwizzle`] layout.
    /// Limitations exist for certain resource formats and properties.
    Tier1 = D3D12_TILED_RESOURCES_TIER_1.0,

    /// Indicates that a superset of Tier_1 functionality is supported.
    Tier2 = D3D12_TILED_RESOURCES_TIER_2.0,

    /// Indicates that a superset of Tier 2 is supported, with the addition that 3D textures (Volume Tiled Resources) are supported.
    Tier3 = D3D12_TILED_RESOURCES_TIER_3.0,

    /// TBD
    Tier4 = D3D12_TILED_RESOURCES_TIER_4.0,
}

/// Indicates the tier level at which view instancing is supported.
///
/// For more information: [`D3D12_VIEW_INSTANCING_TIER enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_view_instancing_tier)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum ViewInstancingTier {
    /// View instancing is not supported.
    #[default]
    NotSupported = D3D12_VIEW_INSTANCING_TIER_NOT_SUPPORTED.0,

    /// View instancing is supported by draw-call level looping only.
    Tier1 = D3D12_VIEW_INSTANCING_TIER_1.0,

    /// View instancing is supported by draw-call level looping at worst, but the GPU can perform view instancing more efficiently in certain circumstances which are architecture-dependent.
    Tier2 = D3D12_VIEW_INSTANCING_TIER_2.0,

    /// View instancing is supported and instancing begins with the first shader stage that references SV_ViewID or with rasterization
    /// if no shader stage references SV_ViewID. This means that redundant work is eliminated across view instances when it's not dependent on SV_ViewID.
    /// Before rasterization, work that doesn't directly depend on SV_ViewID is shared across all views; only work that depends on SV_ViewID is repeated for each view.
    Tier3 = D3D12_VIEW_INSTANCING_TIER_3.0,
}
