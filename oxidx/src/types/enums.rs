use strum::FromRepr;
use windows::Win32::Graphics::{Direct3D::*, Direct3D12::*};

#[allow(unused_imports)]
use super::*;

/// Identifies a technique for resolving texture coordinates that are outside of the boundaries of a texture.
///
/// For more information: [`D3D12_TEXTURE_ADDRESS_MODE enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_texture_address_mode)
#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(i32)]
pub enum AddressMode {
    /// Tile the texture at every (u,v) integer junction.
    Wrap = D3D12_TEXTURE_ADDRESS_MODE_WRAP.0,

    /// Flip the texture at every (u,v) integer junction.
    Mirror = D3D12_TEXTURE_ADDRESS_MODE_MIRROR.0,

    /// Texture coordinates outside the range [0.0, 1.0] are set to the texture color at 0.0 or 1.0, respectively.
    Clamp = D3D12_TEXTURE_ADDRESS_MODE_CLAMP.0,

    /// Texture coordinates outside the range [0.0, 1.0] are set to the border color specified in [`SamplerDesc`] or HLSL code.
    Border = D3D12_TEXTURE_ADDRESS_MODE_BORDER.0,

    /// Similar to [`AddressMode::Mirror`] and [`AddressMode::Clamp`]. Takes the absolute value of the texture coordinate (thus, mirroring around 0), and then clamps to the maximum value.
    MirrorOnce = D3D12_TEXTURE_ADDRESS_MODE_MIRROR_ONCE.0,
}

/// Specifies blend factors, which modulate values for the pixel shader and render target.
///
/// For more information: [`D3D12_BLEND enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_blend)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum Blend {
    /// The blend factor is (0, 0, 0, 0). No pre-blend operation.
    #[default]
    Zero = D3D12_BLEND_ZERO.0,

    /// The blend factor is (1, 1, 1, 1). No pre-blend operation.
    One = D3D12_BLEND_ONE.0,

    /// The blend factor is (Rₛ, Gₛ, Bₛ, Aₛ), that is color data (RGB) from a pixel shader. No pre-blend operation.
    SrcColor = D3D12_BLEND_SRC_COLOR.0,

    /// The blend factor is (1 - Rₛ, 1 - Gₛ, 1 - Bₛ, 1 - Aₛ), that is color data (RGB) from a pixel shader. The pre-blend operation inverts the data, generating 1 - RGB.
    InvSrcColor = D3D12_BLEND_INV_SRC_COLOR.0,

    /// The blend factor is (Aₛ, Aₛ, Aₛ, Aₛ), that is alpha data (A) from a pixel shader. No pre-blend operation.
    SrcAlpha = D3D12_BLEND_SRC_ALPHA.0,

    /// The blend factor is ( 1 - Aₛ, 1 - Aₛ, 1 - Aₛ, 1 - Aₛ), that is alpha data (A) from a pixel shader. The pre-blend operation inverts the data, generating 1 - A.
    InvSrcAlpha = D3D12_BLEND_INV_SRC_ALPHA.0,

    /// The blend factor is (Ad Ad Ad Ad), that is alpha data from a render target. No pre-blend operation.
    DestAlpha = D3D12_BLEND_DEST_ALPHA.0,

    /// The blend factor is (1 - Ad 1 - Ad 1 - Ad 1 - Ad), that is alpha data from a render target. The pre-blend operation inverts the data, generating 1 - A.
    InvDestAlpha = D3D12_BLEND_INV_DEST_ALPHA.0,

    /// The blend factor is (Rd, Gd, Bd, Ad), that is color data from a render target. No pre-blend operation.
    DestColor = D3D12_BLEND_DEST_COLOR.0,

    /// The blend factor is (1 - Rd, 1 - Gd, 1 - Bd, 1 - Ad), that is color data from a render target. The pre-blend operation inverts the data, generating 1 - RGB.
    InvDestColor = D3D12_BLEND_INV_DEST_COLOR.0,

    /// The blend factor is (f, f, f, 1); where f = min(Aₛ, 1 - Ad). The pre-blend operation clamps the data to 1 or less.
    SrcAlphaSat = D3D12_BLEND_SRC_ALPHA_SAT.0,

    /// The blend factor is the blend factor set with [`GraphicsCommandListInterface::om_set_blend_factor`]. No pre-blend operation.
    BlendFactor = D3D12_BLEND_BLEND_FACTOR.0,

    /// The blend factor is the blend factor set with [`GraphicsCommandListInterface::om_set_blend_factor`]. The pre-blend operation inverts the blend factor, generating 1 - blend_factor.
    InvBlendFactor = D3D12_BLEND_INV_BLEND_FACTOR.0,

    /// The blend factor is data sources both as color data output by a pixel shader. There is no pre-blend operation. This blend factor supports dual-source color blending.
    Src1Color = D3D12_BLEND_SRC1_COLOR.0,

    /// The blend factor is data sources both as color data output by a pixel shader. The pre-blend operation inverts the data, generating 1 - RGB. This blend factor supports dual-source color blending.
    InvSrc1Color = D3D12_BLEND_INV_SRC1_COLOR.0,

    /// The blend factor is data sources as alpha data output by a pixel shader. There is no pre-blend operation. This blend factor supports dual-source color blending.
    Src1Alpha = D3D12_BLEND_SRC1_ALPHA.0,

    /// The blend factor is data sources as alpha data output by a pixel shader. The pre-blend operation inverts the data, generating 1 - A. This blend factor supports dual-source color blending.
    InvSrc1Alpha = D3D12_BLEND_INV_SRC1_ALPHA.0,

    /// The blend factor is (A, A, A, A), where the constant, A, is taken from the blend factor set with [`GraphicsCommandListInterface::om_set_blend_factor`].
    AlphaFactor = D3D12_BLEND_ALPHA_FACTOR.0,

    /// The blend factor is (1 – A, 1 – A, 1 – A, 1 – A), where the constant, A, is taken from the blend factor set with [`GraphicsCommandListInterface::om_set_blend_factor`].
    InvAlphaFactor = D3D12_BLEND_INV_ALPHA_FACTOR.0,
}

/// Specifies RGB or alpha blending operations.
///
/// For more information: [`D3D12_BLEND_OP enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_blend_op)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum BlendOp {
    /// Add source 1 and source 2.
    #[default]
    Add = D3D12_BLEND_OP_ADD.0,

    /// Subtract source 1 from source 2.
    Subtract = D3D12_BLEND_OP_SUBTRACT.0,

    /// Subtract source 2 from source 1.
    RevSubtract = D3D12_BLEND_OP_REV_SUBTRACT.0,

    /// Find the minimum of source 1 and source 2.
    Min = D3D12_BLEND_OP_MIN.0,

    /// Find the maximum of source 1 and source 2.
    Max = D3D12_BLEND_OP_MAX.0,
}

/// Specifies the border color for a static sampler.
///
/// For more information: [`D3D12_STATIC_BORDER_COLOR structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_static_border_color)
#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(i32)]
pub enum BorderColor {
    /// Indicates black, with the alpha component as fully transparent.
    TransparentBlack = D3D12_STATIC_BORDER_COLOR_TRANSPARENT_BLACK.0,

    /// Indicates black, with the alpha component as fully opaque.
    OpaqueBlack = D3D12_STATIC_BORDER_COLOR_OPAQUE_BLACK.0,

    /// Indicates white, with the alpha component as fully opaque.
    OpaqueWhite = D3D12_STATIC_BORDER_COLOR_OPAQUE_WHITE.0,

    /// TBD
    OpaqueBlackUint = D3D12_STATIC_BORDER_COLOR_OPAQUE_BLACK_UINT.0,

    /// TBD
    OpaqueWhiteUint = D3D12_STATIC_BORDER_COLOR_OPAQUE_WHITE_UINT.0,
}

/// Describes a value used to optimize clear operations for a particular resource.
///
/// For more information: [`D3D12_CLEAR_VALUE structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_clear_value)
#[derive(Clone, Copy, Debug)]
pub enum ClearValue {
    Color {
        /// Specifies one member of the [`Format`] enum.
        format: Format,

        /// Specifies a 4-entry array of float values, determining the RGBA value.
        value: [f32; 4],
    },
    Depth {
        /// Specifies one member of the [`Format`] enum.
        format: Format,

        /// Specifies the depth value.
        depth: f32,

        /// Specifies the stencil value.
        stencil: u8,
    },
}

/// Identifies whether conservative rasterization is on or off.
///
/// For more information: [`D3D12_CONSERVATIVE_RASTERIZATION_MODE enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_conservative_rasterization_mode)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum ConservativeRaster {
    /// Conservative rasterization is off.
    #[default]
    Off = D3D12_CONSERVATIVE_RASTERIZATION_MODE_OFF.0,

    /// Conservative rasterization is on.
    On = D3D12_CONSERVATIVE_RASTERIZATION_MODE_ON.0,
}

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

/// Specifies comparison options.
///
/// For more information: [`D3D12_COMPARISON_FUNC enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_comparison_func)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum ComparisonFunc {
    /// None
    #[default]
    None = D3D12_COMPARISON_FUNC_NONE.0,

    /// Never pass the comparison.
    Never = D3D12_COMPARISON_FUNC_NEVER.0,

    /// If the source data is less than the destination data, the comparison passes.
    Less = D3D12_COMPARISON_FUNC_LESS.0,

    /// If the source data is equal to the destination data, the comparison passes.
    Equal = D3D12_COMPARISON_FUNC_EQUAL.0,

    /// If the source data is less than or equal to the destination data, the comparison passes.
    LessEqual = D3D12_COMPARISON_FUNC_LESS_EQUAL.0,

    /// If the source data is greater than the destination data, the comparison passes.
    Greater = D3D12_COMPARISON_FUNC_GREATER.0,

    /// If the source data is not equal to the destination data, the comparison passes.
    NotEqual = D3D12_COMPARISON_FUNC_NOT_EQUAL.0,

    /// If the source data is greater than or equal to the destination data, the comparison passes.
    GreaterEqual = D3D12_COMPARISON_FUNC_GREATER_EQUAL.0,

    /// Always pass the comparison.
    Always = D3D12_COMPARISON_FUNC_ALWAYS.0,
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

/// Specifies triangles facing a particular direction are not drawn.
///
/// For more information: [`D3D12_CULL_MODE enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_cull_mode)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum CullMode {
    /// Always draw all triangles.
    #[default]
    None = D3D12_CULL_MODE_NONE.0,

    /// Do not draw triangles that are front-facing.
    Front = D3D12_CULL_MODE_FRONT.0,

    /// Do not draw triangles that are back-facing.
    Back = D3D12_CULL_MODE_BACK.0,
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

/// Specifies a range so that, for example, if part of a descriptor table has 100 shader-resource views (SRVs) that range can be declared in one entry rather than 100.
///
/// For more information: [`D3D12_DESCRIPTOR_RANGE_TYPE enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_descriptor_range_type)
#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(i32)]
pub enum DescriptorRangeType {
    /// Specifies a range of SRVs.
    Srv = D3D12_DESCRIPTOR_RANGE_TYPE_SRV.0,

    /// Specifies a range of unordered-access views (UAVs).
    Uav = D3D12_DESCRIPTOR_RANGE_TYPE_UAV.0,

    /// Specifies a range of constant-buffer views (CBVs).
    Cbv = D3D12_DESCRIPTOR_RANGE_TYPE_CBV.0,

    /// Specifies a range of samplers.
    Sampler = D3D12_DESCRIPTOR_RANGE_TYPE_SAMPLER.0,
}

/// Specifies how to access a resource used in a depth-stencil view.
///
/// For more information: [`D3D12_DSV_DIMENSION enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_dsv_dimension)
#[derive(Clone, Copy, Debug)]
pub enum DsvDimension {
    /// The resource will be accessed as a 1D texture.
    Tex1D {
        /// The index of the first mipmap level to use.
        mip_slice: u32,
    },

    /// The resource will be accessed as an array of 1D textures.
    ArrayTex1D {
        /// The index of the first mipmap level to use.
        mip_slice: u32,

        /// The index of the first texture to use in an array of textures.
        first_array_slice: u32,

        /// Number of textures to use.
        array_size: u32,
    },

    /// The resource will be accessed as a 2D texture.
    Tex2D {
        /// The index of the first mipmap level to use.
        mip_slice: u32,
    },

    /// The resource will be accessed as an array of 2D textures.
    ArrayTex2D {
        /// The index of the first mipmap level to use.
        mip_slice: u32,

        /// The index of the first texture to use in an array of textures.
        first_array_slice: u32,

        /// Number of textures to use.
        array_size: u32,
    },

    /// The resource will be accessed as a 2D texture with multi sampling.
    Tex2DMs,

    /// The resource will be accessed as an array of 2D textures with multi sampling.
    ArrayTex2DMs {
        /// The index of the first texture to use in an array of textures.
        first_array_slice: u32,

        /// Number of textures to use.
        array_size: u32,
    },
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

/// Specifies filtering options during texture sampling.
///
/// For more information: [`D3D12_FILTER enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_filter)
#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(i32)]
pub enum Filter {
    /// Use point sampling for minification, magnification, and mip-level sampling.
    Point = D3D12_FILTER_MIN_MAG_MIP_POINT.0,

    /// Use point sampling for minification and magnification; use linear interpolation for mip-level sampling.
    MinMagPointMipLinear = D3D12_FILTER_MIN_MAG_POINT_MIP_LINEAR.0,

    /// Use point sampling for minification; use linear interpolation for magnification; use point sampling for mip-level sampling.
    MinMipPointMagLinear = D3D12_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT.0,

    /// Use point sampling for minification; use linear interpolation for magnification and mip-level sampling.
    MinPointMagMipLinear = D3D12_FILTER_MIN_POINT_MAG_MIP_LINEAR.0,

    /// Use linear interpolation for minification; use point sampling for magnification and mip-level sampling.
    MinLinearMagMipPoint = D3D12_FILTER_MIN_LINEAR_MAG_MIP_POINT.0,

    /// Use linear interpolation for minification; use point sampling for magnification; use linear interpolation for mip-level sampling.
    MinMipLinearMagPoint = D3D12_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR.0,

    /// Use linear interpolation for minification and magnification; use point sampling for mip-level sampling.
    MinMagLinearMipPoint = D3D12_FILTER_MIN_MAG_LINEAR_MIP_POINT.0,

    /// Use linear interpolation for minification, magnification, and mip-level sampling.
    Linear = D3D12_FILTER_MIN_MAG_MIP_LINEAR.0,

    /// TBD
    MinMagAnisotropicMipPoint = D3D12_FILTER_MIN_MAG_ANISOTROPIC_MIP_POINT.0,

    /// Use anisotropic interpolation for minification, magnification, and mip-level sampling.
    Anisotropic = D3D12_FILTER_ANISOTROPIC.0,

    /// Use point sampling for minification, magnification, and mip-level sampling. Compare the result to the comparison value.
    ComparisonPoint = D3D12_FILTER_COMPARISON_MIN_MAG_MIP_POINT.0,

    /// Use point sampling for minification and magnification; use linear interpolation for mip-level sampling. Compare the result to the comparison value.
    ComparisonMinMagPointMipLinear = D3D12_FILTER_COMPARISON_MIN_MAG_POINT_MIP_LINEAR.0,

    /// Use point sampling for minification; use linear interpolation for magnification; use point sampling for mip-level sampling. Compare the result to the comparison value.
    ComparisonMinMipPointMagLinear = D3D12_FILTER_COMPARISON_MIN_POINT_MAG_LINEAR_MIP_POINT.0,

    /// Use point sampling for minification; use linear interpolation for magnification and mip-level sampling. Compare the result to the comparison value.
    ComparisonMinPointMagMipLinear = D3D12_FILTER_COMPARISON_MIN_POINT_MAG_MIP_LINEAR.0,

    /// Use linear interpolation for minification; use point sampling for magnification and mip-level sampling. Compare the result to the comparison value.
    ComparisonMinLinearMagMipPoint = D3D12_FILTER_COMPARISON_MIN_LINEAR_MAG_MIP_POINT.0,

    /// Use linear interpolation for minification; use point sampling for magnification; use linear interpolation for mip-level sampling. Compare the result to the comparison value.
    ComparisonMinMipLinearMagPoint = D3D12_FILTER_COMPARISON_MIN_LINEAR_MAG_POINT_MIP_LINEAR.0,

    /// Use linear interpolation for minification and magnification; use point sampling for mip-level sampling. Compare the result to the comparison value.
    ComparisonMinMagLinearMipPoint = D3D12_FILTER_COMPARISON_MIN_MAG_LINEAR_MIP_POINT.0,

    /// Use linear interpolation for minification, magnification, and mip-level sampling. Compare the result to the comparison value.
    ComparisonLinear = D3D12_FILTER_COMPARISON_MIN_MAG_MIP_LINEAR.0,

    /// TBD
    ComparisonMinMagAnisotropicMipPoint = D3D12_FILTER_COMPARISON_MIN_MAG_ANISOTROPIC_MIP_POINT.0,

    /// Use anisotropic interpolation for minification, magnification, and mip-level sampling. Compare the result to the comparison value.
    ComparisonAnisotropic = D3D12_FILTER_COMPARISON_ANISOTROPIC.0,

    /// Fetch the same set of texels as [`Filter::Point`] and instead of filtering them return the minimum of the texels.
    MinimumPoint = D3D12_FILTER_MINIMUM_MIN_MAG_MIP_POINT.0,

    /// Fetch the same set of texels as [`Filter::MinMagPointMipLinear`] and instead of filtering them return the minimum of the texels.
    MinimumMinMagPointMipLinear = D3D12_FILTER_MINIMUM_MIN_MAG_POINT_MIP_LINEAR.0,

    /// Fetch the same set of texels as [`Filter::MinMipPointMagLinear`] and instead of filtering them return the minimum of the texels.
    MinimumMinMipPointMagLinear = D3D12_FILTER_MINIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT.0,

    /// Fetch the same set of texels as [`Filter::MinPointMagMipLinear`] and instead of filtering them return the minimum of the texels.
    MinimumMinPointMagMipLinear = D3D12_FILTER_MINIMUM_MIN_POINT_MAG_MIP_LINEAR.0,

    /// Fetch the same set of texels as [`Filter::MinLinearMagMipPoint`] and instead of filtering them return the minimum of the texels.
    MinimumMinLinearMagMipPoint = D3D12_FILTER_MINIMUM_MIN_LINEAR_MAG_MIP_POINT.0,

    /// Fetch the same set of texels as [`Filter::MinMipLinearMagPoint`] and instead of filtering them return the minimum of the texels.
    MinimumMinMipLinearMagPoint = D3D12_FILTER_MINIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR.0,

    /// Fetch the same set of texels as [`Filter::MinMagLinearMipPoint`] and instead of filtering them return the minimum of the texels.
    MinimumMinMagLinearMipPoint = D3D12_FILTER_MINIMUM_MIN_MAG_LINEAR_MIP_POINT.0,

    /// Fetch the same set of texels as [`Filter::Linear`] and instead of filtering them return the minimum of the texels.
    MinimumLinear = D3D12_FILTER_MINIMUM_MIN_MAG_MIP_LINEAR.0,

    /// Fetch the same set of texels as [`Filter::MinMagAnisotropicMipPoint`] and instead of filtering them return the minimum of the texels.
    MinimumMinMagAnisotropicMipPoint = D3D12_FILTER_MINIMUM_MIN_MAG_ANISOTROPIC_MIP_POINT.0,

    /// Fetch the same set of texels as [`Filter::Anisotropic`] and instead of filtering them return the minimum of the texels.
    MinimumAnisotropic = D3D12_FILTER_MINIMUM_ANISOTROPIC.0,

    /// Fetch the same set of texels as [`Filter::Point`] and instead of filtering them return the maximum of the texels.
    MaximumPoint = D3D12_FILTER_MAXIMUM_MIN_MAG_MIP_POINT.0,

    /// Fetch the same set of texels as [`Filter::MinMagPointMipLinear`] and instead of filtering them return the maximum of the texels.
    MaximumMinMagPointMipLinear = D3D12_FILTER_MAXIMUM_MIN_MAG_POINT_MIP_LINEAR.0,

    /// Fetch the same set of texels as [`Filter::MinMipPointMagLinear`] and instead of filtering them return the maximum of the texels.
    MaximumMinMipPointMagLinear = D3D12_FILTER_MAXIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT.0,

    /// Fetch the same set of texels as [`Filter::MinPointMagMipLinear`] and instead of filtering them return the maximum of the texels.
    MaximumMinPointMagMipLinear = D3D12_FILTER_MAXIMUM_MIN_POINT_MAG_MIP_LINEAR.0,

    /// Fetch the same set of texels as [`Filter::MinLinearMagMipPoint`] and instead of filtering them return the maximum of the texels.
    MaximumMinLinearMagMipPoint = D3D12_FILTER_MAXIMUM_MIN_LINEAR_MAG_MIP_POINT.0,

    /// Fetch the same set of texels as [`Filter::MinMipLinearMagPoint`] and instead of filtering them return the maximum of the texels.
    MaximumMinMipLinearMagPoint = D3D12_FILTER_MAXIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR.0,

    /// Fetch the same set of texels as [`Filter::MinMagLinearMipPoint`] and instead of filtering them return the maximum of the texels.
    MaximumMinMagLinearMipPoint = D3D12_FILTER_MAXIMUM_MIN_MAG_LINEAR_MIP_POINT.0,

    /// Fetch the same set of texels as [`Filter::Linear`] and instead of filtering them return the maximum of the texels.
    MaximumLinear = D3D12_FILTER_MAXIMUM_MIN_MAG_MIP_LINEAR.0,

    /// Fetch the same set of texels as [`Filter::MinMagAnisotropicMipPoint`] and instead of filtering them return the maximum of the texels.
    MaximumMinMagAnisotropicMipPoint = D3D12_FILTER_MAXIMUM_MIN_MAG_ANISOTROPIC_MIP_POINT.0,

    /// Fetch the same set of texels as [`Filter::Anisotropic`] and instead of filtering them return the maximum of the texels.
    MaximumAnisotropic = D3D12_FILTER_MAXIMUM_ANISOTROPIC.0,
}

/// Specifies the fill mode to use when rendering triangles.
///
/// For more information: [`D3D12_FILL_MODE enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_fill_mode)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum FillMode {
    /// Draw lines connecting the vertices. Adjacent vertices are not drawn.
    Wireframe = D3D12_FILL_MODE_WIREFRAME.0,

    /// Fill the triangles formed by the vertices. Adjacent vertices are not drawn.
    #[default]
    Solid = D3D12_FILL_MODE_SOLID.0,
}

/// Resource data formats, including fully-typed and typeless formats. A list of modifiers at the bottom of the page more fully describes each format type.
///
/// For more information: [`DXGI_FORMAT enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/dxgiformat/ne-dxgiformat-dxgi_format)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum Format {
    /// The format is not known.
    #[default]
    Unknown = DXGI_FORMAT_UNKNOWN.0,

    /// A four-component, 128-bit typeless format that supports 32 bits per channel including alpha.
    Rgba32Typeless = DXGI_FORMAT_R32G32B32A32_TYPELESS.0,

    /// A four-component, 128-bit floating-point format that supports 32 bits per channel including alpha.
    Rgba32Float = DXGI_FORMAT_R32G32B32A32_FLOAT.0,

    /// A four-component, 128-bit unsigned-integer format that supports 32 bits per channel including alpha.
    Rgba32Uint = DXGI_FORMAT_R32G32B32A32_UINT.0,

    /// A four-component, 128-bit signed-integer format that supports 32 bits per channel including alpha.
    Rgba32Sint = DXGI_FORMAT_R32G32B32A32_SINT.0,

    /// A three-component, 96-bit typeless format that supports 32 bits per color channel.
    Rgb32Typeless = DXGI_FORMAT_R32G32B32_TYPELESS.0,

    /// A three-component, 96-bit floating-point format that supports 32 bits per color channel.
    Rgb32Float = DXGI_FORMAT_R32G32B32_FLOAT.0,

    /// A three-component, 96-bit unsigned-integer format that supports 32 bits per color channel.
    Rgb32Uint = DXGI_FORMAT_R32G32B32_UINT.0,

    /// A three-component, 96-bit signed-integer format that supports 32 bits per color channel.
    Rgb32Sint = DXGI_FORMAT_R32G32B32_SINT.0,

    /// A four-component, 64-bit typeless format that supports 16 bits per channel including alpha.
    Rgba16Typeless = DXGI_FORMAT_R16G16B16A16_TYPELESS.0,

    /// A four-component, 64-bit floating-point format that supports 16 bits per channel including alpha.
    Rgba16Float = DXGI_FORMAT_R16G16B16A16_FLOAT.0,

    /// A four-component, 64-bit unsigned-normalized-integer format that supports 16 bits per channel including alpha.
    Rgba16Unorm = DXGI_FORMAT_R16G16B16A16_UNORM.0,

    /// A four-component, 64-bit unsigned-integer format that supports 16 bits per channel including alpha.
    Rgba16Uint = DXGI_FORMAT_R16G16B16A16_UINT.0,

    /// A four-component, 64-bit signed-normalized-integer format that supports 16 bits per channel including alpha.
    Rgba16Snorm = DXGI_FORMAT_R16G16B16A16_SNORM.0,

    /// A four-component, 64-bit signed-integer format that supports 16 bits per channel including alpha.
    Rgba16Sint = DXGI_FORMAT_R16G16B16A16_SINT.0,

    /// A two-component, 64-bit typeless format that supports 32 bits for the red channel and 32 bits for the green channel.
    Rg32Typeless = DXGI_FORMAT_R32G32_TYPELESS.0,

    /// A two-component, 64-bit floating-point format that supports 32 bits for the red channel and 32 bits for the green channel.
    Rg32Float = DXGI_FORMAT_R32G32_FLOAT.0,

    /// A two-component, 64-bit unsigned-integer format that supports 32 bits for the red channel and 32 bits for the green channel.
    Rg32Uint = DXGI_FORMAT_R32G32_UINT.0,

    /// A two-component, 64-bit signed-integer format that supports 32 bits for the red channel and 32 bits for the green channel.
    Rg32Sint = DXGI_FORMAT_R32G32_SINT.0,

    /// A two-component, 64-bit typeless format that supports 32 bits for the red channel, 8 bits for the green channel, and 24 bits are unused.
    R32G8X24Typeless = DXGI_FORMAT_R32G8X24_TYPELESS.0,

    /// A 32-bit floating-point component, and two unsigned-integer components (with an additional 32 bits). This format supports 32-bit depth, 8-bit stencil, and 24 bits are unused.
    D32FloatS8X24Uint = DXGI_FORMAT_D32_FLOAT_S8X24_UINT.0,

    /// A 32-bit typeless component, and two unsigned-integer components (with an additional 32 bits). This format has 32 bits unused, 8 bits for green channel, and 24 bits are unused.
    R32FloatX8X24Typeless = DXGI_FORMAT_R32_FLOAT_X8X24_TYPELESS.0,

    /// A four-component, 32-bit typeless format that supports 10 bits for each color and 2 bits for alpha.
    Rgb10A2Typeless = DXGI_FORMAT_R10G10B10A2_TYPELESS.0,

    /// A four-component, 32-bit unsigned-normalized-integer format that supports 10 bits for each color and 2 bits for alpha.
    Rgb10A2Unorm = DXGI_FORMAT_R10G10B10A2_UNORM.0,

    /// A four-component, 32-bit unsigned-integer format that supports 10 bits for each color and 2 bits for alpha.
    Rgb10A2Uint = DXGI_FORMAT_R10G10B10A2_UINT.0,

    /// Three partial-precision floating-point numbers encoded into a single 32-bit value (a variant of s10e5, which is sign bit, 10-bit mantissa, and 5-bit biased (15) exponent).
    Rg11B10Float = DXGI_FORMAT_R11G11B10_FLOAT.0,

    /// A four-component, 32-bit typeless format that supports 8 bits per channel including alpha.
    Rgba8Typeless = DXGI_FORMAT_R8G8B8A8_TYPELESS.0,

    /// A four-component, 32-bit unsigned-normalized-integer format that supports 8 bits per channel including alpha.
    Rgba8Unorm = DXGI_FORMAT_R8G8B8A8_UNORM.0,

    /// A four-component, 32-bit unsigned-normalized integer sRGB format that supports 8 bits per channel including alpha.
    Rgba8UnormSrgb = DXGI_FORMAT_R8G8B8A8_UNORM_SRGB.0,

    /// A four-component, 32-bit unsigned-integer format that supports 8 bits per channel including alpha.
    Rgba8Uint = DXGI_FORMAT_R8G8B8A8_UINT.0,

    /// A four-component, 32-bit signed-normalized-integer format that supports 8 bits per channel including alpha.
    Rgba8Snorm = DXGI_FORMAT_R8G8B8A8_SNORM.0,

    /// A four-component, 32-bit signed-integer format that supports 8 bits per channel including alpha.
    Rgba8Sint = DXGI_FORMAT_R8G8B8A8_SINT.0,

    /// A two-component, 32-bit typeless format that supports 16 bits for the red channel and 16 bits for the green channel.
    Rg16Typeless = DXGI_FORMAT_R16G16_TYPELESS.0,

    /// A two-component, 32-bit floating-point format that supports 16 bits for the red channel and 16 bits for the green channel.
    Rg16Float = DXGI_FORMAT_R16G16_FLOAT.0,

    /// A two-component, 32-bit unsigned-normalized-integer format that supports 16 bits each for the green and red channels.
    Rg16Unorm = DXGI_FORMAT_R16G16_UNORM.0,

    /// A two-component, 32-bit unsigned-integer format that supports 16 bits for the red channel and 16 bits for the green channel.
    Rg16Uint = DXGI_FORMAT_R16G16_UINT.0,

    /// A two-component, 32-bit signed-normalized-integer format that supports 16 bits each for the green and red channels.
    Rg16Snorm = DXGI_FORMAT_R16G16_SNORM.0,

    /// A two-component, 32-bit signed-integer format that supports 16 bits for the red channel and 16 bits for the green channel.
    Rg16Sint = DXGI_FORMAT_R16G16_SINT.0,

    /// A single-component, 32-bit typeless format that supports 32 bits for the red channel.
    R32Typeless = DXGI_FORMAT_R32_TYPELESS.0,

    /// A single-component, 32-bit floating-point format that supports 32 bits for the red channel.
    D32Float = DXGI_FORMAT_D32_FLOAT.0,

    /// A single-component, 32-bit floating-point format that supports 32 bits for the red channel.
    R32Float = DXGI_FORMAT_R32_FLOAT.0,

    /// A single-component, 32-bit unsigned-integer format that supports 32 bits for the red channel.
    R32Uint = DXGI_FORMAT_R32_UINT.0,

    /// A single-component, 32-bit signed-integer format that supports 32 bits for the red channel.
    R32Sint = DXGI_FORMAT_R32_SINT.0,

    /// A two-component, 32-bit typeless format that supports 24 bits for the red channel and 8 bits for the green channel.
    R24G8Typeless = DXGI_FORMAT_R24G8_TYPELESS.0,

    /// A 32-bit z-buffer format that supports 24 bits for depth and 8 bits for stencil.
    D24UnormS8Uint = DXGI_FORMAT_D24_UNORM_S8_UINT.0,

    /// A 32-bit format, that contains a 24 bit, single-component, unsigned-normalized integer, with an additional typeless 8 bits. This format has 24 bits red channel and 8 bits unused.
    R24UnormX8Typeless = DXGI_FORMAT_R24_UNORM_X8_TYPELESS.0,

    /// A 32-bit format, that contains a 24 bit, single-component, typeless format, with an additional 8 bit unsigned integer component. This format has 24 bits unused and 8 bits green channel.
    X24TypelessG8Uint = DXGI_FORMAT_X24_TYPELESS_G8_UINT.0,

    /// A two-component, 16-bit typeless format that supports 8 bits for the red channel and 8 bits for the green channel.
    Rg8Typeless = DXGI_FORMAT_R8G8_TYPELESS.0,

    /// A two-component, 16-bit unsigned-normalized-integer format that supports 8 bits for the red channel and 8 bits for the green channel.
    Rg8Unorm = DXGI_FORMAT_R8G8_UNORM.0,

    /// A two-component, 16-bit unsigned-integer format that supports 8 bits for the red channel and 8 bits for the green channel.
    Rg8Uint = DXGI_FORMAT_R8G8_UINT.0,

    /// A two-component, 16-bit signed-normalized-integer format that supports 8 bits for the red channel and 8 bits for the green channel.
    Rg8Snorm = DXGI_FORMAT_R8G8_SNORM.0,

    /// A two-component, 16-bit signed-integer format that supports 8 bits for the red channel and 8 bits for the green channel.
    Rg8Sint = DXGI_FORMAT_R8G8_SINT.0,

    /// A single-component, 16-bit typeless format that supports 16 bits for the red channel.
    R16Typeless = DXGI_FORMAT_R16_TYPELESS.0,

    /// A single-component, 16-bit floating-point format that supports 16 bits for the red channel.
    R16Float = DXGI_FORMAT_R16_FLOAT.0,

    /// A single-component, 16-bit unsigned-normalized-integer format that supports 16 bits for depth.
    D16Unorm = DXGI_FORMAT_D16_UNORM.0,

    /// A single-component, 16-bit unsigned-normalized-integer format that supports 16 bits for the red channel.
    R16Unorm = DXGI_FORMAT_R16_UNORM.0,

    /// A single-component, 16-bit unsigned-integer format that supports 16 bits for the red channel.
    R16Uint = DXGI_FORMAT_R16_UINT.0,

    /// A single-component, 16-bit signed-normalized-integer format that supports 16 bits for the red channel.
    R16Snorm = DXGI_FORMAT_R16_SNORM.0,

    /// A single-component, 16-bit signed-integer format that supports 16 bits for the red channel.
    R16Sint = DXGI_FORMAT_R16_SINT.0,

    /// A single-component, 8-bit typeless format that supports 8 bits for the red channel.
    R8Typeless = DXGI_FORMAT_R8_TYPELESS.0,

    /// A single-component, 8-bit unsigned-normalized-integer format that supports 8 bits for the red channel.
    R8Unorm = DXGI_FORMAT_R8_UNORM.0,

    /// A single-component, 8-bit unsigned-integer format that supports 8 bits for the red channel.
    R8Uint = DXGI_FORMAT_R8_UINT.0,

    /// A single-component, 8-bit signed-normalized-integer format that supports 8 bits for the red channel.
    R8Snorm = DXGI_FORMAT_R8_SNORM.0,

    /// A single-component, 8-bit signed-integer format that supports 8 bits for the red channel.
    R8Sint = DXGI_FORMAT_R8_SINT.0,

    /// A single-component, 8-bit unsigned-normalized-integer format for alpha only.
    A8Unorm = DXGI_FORMAT_A8_UNORM.0,

    /// A single-component, 1-bit unsigned-normalized integer format that supports 1 bit for the red channel.
    R1Unorm = DXGI_FORMAT_R1_UNORM.0,

    /// Three partial-precision floating-point numbers encoded into a single 32-bit value all sharing the same 5-bit exponent (variant of s10e5, which is sign bit, 10-bit mantissa, and 5-bit biased (15) exponent).
    Rgb9E5 = DXGI_FORMAT_R9G9B9E5_SHAREDEXP.0,

    /// A four-component, 32-bit unsigned-normalized-integer format.
    /// This packed RGB format is analogous to the UYVY format. Each 32-bit block describes a pair of pixels: (R8, G8, B8) and (R8, G8, B8) where the R8/B8 values are repeated, and the G8 values are unique to each pixel.
    Rg8Bg8Unorm = DXGI_FORMAT_R8G8_B8G8_UNORM.0,

    /// A four-component, 32-bit unsigned-normalized-integer format. This packed RGB format is analogous to the YUY2 format.
    /// Each 32-bit block describes a pair of pixels: (R8, G8, B8) and (R8, G8, B8) where the R8/B8 values are repeated, and the G8 values are unique to each pixel.
    Gr8Gb8Unorm = DXGI_FORMAT_G8R8_G8B8_UNORM.0,

    /// Four-component typeless block-compression format.
    Bc1Typeless = DXGI_FORMAT_BC1_TYPELESS.0,

    /// Four-component block-compression format.
    Bc1Unorm = DXGI_FORMAT_BC1_UNORM.0,

    /// Four-component block-compression format for sRGB data.
    Bc1UnormSrgb = DXGI_FORMAT_BC1_UNORM_SRGB.0,

    /// Four-component typeless block-compression format.
    Bc2Typeless = DXGI_FORMAT_BC2_TYPELESS.0,

    /// Four-component block-compression format.
    Bc2Unorm = DXGI_FORMAT_BC2_UNORM.0,

    /// Four-component block-compression format for sRGB data.
    Bc2UnormSrgb = DXGI_FORMAT_BC2_UNORM_SRGB.0,

    /// Four-component typeless block-compression format.
    Bc3Typeless = DXGI_FORMAT_BC3_TYPELESS.0,

    /// Four-component block-compression format.
    Bc3Unorm = DXGI_FORMAT_BC3_UNORM.0,

    /// Four-component block-compression format for sRGB data.
    Bc3UnormSrgb = DXGI_FORMAT_BC3_UNORM_SRGB.0,

    /// Four-component typeless block-compression format.
    Bc4Typeless = DXGI_FORMAT_BC4_TYPELESS.0,

    /// Four-component block-compression format.
    Bc4Unorm = DXGI_FORMAT_BC4_UNORM.0,

    /// Four-component block-compression format for sRGB data.
    Bc4Snorm = DXGI_FORMAT_BC4_SNORM.0,

    /// Four-component typeless block-compression format.
    Bc5Typeless = DXGI_FORMAT_BC5_TYPELESS.0,

    /// Four-component block-compression format.
    Bc5Unorm = DXGI_FORMAT_BC5_UNORM.0,

    /// Four-component block-compression format for sRGB data.
    Bc5Snorm = DXGI_FORMAT_BC5_SNORM.0,

    /// A three-component, 16-bit unsigned-normalized-integer format that supports 5 bits for blue, 6 bits for green, and 5 bits for red.
    B5G6R5Unorm = DXGI_FORMAT_B5G6R5_UNORM.0,

    /// A four-component, 16-bit unsigned-normalized-integer format that supports 5 bits for each color channel and 1-bit alpha.
    B5G6R5A1Unorm = DXGI_FORMAT_B5G5R5A1_UNORM.0,

    /// A four-component, 32-bit unsigned-normalized-integer format that supports 8 bits for each color channel and 8-bit alpha.
    Bgra8Unorm = DXGI_FORMAT_B8G8R8A8_UNORM.0,

    ///A four-component, 32-bit unsigned-normalized-integer format that supports 8 bits for each color channel and 8 bits unused.
    Bgrx8Unorm = DXGI_FORMAT_B8G8R8X8_UNORM.0,

    /// A four-component, 32-bit 2.8-biased fixed-point format that supports 10 bits for each color channel and 2-bit alpha.
    Rgb10XRBiasA2Unorm = DXGI_FORMAT_R10G10B10_XR_BIAS_A2_UNORM.0,

    /// A four-component, 32-bit typeless format that supports 8 bits for each channel including alpha.
    Bgra8Typeless = DXGI_FORMAT_B8G8R8A8_TYPELESS.0,

    /// A four-component, 32-bit unsigned-normalized standard RGB format that supports 8 bits for each channel including alpha.
    Bgra8UnormSrgb = DXGI_FORMAT_B8G8R8A8_UNORM_SRGB.0,

    /// A four-component, 32-bit typeless format that supports 8 bits for each color channel, and 8 bits are unused.
    Bgrx8Typeless = DXGI_FORMAT_B8G8R8X8_TYPELESS.0,

    /// A four-component, 32-bit unsigned-normalized standard RGB format that supports 8 bits for each color channel, and 8 bits are unused.
    Bgrx8UnormSrgb = DXGI_FORMAT_B8G8R8X8_UNORM_SRGB.0,

    /// A typeless block-compression format.
    Bc6hTypeless = DXGI_FORMAT_BC6H_TYPELESS.0,

    /// A block-compression format.
    Bc6hUf16 = DXGI_FORMAT_BC6H_UF16.0,

    /// A block-compression format.
    Bc6hSf16 = DXGI_FORMAT_BC6H_SF16.0,

    /// A typeless block-compression format.
    Bc7Typeless = DXGI_FORMAT_BC7_TYPELESS.0,

    /// A block-compression format.
    Bc7Unorm = DXGI_FORMAT_BC7_UNORM.0,

    /// A block-compression format.
    Bc7UnormSrgb = DXGI_FORMAT_BC7_UNORM_SRGB.0,

    /// Most common YUV 4:4:4 video resource format.
    Ayuv = DXGI_FORMAT_AYUV.0,

    /// 10-bit per channel packed YUV 4:4:4 video resource format.
    Y410 = DXGI_FORMAT_Y410.0,

    /// 16-bit per channel packed YUV 4:4:4 video resource format.
    Y416 = DXGI_FORMAT_Y416.0,

    /// Most common YUV 4:2:0 video resource format.
    Nv12 = DXGI_FORMAT_NV12.0,

    /// 10-bit per channel planar YUV 4:2:0 video resource format.
    P010 = DXGI_FORMAT_P010.0,

    /// 16-bit per channel planar YUV 4:2:0 video resource format.
    P016 = DXGI_FORMAT_P016.0,

    /// 8-bit per channel planar YUV 4:2:0 video resource format.
    Opaque420 = DXGI_FORMAT_420_OPAQUE.0,

    /// Most common YUV 4:2:2 video resource format.
    Yuy2 = DXGI_FORMAT_YUY2.0,

    /// 10-bit per channel packed YUV 4:2:2 video resource format.
    Y210 = DXGI_FORMAT_Y210.0,

    /// 16-bit per channel packed YUV 4:2:2 video resource format.
    Y216 = DXGI_FORMAT_Y216.0,

    /// Most common planar YUV 4:1:1 video resource format.
    Nv11 = DXGI_FORMAT_NV11.0,

    /// 4-bit palletized YUV format that is commonly used for DVD subpicture.
    Ai44 = DXGI_FORMAT_AI44.0,

    /// 4-bit palletized YUV format that is commonly used for DVD subpicture.
    Ia44 = DXGI_FORMAT_IA44.0,

    /// 8-bit palletized format that is used for palletized RGB data when the processor processes ISDB-T data and for palletized YUV data when the processor processes BluRay data.
    P8 = DXGI_FORMAT_P8.0,

    /// 8-bit palletized format with 8 bits of alpha that is used for palletized YUV data when the processor processes BluRay data.
    A8P8 = DXGI_FORMAT_A8P8.0,

    /// A four-component, 16-bit unsigned-normalized integer format that supports 4 bits for each channel including alpha.
    Bgra4Unorm = DXGI_FORMAT_B4G4R4A4_UNORM.0,

    /// A video format; an 8-bit version of a hybrid planar 4:2:2 format.
    P208 = DXGI_FORMAT_P208.0,

    /// An 8 bit YCbCrA 4:4 rendering format.
    V208 = DXGI_FORMAT_V208.0,

    /// An 8 bit YCbCrA 4:4:4:4 rendering format.
    V408 = DXGI_FORMAT_V408.0,
}

/// When using triangle strip primitive topology, vertex positions are interpreted as vertices of a continuous triangle “strip”.
/// There is a special index value that represents the desire to have a discontinuity in the strip, the cut index value. This enum lists the supported cut values.
///
/// For more information: [`D3D12_INDEX_BUFFER_STRIP_CUT_VALUE enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_index_buffer_strip_cut_value)
#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(i32)]
pub enum IndexBufferStripCutValue {
    /// Indicates that there is no cut value.
    Disabled = D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_DISABLED.0,

    /// Indicates that 0xFFFF should be used as the cut value.
    _0xFFFF = D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_0xFFFF.0,

    /// Indicates that 0xFFFFFFFF should be used as the cut value.
    _0xFFFFFFFF = D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_0xFFFFFFFF.0,
}

/// Identifies the type of data contained in an input slot.
///
/// For more information: [`D3D12_INPUT_CLASSIFICATION enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_input_classification)
#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(i32)]
pub enum InputSlotClass {
    /// Input data is per-vertex data.
    PerVertex = D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA.0,

    /// Input data is per-instance data.
    InstanceData = D3D12_INPUT_CLASSIFICATION_PER_INSTANCE_DATA.0,
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
#[derive(Clone, Copy, Debug, Default, FromRepr, PartialEq, Eq)]
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

/// Specifies the type of the indirect parameter.
///
/// For more information: [`D3D12_INDIRECT_ARGUMENT_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_indirect_argument_desc)
#[derive(Clone, Copy, Debug)]
pub enum IndirectArgumentDesc {
    /// Indicates the type is a Draw call.
    Draw,

    /// Indicates the type is a DrawIndexed call.
    DrawIndexed,

    /// Indicates the type is a Dispatch call.
    Dispatch,

    /// Indicates the type is a vertex buffer view.
    VertexBufferView {
        /// Specifies the slot containing the vertex buffer address.
        slot: u32,
    },

    /// Indicates the type is an index buffer view.
    IndexBufferView,

    /// Indicates the type is a constant.
    Constant {
        /// Specifies the root index of the constant.
        root_parameter_index: u32,

        /// The offset, in 32-bit values, to set the first constant of the group.
        /// Supports multi-value constants at a given root index. Root constant entries must be sorted from smallest to largest DestOffsetIn32BitValues.
        dest_offset_in32_bit_values: u32,

        /// The number of 32-bit constants that are set at the given root index. Supports multi-value constants at a given root index.
        num32_bit_values_to_set: u32,
    },

    /// Indicates the type is a constant buffer view (CBV).
    ConstantBufferView {
        /// Specifies the root index of the CBV.
        root_parameter_index: u32,
    },

    /// Indicates the type is a shader resource view (SRV).
    ShaderResourceView {
        /// Specifies the root index of the SRV.
        root_parameter_index: u32,
    },

    /// Indicates the type is an unordered access view (UAV).
    UnorderedAccessView {
        /// Specifies the root index of the UAV.
        root_parameter_index: u32,
    },
}

/// Defines constants that specify logical operations to configure for a render target.
///
/// For more information: [`D3D12_LOGIC_OP enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_logic_op)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum LogicOp {
    /// Clears the render target (0).
    #[default]
    Clear = D3D12_LOGIC_OP_CLEAR.0,

    /// Sets the render target (1).
    Set = D3D12_LOGIC_OP_SET.0,

    /// Copys the render target (s source from Pixel Shader output).
    Copy = D3D12_LOGIC_OP_COPY.0,

    /// Performs an inverted-copy of the render target (~s).
    CopyInverted = D3D12_LOGIC_OP_COPY_INVERTED.0,

    /// No operation is performed on the render target (d destination in the Render Target View).
    Noop = D3D12_LOGIC_OP_NOOP.0,

    /// Inverts the render target (~d).
    Invert = D3D12_LOGIC_OP_INVERT.0,

    /// Performs a logical AND operation on the render target (s & d).
    And = D3D12_LOGIC_OP_AND.0,

    /// Performs a logical NAND operation on the render target (~(s & d)).
    Nand = D3D12_LOGIC_OP_NAND.0,

    /// Performs a logical OR operation on the render target (s | d).
    Or = D3D12_LOGIC_OP_OR.0,

    /// Performs a logical NOR operation on the render target (~(s | d)).
    Nor = D3D12_LOGIC_OP_NOR.0,

    /// Performs a logical XOR operation on the render target (s ^ d).
    Xor = D3D12_LOGIC_OP_XOR.0,

    /// Performs a logical equal operation on the render target (~(s ^ d)).
    Equiv = D3D12_LOGIC_OP_EQUIV.0,

    /// Performs a logical AND and reverse operation on the render target (s & ~d).
    Reverse = D3D12_LOGIC_OP_AND_REVERSE.0,

    /// Performs a logical AND and invert operation on the render target (~s & d).
    AndInverted = D3D12_LOGIC_OP_AND_INVERTED.0,

    /// Performs a logical OR and reverse operation on the render target (s | ~d).
    OrReverse = D3D12_LOGIC_OP_OR_REVERSE.0,

    /// Performs a logical OR and invert operation on the render target (~s | d).
    OrInverted = D3D12_LOGIC_OP_OR_INVERTED.0,
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
/// Defines constants that specify mesh and amplification shader support.
///
/// For more information: [`D3D12_MESH_SHADER_TIER enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_mesh_shader_tier)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum MeshShaderTier {
    /// Specifies that mesh and amplification shaders are not supported.
    #[default]
    NotSupported = D3D12_MESH_SHADER_TIER_NOT_SUPPORTED.0,

    /// Specifies that mesh and amplification shaders are supported.
    Tier1 = D3D12_MESH_SHADER_TIER_1.0,
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

/// Values that indicate how the pipeline interprets vertex data that is bound to the input-assembler stage. These primitive topology values determine how the vertex data is rendered on screen.
///
/// For more information: [`D3D_PRIMITIVE_TOPOLOGY enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_primitive_topology)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum PrimitiveTopology {
    /// The IA stage has not been initialized with a primitive topology. The IA stage will not function properly unless a primitive topology is defined.
    #[default]
    Undefined = D3D_PRIMITIVE_TOPOLOGY_UNDEFINED.0,

    /// Interpret the vertex data as a list of points.
    Point = D3D_PRIMITIVE_TOPOLOGY_POINTLIST.0,

    /// Interpret the vertex data as a list of lines.
    Line = D3D_PRIMITIVE_TOPOLOGY_LINELIST.0,

    /// Interpret the vertex data as a list of triangles.
    Triangle = D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST.0,
}

/// Specifies the level of support for programmable sample positions that's offered by the adapter.
///
/// For more information: [`D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_programmable_sample_positions_tier)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum PipelinePrimitiveTopology {
    /// The shader has not been initialized with an input primitive type.
    #[default]
    Undefined = D3D12_PRIMITIVE_TOPOLOGY_TYPE_UNDEFINED.0,

    /// Interpret the input primitive as a point.
    Point = D3D12_PRIMITIVE_TOPOLOGY_TYPE_POINT.0,

    /// Interpret the input primitive as a line.
    Line = D3D12_PRIMITIVE_TOPOLOGY_TYPE_LINE.0,

    /// Interpret the input primitive as a triangle.
    Triangle = D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE.0,

    /// Interpret the input primitive as a control point patch.
    Patch = D3D12_PRIMITIVE_TOPOLOGY_TYPE_PATCH.0,
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

/// Specifies the type of query heap to create.
///
/// For more information: [`D3D12_QUERY_HEAP_TYPE enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_query_heap_type)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum QueryType {
    /// This returns a binary 0/1 result:
    /// 0 indicates that no samples passed depth and stencil testing,
    /// 1 indicates that at least one sample passed depth and stencil testing.
    /// This enables occlusion queries to not interfere with any GPU performance optimization associated with depth/stencil testing.
    #[default]
    Occlusion = D3D12_QUERY_HEAP_TYPE_OCCLUSION.0,

    /// Indicates that the heap is for high-performance timing data.
    Timestamp = D3D12_QUERY_HEAP_TYPE_TIMESTAMP.0,

    /// Indicates the heap is to contain pipeline data.
    PipelineStatistics = D3D12_QUERY_HEAP_TYPE_PIPELINE_STATISTICS.0,

    /// Indicates the heap is to contain stream output data.
    SoStatistics = D3D12_QUERY_HEAP_TYPE_SO_STATISTICS.0,

    /// Indicates the heap is to contain video decode statistics data.
    VideoDecodeStatistics = D3D12_QUERY_HEAP_TYPE_VIDEO_DECODE_STATISTICS.0,

    /// Indicates the heap is to contain timestamp queries emitted exclusively by copy command lists.
    /// Copy queue timestamps can only be queried from a copy command list, and a copy command list can not emit to a regular timestamp query Heap.
    CopyQueueTimestamp = D3D12_QUERY_HEAP_TYPE_COPY_QUEUE_TIMESTAMP.0,

    /// TBD
    PipelineStatistics1 = D3D12_QUERY_HEAP_TYPE_PIPELINE_STATISTICS1.0,
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

/// Describes the blend state for a render target.
///
/// For more information: [`D3D12_RENDER_TARGET_BLEND_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_render_target_blend_desc)
#[derive(Clone, Debug, Default)]
pub enum RenderTargetBlendDesc {
    /// No blend or logic op.
    #[default]
    None,
    /// Specifies whether to enable blending.
    Blend {
        /// A [`Blend`]-typed value that specifies the operation to perform on the RGB value that the pixel shader outputs. The BlendOp member defines how to combine the src_blend and dest_blend operations.
        src_blend: Blend,

        /// A [`Blend`]-typed value that specifies the operation to perform on the current RGB value in the render target. The BlendOp member defines how to combine the src_blend and dest_blend operations.
        dst_blend: Blend,

        /// A [`BlendOp]-typed value that defines how to combine the SrcBlend and DestBlend operations.
        blend_op: BlendOp,

        /// A [`Blend`]-typed value that specifies the operation to perform on the alpha value that the pixel shader outputs.
        /// Blend options that end in _COLOR are not allowed. The BlendOpAlpha member defines how to combine the src_blend_alpha and dst_blend_alpha operations.
        src_blend_alpha: Blend,

        /// A [`Blend`]-typed value that specifies the operation to perform on the current alpha value in the render target.
        /// Blend options that end in _COLOR are not allowed. The BlendOpAlpha member defines how to combine the src_blend_alpha and dst_blend_alpha operations.
        dst_blend_alpha: Blend,

        /// A [`BlendOp`]-typed value that defines how to combine the SrcBlendAlpha and DestBlendAlpha operations.
        blend_op_alpha: BlendOp,

        /// A combination of [`ColorWriteEnable`]-typed values that are combined by using a bitwise OR operation. The resulting value specifies a write mask.
        mask: ColorWriteEnable,
    },
    /// Specifies whether to enable a logical operation.
    Logic {
        /// A [`LogicOp`]-typed value that specifies the logical operation to configure for the render target.
        logic_op: LogicOp,

        /// A combination of [`ColorWriteEnable`]-typed values that are combined by using a bitwise OR operation. The resulting value specifies a write mask.
        mask: ColorWriteEnable,
    },
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

/// Identifies the type of resource being used.
///
/// For more information: [`D3D12_RESOURCE_DIMENSION enumeration `](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_resource_dimension)
#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(i32)]
pub enum ResourceDimension {
    /// Resource is of unknown type.
    Unknown = D3D12_RESOURCE_DIMENSION_UNKNOWN.0,

    /// Resource is a buffer.
    Buffer = D3D12_RESOURCE_DIMENSION_BUFFER.0,

    /// Resource is a 1D texture.
    Texture1D = D3D12_RESOURCE_DIMENSION_TEXTURE1D.0,

    /// Resource is a 2D texture.
    Texture2D = D3D12_RESOURCE_DIMENSION_TEXTURE2D.0,

    /// Resource is a 3D texture.
    Texture3D = D3D12_RESOURCE_DIMENSION_TEXTURE3D.0,
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

/// Specifies the type of root signature slot.
///
/// For more information: [`D3D12_ROOT_PARAMETER_TYPE enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_root_parameter_type)
#[derive(Clone, Debug)]
pub enum RootParameterType<'a> {
    /// The slot is for a descriptor table.
    DescriptorTable {
        /// An array of [`DescriptorRange`] structures that describe the descriptor ranges.
        ranges: &'a [DescriptorRange],
    },

    /// The slot is for root constants.
    Constants32Bit {
        /// The shader register.
        shader_register: u32,

        /// The register space.
        register_space: u32,

        /// The number of constants that occupy a single shader slot (these constants appear like a single constant buffer). All constants occupy a single root signature bind slot.
        num_32bit_values: u32,
    },

    /// The slot is for a constant-buffer view (CBV).
    Cbv {
        /// The shader register.
        shader_register: u32,

        /// The register space.
        register_space: u32,
    },

    /// The slot is for a shader-resource view (SRV).
    Srv {
        /// The shader register.
        shader_register: u32,

        /// The register space.
        register_space: u32,
    },

    /// The slot is for a unordered-access view (UAV).
    Uav {
        /// The shader register.
        shader_register: u32,

        /// The register space.
        register_space: u32,
    },
}

/// Identifies the type of resource to view as a render target.
///
/// For more information: [`D3D12_RTV_DIMENSION enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_rtv_dimension)
#[derive(Clone, Copy, Debug)]
pub enum RtvDimension {
    /// The resource will be accessed as a buffer.
    Buffer {
        /// Number of elements between the beginning of the buffer and the first element to access.
        first_element: u64,

        /// The total number of elements in the view.
        num_elements: u32,
    },

    /// The resource will be accessed as a 1D texture.
    Tex1D {
        /// The index of the mipmap level to use mip slice.
        mip_slice: u32,
    },

    // The resource will be accessed as an array of 1D textures.
    ArrayTex1D {
        /// The index of the mipmap level to use mip slice.
        mip_slice: u32,

        /// The index of the first texture to use in an array of textures.
        first_array_slice: u32,

        /// Number of textures to use.
        array_size: u32,
    },

    /// The resource will be accessed as a 2D texture.
    Tex2D {
        /// The index of the mipmap level to use.
        mip_slice: u32,

        /// The index (plane slice number) of the plane to use in the texture.
        plane_slice: u32,
    },

    /// The resource will be accessed as an array of 2D textures.
    ArrayTex2D {
        /// The index of the mipmap level to use mip slice.
        mip_slice: u32,

        /// The index of the first texture to use in an array of textures.
        plane_slice: u32,

        /// Number of textures in the array to use in the render target view, starting from FirstArraySlice.
        first_array_slice: u32,

        /// The index (plane slice number) of the plane to use in an array of textures.
        array_size: u32,
    },

    /// The resource will be accessed as a 2D texture with multisampling.
    Tex2DMs,

    /// The resource will be accessed as an array of 2D textures with multisampling.
    Array2DMs {
        /// The index of the first texture to use in an array of textures.
        first_array_slice: u32,

        /// The number of textures to use.
        array_size: u32,
    },

    /// The resource will be accessed as a 3D texture.
    Tex3D {
        /// The index of the mipmap level to use mip slice.
        mip_slice: u32,

        /// First depth level to use.
        first_w_slice: u32,

        /// Number of depth levels to use in the render-target view, starting from FirstWSlice. A value of -1 indicates all of the slices along the w axis, starting from FirstWSlice.
        w_size: u32,
    },
}

/// Defines constants that specify sampler feedback support.
///
/// For more information: [`D3D12_SAMPLER_FEEDBACK_TIER enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_sampler_feedback_tier)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum SamplerFeedbackTier {
    /// Specifies that sampler feedback is not supported. Attempts at calling sampler feedback APIs represent an error.
    #[default]
    NoSupported = D3D12_SAMPLER_FEEDBACK_TIER_NOT_SUPPORTED.0,

    /// Specifies that sampler feedback is supported to tier 0.9.
    Tier0_9 = D3D12_SAMPLER_FEEDBACK_TIER_0_9.0,

    /// Specifies sample feedback is supported to tier 1.0.
    /// This indicates that sampler feedback is supported for all texture addressing modes, and feedback-writing methods are supported irrespective of the passed-in
    /// shader resource view.
    Tier1_0 = D3D12_SAMPLER_FEEDBACK_TIER_1_0.0,
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

/// Specifies the shaders that can access the contents of a given root signature slot.
///
/// For more information: [`D3D12_SHADER_VISIBILITY enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_shader_visibility)
#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(i32)]
pub enum ShaderVisibility {
    /// Specifies that all shader stages can access whatever is bound at the root signature slot.
    All = D3D12_SHADER_VISIBILITY_ALL.0,

    /// Specifies that the vertex shader stage can access whatever is bound at the root signature slot.
    Vertex = D3D12_SHADER_VISIBILITY_VERTEX.0,

    /// Specifies that the hull shader stage can access whatever is bound at the root signature slot.
    Hull = D3D12_SHADER_VISIBILITY_HULL.0,

    /// Specifies that the domain shader stage can access whatever is bound at the root signature slot.
    Domain = D3D12_SHADER_VISIBILITY_DOMAIN.0,

    /// Specifies that the geometry shader stage can access whatever is bound at the root signature slot.
    Geometry = D3D12_SHADER_VISIBILITY_GEOMETRY.0,

    /// Specifies that the pixel shader stage can access whatever is bound at the root signature slot.
    Pixel = D3D12_SHADER_VISIBILITY_PIXEL.0,

    /// Specifies that the amplification shader stage can access whatever is bound at the root signature slot.
    Amplification = D3D12_SHADER_VISIBILITY_AMPLIFICATION.0,

    /// Specifies that the mesh shader stage can access whatever is bound at the root signature slot.
    Mesh = D3D12_SHADER_VISIBILITY_MESH.0,
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

/// Identifies the type of resource that will be viewed as a shader resource.
///
/// For more information: [`D3D12_SRV_DIMENSION enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_srv_dimension)
#[derive(Clone, Copy, Debug)]
pub enum SrvDimension {
    /// Describes the elements in a buffer resource to use in a shader-resource view.
    Buffer {
        /// The index of the first element to be accessed by the view.
        first_element: u64,

        /// The number of elements in the resource.
        num_elements: u32,

        /// The size of each element in the buffer structure (in bytes) when the buffer represents a structured buffer.
        /// The size must match the struct size declared in shaders that access the view.
        structure_byte_stride: u32,

        /// A [`BufferSrvFlags`]-typed value that identifies view options for the buffer. Currently, the only option is to identify a raw view of the buffer.
        flags: BufferSrvFlags,
    },

    /// Specifies the subresource from a 1D texture to use in a shader-resource view.
    Tex1D {
        /// Index of the most detailed mipmap level to use; this number is between 0 and MipLevels
        most_detailed_mip: u32,

        /// The maximum number of mipmap levels for the view of the texture.
        mip_levels: u32,

        /// Specifies the minimum mipmap level that you can access.
        resource_min_lod_clamp: f32,
    },

    /// Describes the subresources from an array of 1D textures to use in a shader-resource view.
    ArrayTex1D {
        /// Index of the most detailed mipmap level to use; this number is between 0 and MipLevels
        most_detailed_mip: u32,

        /// The maximum number of mipmap levels for the view of the texture.
        mip_levels: u32,

        /// Specifies the minimum mipmap level that you can access.
        resource_min_lod_clamp: f32,

        /// The index of the first texture to use in an array of textures.
        first_array_slice: u32,

        /// Number of textures in the array.
        array_size: u32,
    },

    /// Describes the subresource from a 2D texture to use in a shader-resource view.
    Tex2D {
        /// Index of the most detailed mipmap level to use; this number is between 0 and MipLevels
        most_detailed_mip: u32,

        /// The maximum number of mipmap levels for the view of the texture.
        mip_levels: u32,

        /// Specifies the minimum mipmap level that you can access.
        resource_min_lod_clamp: f32,

        /// The index (plane slice number) of the plane to use in the texture.
        plane_slice: u32,
    },

    /// Describes the subresources from an array of 2D textures to use in a shader-resource view.
    ArrayTex2D {
        /// Index of the most detailed mipmap level to use; this number is between 0 and MipLevels
        most_detailed_mip: u32,

        /// The maximum number of mipmap levels for the view of the texture.
        mip_levels: u32,

        /// Specifies the minimum mipmap level that you can access.
        resource_min_lod_clamp: f32,

        /// The index (plane slice number) of the plane to use in the texture.
        plane_slice: u32,

        /// Number of textures in the array to use in the render target view, starting from FirstArraySlice.
        first_array_slice: u32,

        /// The index (plane slice number) of the plane to use in an array of textures.
        array_size: u32,
    },

    /// Describes the subresources from a multi sampled 2D texture to use in a shader-resource view.
    Tex2DMs,

    /// Describes the subresources from an array of multi sampled 2D textures to use in a shader-resource view.
    Array2DMs {
        /// The index of the first texture to use in an array of textures.
        first_array_slice: u32,

        /// The number of textures to use.
        array_size: u32,
    },

    /// Describes the subresources from a 3D texture to use in a shader-resource view.
    Tex3D {
        /// Index of the most detailed mipmap level to use; this number is between 0 and MipLevels
        most_detailed_mip: u32,

        /// The maximum number of mipmap levels for the view of the texture.
        mip_levels: u32,

        /// Specifies the minimum mipmap level that you can access.
        resource_min_lod_clamp: f32,
    },

    /// Describes the subresource from a cube texture to use in a shader-resource view.
    TexCube {
        /// Index of the most detailed mipmap level to use; this number is between 0 and MipLevels
        most_detailed_mip: u32,

        /// The maximum number of mipmap levels for the view of the texture.
        mip_levels: u32,

        /// Specifies the minimum mipmap level that you can access.
        resource_min_lod_clamp: f32,
    },

    /// Describes the subresources from an array of cube textures to use in a shader-resource view.
    ArrayCube {
        /// Index of the most detailed mipmap level to use; this number is between 0 and MipLevels
        most_detailed_mip: u32,

        /// The maximum number of mipmap levels for the view of the texture.
        mip_levels: u32,

        /// Specifies the minimum mipmap level that you can access.
        resource_min_lod_clamp: f32,

        /// Index of the first 2D texture to use.
        first_2d_array_face: u32,

        /// Number of cube textures in the array.
        num_cubes: u32,
    },

    /// A shader resource view (SRV) structure for storing a raytracing acceleration structure.
    RaytracingAccelerationStructure {
        /// The GPU virtual address of the SRV.
        location: u64,
    },
}

/// Identifies the stencil operations that can be performed during depth-stencil testing.
///
/// For more information: [`D3D12_STENCIL_OP enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_stencil_op)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum StencilOp {
    /// Keep the existing stencil data.
    #[default]
    Keep = D3D12_STENCIL_OP_KEEP.0,

    /// Set the stencil data to 0.
    Zero = D3D12_STENCIL_OP_ZERO.0,

    /// Set the stencil data to the reference value set by calling [`GraphicsCommandListInterface::om_set_stencil_ref`](crate::command_list::GraphicsCommandListInterface).
    Replace = D3D12_STENCIL_OP_REPLACE.0,

    /// Increment the stencil value by 1, and clamp the result.
    IncrSat = D3D12_STENCIL_OP_INCR_SAT.0,

    /// Decrement the stencil value by 1, and clamp the result.
    DecrSat = D3D12_STENCIL_OP_DECR_SAT.0,

    /// Invert the stencil data.
    Invert = D3D12_STENCIL_OP_INVERT.0,

    /// Increment the stencil value by 1, and wrap the result if necessary.
    Incr = D3D12_STENCIL_OP_INCR.0,

    /// Decrement the stencil value by 1, and wrap the result if necessary.
    Decr = D3D12_STENCIL_OP_DECR.0,
}

/// Specifies texture layout options.
///
/// For more information: [`D3D12_TEXTURE_LAYOUT enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_texture_layout)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum TextureLayout {
    /// Indicates that the layout is unknown, and is likely adapter-dependent.
    ///
    /// During creation, the driver chooses the most efficient layout based on other resource properties, especially resource size and flags.
    ///
    /// Prefer this choice unless certain functionality is required from another texture layout.
    #[default]
    Unknown = D3D12_TEXTURE_LAYOUT_UNKNOWN.0,

    /// Indicates that data for the texture is stored in row-major order (sometimes called "pitch-linear order").
    RowMajor = D3D12_TEXTURE_LAYOUT_ROW_MAJOR.0,

    /// Indicates that the layout within 64KB tiles and tail mip packing is up to the driver.
    UndefinedSwizzle64Kb = D3D12_TEXTURE_LAYOUT_64KB_UNDEFINED_SWIZZLE.0,

    /// Indicates that a default texture uses the standardized swizzle pattern.
    StandardSwizzle64Kb = D3D12_TEXTURE_LAYOUT_64KB_STANDARD_SWIZZLE.0,
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

/// Defines constants that specify a shading rate tier (for variable-rate shading, or VRS).
///
/// For more information: [`D3D12_VARIABLE_SHADING_RATE_TIER enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_variable_shading_rate_tier)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum VariableShadingRateTier {
    ///Specifies that variable-rate shading is not supported.
    #[default]
    NotSupported = D3D12_VARIABLE_SHADING_RATE_TIER_NOT_SUPPORTED.0,

    /// Specifies that variable-rate shading tier 1 is supported.
    Tier1 = D3D12_VARIABLE_SHADING_RATE_TIER_1.0,

    /// Specifies that variable-rate shading tier 2 is supported.
    Tier2 = D3D12_VARIABLE_SHADING_RATE_TIER_2.0,
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

/// Defines constants that specify a level of support for WaveMMA (wave_matrix) operations.
///
/// For more information: [`D3D12_WAVE_MMA_TIER  enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_wave_mma_tier)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum WaveMmaTier {
    /// Specifies that WaveMMA (wave_matrix) operations are not supported.
    #[default]
    NotSupported = D3D12_WAVE_MMA_TIER_NOT_SUPPORTED.0,

    /// Specifies that WaveMMA (wave_matrix) operations are supported.
    Tier1_0 = D3D12_WAVE_MMA_TIER_1_0.0,
}

/// Identifies unordered-access view options.
///
/// For more information: [`D3D12_UAV_DIMENSION enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_uav_dimension)
#[derive(Clone, Copy, Debug)]
pub enum UavDimension {
    /// Describes the elements in a buffer to use in a unordered-access view.
    Buffer {
        /// The zero-based index of the first element to be accessed.
        first_element: u64,

        /// The number of elements in the resource. For structured buffers, this is the number of structures in the buffer.
        num_elements: u32,

        /// The size of each element in the buffer structure (in bytes) when the buffer represents a structured buffer.
        structure_byte_stride: u32,

        /// The counter offset, in bytes.
        counter_offset: u64,

        /// A [`BufferUavFlags`]-typed value that specifies the view options for the resource.
        flags: BufferUavFlags,
    },

    /// Describes a unordered-access 1D texture resource.
    Tex1D {
        /// The mipmap slice index.
        mip_slice: u32,
    },

    // Describes an array of unordered-access 1D texture resources.
    ArrayTex1D {
        /// The mipmap slice index.
        mip_slice: u32,

        /// The zero-based index of the first array slice to be accessed.
        first_array_slice: u32,

        /// The number of slices in the array.
        array_size: u32,
    },

    /// Describes a unordered-access 2D texture resource.
    Tex2D {
        /// The mipmap slice index.
        mip_slice: u32,

        /// The index (plane slice number) of the plane to use in the texture.
        plane_slice: u32,
    },

    /// Describes an array of unordered-access 2D texture resources.
    ArrayTex2D {
        /// The mipmap slice index.
        mip_slice: u32,

        /// NThe zero-based index of the first array slice to be accessed.
        first_array_slice: u32,

        /// The number of slices in the array.
        array_size: u32,

        /// The index (plane slice number) of the plane to use in an array of textures.
        plane_slice: u32,
    },

    /// TBD
    Tex2DMs,

    /// TBD
    Array2DMs {
        /// The index of the first texture to use in an array of textures.
        first_array_slice: u32,

        /// The number of textures to use.
        array_size: u32,
    },

    /// The resource will be accessed as a 3D texture.
    Tex3D {
        /// The mipmap slice index.
        mip_slice: u32,

        /// The zero-based index of the first depth slice to be accessed.
        first_w_slice: u32,

        /// The number of depth slices.
        w_size: u32,
    },
}

#[derive(Clone, Copy, Debug)]
pub enum TextureCopyType {
    SubresourceIndex(u32),
    PlacedFootprint(PlacedSubresourceFootprint),
}

#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(i32)]
pub enum PredicationOp {
    EqualZero = D3D12_PREDICATION_OP_EQUAL_ZERO.0,
    NotEqualZero = D3D12_PREDICATION_OP_NOT_EQUAL_ZERO.0,
}
