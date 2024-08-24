use std::ffi::CStr;

use strum::FromRepr;
use windows::Win32::Graphics::{Direct3D::*, Direct3D12::*};

#[allow(unused_imports)]
use super::*;

/// Identifies a technique for resolving texture coordinates that are outside of the boundaries of a texture.
///
/// For more information: [`D3D12_TEXTURE_ADDRESS_MODE enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_texture_address_mode)
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
#[repr(i32)]
pub enum AddressMode {
    /// Tile the texture at every (u,v) integer junction.
    #[default]
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

/// Identifies the alpha value, transparency behavior, of a surface.
///
/// For more information: [`DXGI_ALPHA_MODE enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_2/ne-dxgi1_2-dxgi_alpha_mode)
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
#[repr(i32)]
pub enum AlphaMode {
    /// Indicates that the transparency behavior is not specified.
    #[default]
    Unspecified = DXGI_ALPHA_MODE_UNSPECIFIED.0,

    /// Indicates that the transparency behavior is premultiplied. Each color is first scaled by the alpha value.
    /// The alpha value itself is the same in both straight and premultiplied alpha.
    /// Typically, no color channel value is greater than the alpha channel value.
    /// If a color channel value in a premultiplied format is greater than the alpha channel,
    /// the standard source-over blending math results in an additive blend.
    Premultiplied = DXGI_ALPHA_MODE_PREMULTIPLIED.0,

    /// Indicates that the transparency behavior is not premultiplied. The alpha channel indicates the transparency of the color.
    Straight = DXGI_ALPHA_MODE_STRAIGHT.0,

    /// Indicates to ignore the transparency behavior.
    Ignore = DXGI_ALPHA_MODE_IGNORE.0,
}

/// Specifies blend factors, which modulate values for the pixel shader and render target.
///
/// For more information: [`D3D12_BLEND enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_blend)
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
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

    /// The blend factor is the blend factor set with [`IGraphicsCommandList::om_set_blend_factor`](crate::command_list::IGraphicsCommandList::om_set_blend_factor). No pre-blend operation.
    BlendFactor = D3D12_BLEND_BLEND_FACTOR.0,

    /// The blend factor is the blend factor set with [`IGraphicsCommandList::om_set_blend_factor`](crate::command_list::IGraphicsCommandList::om_set_blend_factor). The pre-blend operation inverts the blend factor, generating 1 - blend_factor.
    InvBlendFactor = D3D12_BLEND_INV_BLEND_FACTOR.0,

    /// The blend factor is data sources both as color data output by a pixel shader. There is no pre-blend operation. This blend factor supports dual-source color blending.
    Src1Color = D3D12_BLEND_SRC1_COLOR.0,

    /// The blend factor is data sources both as color data output by a pixel shader. The pre-blend operation inverts the data, generating 1 - RGB. This blend factor supports dual-source color blending.
    InvSrc1Color = D3D12_BLEND_INV_SRC1_COLOR.0,

    /// The blend factor is data sources as alpha data output by a pixel shader. There is no pre-blend operation. This blend factor supports dual-source color blending.
    Src1Alpha = D3D12_BLEND_SRC1_ALPHA.0,

    /// The blend factor is data sources as alpha data output by a pixel shader. The pre-blend operation inverts the data, generating 1 - A. This blend factor supports dual-source color blending.
    InvSrc1Alpha = D3D12_BLEND_INV_SRC1_ALPHA.0,

    /// The blend factor is (A, A, A, A), where the constant, A, is taken from the blend factor set with [`IGraphicsCommandList::om_set_blend_factor`](crate::command_list::IGraphicsCommandList::om_set_blend_factor).
    AlphaFactor = D3D12_BLEND_ALPHA_FACTOR.0,

    /// The blend factor is (1 – A, 1 – A, 1 – A, 1 – A), where the constant, A, is taken from the blend factor set with [`IGraphicsCommandList::om_set_blend_factor`](crate::command_list::IGraphicsCommandList::om_set_blend_factor).
    InvAlphaFactor = D3D12_BLEND_INV_ALPHA_FACTOR.0,
}

/// Specifies RGB or alpha blending operations.
///
/// For more information: [`D3D12_BLEND_OP enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_blend_op)
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
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
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
#[repr(i32)]
pub enum BorderColor {
    /// Indicates black, with the alpha component as fully transparent.
    #[default]
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

/// Specifies the type of a command list.
///
/// For more information: [`D3D12_COMMAND_LIST_TYPE enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_command_list_type)
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
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

/// Defines priority levels for a command queue.
///
/// For more information: [`D3D12_COMMAND_QUEUE_PRIORITY enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_command_queue_priority)
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
#[repr(i32)]
pub enum CommandQueuePriority {
    /// Normal priority.
    #[default]
    Normal = D3D12_COMMAND_QUEUE_PRIORITY_NORMAL.0,

    /// High priority.
    High = D3D12_COMMAND_QUEUE_PRIORITY_HIGH.0,

    /// Global realtime priority.
    GlobalRealtime = D3D12_COMMAND_QUEUE_PRIORITY_GLOBAL_REALTIME.0,
}

/// Specifies comparison options.
///
/// For more information: [`D3D12_COMPARISON_FUNC enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_comparison_func)
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
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

/// Identifies whether conservative rasterization is on or off.
///
/// For more information: [`D3D12_CONSERVATIVE_RASTERIZATION_MODE enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_conservative_rasterization_mode)
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
#[repr(i32)]
pub enum ConservativeRaster {
    /// Conservative rasterization is off.
    #[default]
    Off = D3D12_CONSERVATIVE_RASTERIZATION_MODE_OFF.0,

    /// Conservative rasterization is on.
    On = D3D12_CONSERVATIVE_RASTERIZATION_MODE_ON.0,
}

/// Identifies the tier level of conservative rasterization.
///
/// For more information: [`D3D12_CONSERVATIVE_RASTERIZATION_TIER enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_conservative_rasterization_tier)
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
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
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
#[repr(i32)]
pub enum CpuPageProperty {
    /// The CPU-page property is unknown.
    #[default]
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
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
#[repr(i32)]
pub enum CrossNodeSharingTier {
    /// If an adapter has only 1 node, then cross-node sharing doesn't apply.
    #[default]
    NotSupported = D3D12_CROSS_NODE_SHARING_TIER_NOT_SUPPORTED.0,

    /// Tier 1 Emulated. Devices that set the [`CrossNodeSharingTier`] member of the [`Options`](crate::types::features::Options) structure to [`CrossNodeSharingTier::Tier1Emulated`] have Tier 1 support.
    ///
    /// However, drivers stage these copy operations through a driver-internal system memory allocation. This will cause these copy operations to consume time on the destination GPU as well as the source.
    Tier1Emulated = D3D12_CROSS_NODE_SHARING_TIER_1_EMULATED.0,

    /// Tier 1. Devices that set the [`CrossNodeSharingTier`] member of the [`Options`](crate::types::features::Options) structure to [`CrossNodeSharingTier::Tier1`] only support the following cross-node copy operations:
    /// * [GraphicsCommandList::copy_buffer_region](crate::command_list::IGraphicsCommandList::copy_buffer_region)
    /// * [GraphicsCommandList::copy_texture_region](crate::command_list::IGraphicsCommandList::copy_texture_region)
    /// * [GraphicsCommandList::copy_resource](crate::command_list::IGraphicsCommandList::copy_resource)
    Tier1 = D3D12_CROSS_NODE_SHARING_TIER_1.0,

    /// Tier 2. Devices that set the [`CrossNodeSharingTier`] member of the [`Options`](crate::types::features::Options) structure to D3D12_CROSS_NODE_SHARING_TIER_2 support all operations across nodes, except for the following:
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
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
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
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
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
#[derive(Clone, Copy, Debug, FromRepr, Hash, PartialEq, Eq)]
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

/// Describes the set of features targeted by a Direct3D device.
///
/// For more information: [`D3D_FEATURE_LEVEL enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_feature_level)
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
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

/// Defines constants that specify a Direct3D 12 feature or feature set to query about.
///
/// For more information: [`D3D12_FEATURE enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_feature)
#[derive(Clone, Copy, Debug, FromRepr, Hash, PartialEq, Eq)]
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

/// Specifies the fill mode to use when rendering triangles.
///
/// For more information: [`D3D12_FILL_MODE enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_fill_mode)
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
#[repr(i32)]
pub enum FillMode {
    /// Draw lines connecting the vertices. Adjacent vertices are not drawn.
    Wireframe = D3D12_FILL_MODE_WIREFRAME.0,

    /// Fill the triangles formed by the vertices. Adjacent vertices are not drawn.
    #[default]
    Solid = D3D12_FILL_MODE_SOLID.0,
}

/// Specifies filtering options during texture sampling.
///
/// For more information: [`D3D12_FILTER enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_filter)
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
#[repr(i32)]
pub enum Filter {
    /// Use point sampling for minification, magnification, and mip-level sampling.
    #[default]
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

/// Resource data formats, including fully-typed and typeless formats. A list of modifiers at the bottom of the page more fully describes each format type.
///
/// For more information: [`DXGI_FORMAT enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/dxgiformat/ne-dxgiformat-dxgi_format)
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
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

/// The preference of GPU for the app to run on.
///
/// For more information: [`DXGI_GPU_PREFERENCE enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_6/ne-dxgi1_6-dxgi_gpu_preference)
#[derive(Clone, Copy, Debug, FromRepr, Hash, PartialEq, Eq)]
#[repr(i32)]
pub enum GpuPreference {
    Unspecified = DXGI_GPU_PREFERENCE_UNSPECIFIED.0,
    MinimumPower = DXGI_GPU_PREFERENCE_MINIMUM_POWER.0,
    HighPerformance = DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE.0,
}

/// Heap alignment variants.
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
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
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
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
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
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

/// When using triangle strip primitive topology, vertex positions are interpreted as vertices of a continuous triangle “strip”.
/// There is a special index value that represents the desire to have a discontinuity in the strip, the cut index value. This enum lists the supported cut values.
///
/// For more information: [`D3D12_INDEX_BUFFER_STRIP_CUT_VALUE enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_index_buffer_strip_cut_value)
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
#[repr(i32)]
pub enum IndexBufferStripCutValue {
    /// Indicates that there is no cut value.
    #[default]
    Disabled = D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_DISABLED.0,

    /// Indicates that 0xFFFF should be used as the cut value.
    _0xFFFF = D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_0xFFFF.0,

    /// Indicates that 0xFFFFFFFF should be used as the cut value.
    _0xFFFFFFFF = D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_0xFFFFFFFF.0,
}

/// Defines constants that specify logical operations to configure for a render target.
///
/// For more information: [`D3D12_LOGIC_OP enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_logic_op)
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
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
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
#[repr(i32)]
pub enum MemoryPool {
    /// The memory pool is unknown.
    #[default]
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
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
#[repr(i32)]
pub enum MeshShaderTier {
    /// Specifies that mesh and amplification shaders are not supported.
    #[default]
    NotSupported = D3D12_MESH_SHADER_TIER_NOT_SUPPORTED.0,

    /// Specifies that mesh and amplification shaders are supported.
    Tier1 = D3D12_MESH_SHADER_TIER_1.0,
}

/// Specifies categories of debug messages.
///
/// For more information: [`D3D12_MESSAGE_CATEGORY enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/ne-d3d12sdklayers-d3d12_message_category)
#[derive(Clone, Copy, Debug, FromRepr, Hash, PartialEq, Eq)]
#[repr(i32)]
pub enum MessageCategory {
    /// Indicates a user defined message,
    ApplicationDefined = D3D12_MESSAGE_CATEGORY_APPLICATION_DEFINED.0,

    /// TBD
    Miscellaneous = D3D12_MESSAGE_CATEGORY_MISCELLANEOUS.0,

    /// TBD
    Initialization = D3D12_MESSAGE_CATEGORY_INITIALIZATION.0,

    /// TBD
    Cleanup = D3D12_MESSAGE_CATEGORY_CLEANUP.0,

    /// TBD
    Compilation = D3D12_MESSAGE_CATEGORY_COMPILATION.0,

    /// TBD
    StateCreation = D3D12_MESSAGE_CATEGORY_STATE_CREATION.0,

    /// TBD
    StateSettings = D3D12_MESSAGE_CATEGORY_STATE_SETTING.0,

    /// TBD
    StateGetting = D3D12_MESSAGE_CATEGORY_STATE_GETTING.0,

    /// TBD
    ResourceManipulation = D3D12_MESSAGE_CATEGORY_RESOURCE_MANIPULATION.0,

    /// TBD
    Execution = D3D12_MESSAGE_CATEGORY_EXECUTION.0,

    /// TBD
    Shader = D3D12_MESSAGE_CATEGORY_SHADER.0,
}

/// Specifies debug message IDs for setting up an info-queue filter.
///
/// For more information: [`D3D12_MESSAGE_ID enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/ne-d3d12sdklayers-d3d12_message_id)
#[derive(Clone, Copy, Debug, FromRepr, Hash, PartialEq, Eq)]
#[repr(i32)]
pub enum MessageId {
    Unknown = D3D12_MESSAGE_ID_UNKNOWN.0,
    StringFromApplication = D3D12_MESSAGE_ID_STRING_FROM_APPLICATION.0,
    CorruptedThis = D3D12_MESSAGE_ID_CORRUPTED_THIS.0,
    CorruptedParameter1 = D3D12_MESSAGE_ID_CORRUPTED_PARAMETER1.0,
    CorruptedParameter2 = D3D12_MESSAGE_ID_CORRUPTED_PARAMETER2.0,
    CorruptedParameter3 = D3D12_MESSAGE_ID_CORRUPTED_PARAMETER3.0,
    CorruptedParameter4 = D3D12_MESSAGE_ID_CORRUPTED_PARAMETER4.0,
    CorruptedParameter5 = D3D12_MESSAGE_ID_CORRUPTED_PARAMETER5.0,
    CorruptedParameter6 = D3D12_MESSAGE_ID_CORRUPTED_PARAMETER6.0,
    CorruptedParameter7 = D3D12_MESSAGE_ID_CORRUPTED_PARAMETER7.0,
    CorruptedParameter8 = D3D12_MESSAGE_ID_CORRUPTED_PARAMETER8.0,
    CorruptedParameter9 = D3D12_MESSAGE_ID_CORRUPTED_PARAMETER9.0,
    CorruptedParameter10 = D3D12_MESSAGE_ID_CORRUPTED_PARAMETER10.0,
    CorruptedParameter11 = D3D12_MESSAGE_ID_CORRUPTED_PARAMETER11.0,
    CorruptedParameter12 = D3D12_MESSAGE_ID_CORRUPTED_PARAMETER12.0,
    CorruptedParameter13 = D3D12_MESSAGE_ID_CORRUPTED_PARAMETER13.0,
    CorruptedParameter14 = D3D12_MESSAGE_ID_CORRUPTED_PARAMETER14.0,
    CorruptedParameter15 = D3D12_MESSAGE_ID_CORRUPTED_PARAMETER15.0,
    CorruptedMultithreading = D3D12_MESSAGE_ID_CORRUPTED_MULTITHREADING.0,
    MessageReportingOutOfMemory = D3D12_MESSAGE_ID_MESSAGE_REPORTING_OUTOFMEMORY.0,
    GetPrivateDataMoredata = D3D12_MESSAGE_ID_GETPRIVATEDATA_MOREDATA.0,
    SetPrivateDataInvalidfreedata = D3D12_MESSAGE_ID_SETPRIVATEDATA_INVALIDFREEDATA.0,
    SetPrivateDataChangingparams = D3D12_MESSAGE_ID_SETPRIVATEDATA_CHANGINGPARAMS.0,
    SetPrivateDataOutOfMemory = D3D12_MESSAGE_ID_SETPRIVATEDATA_OUTOFMEMORY.0,
    CreateShaderResourceViewUnrecognizedformat =
        D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_UNRECOGNIZEDFORMAT.0,
    CreateShaderResourceViewInvaliddesc = D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDDESC.0,
    CreateShaderResourceViewInvalidformat =
        D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDFORMAT.0,
    CreateShaderResourceViewInvalidvideoplaneslice =
        D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDVIDEOPLANESLICE.0,
    CreateShaderResourceViewInvalidplaneslice =
        D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDPLANESLICE.0,
    CreateShaderResourceViewInvaliddimensions =
        D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDDIMENSIONS.0,
    CreateShaderResourceViewInvalidresource =
        D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDRESOURCE.0,
    CreateRenderTargetViewUnrecognizedformat =
        D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_UNRECOGNIZEDFORMAT.0,
    CreateRenderTargetViewUnsupportedformat =
        D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_UNSUPPORTEDFORMAT.0,
    CreateRenderTargetViewInvaliddesc = D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDDESC.0,
    CreateRenderTargetViewInvalidformat = D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDFORMAT.0,
    CreateRenderTargetViewInvalidvideoplaneslice =
        D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDVIDEOPLANESLICE.0,
    CreateRenderTargetViewInvalidplaneslice =
        D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDPLANESLICE.0,
    CreateRenderTargetViewInvaliddimensions =
        D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDDIMENSIONS.0,
    CreateRenderTargetViewInvalidresource =
        D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDRESOURCE.0,
    CreateDepthStencilViewUnrecognizedformat =
        D3D12_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_UNRECOGNIZEDFORMAT.0,
    CreateDepthStencilViewInvaliddesc = D3D12_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDDESC.0,
    CreateDepthStencilViewInvalidformat = D3D12_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDFORMAT.0,
    CreateDepthStencilViewInvaliddimensions =
        D3D12_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDDIMENSIONS.0,
    CreateDepthStencilViewInvalidresource =
        D3D12_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDRESOURCE.0,
    CreateInputLayoutOutOfMemory = D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_OUTOFMEMORY.0,
    CreateInputLayoutToomanyelements = D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_TOOMANYELEMENTS.0,
    CreateInputLayoutInvalidformat = D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDFORMAT.0,
    CreateInputLayoutIncompatibleformat = D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_INCOMPATIBLEFORMAT.0,
    CreateInputLayoutInvalidslot = D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSLOT.0,
    CreateInputLayoutInvalidinputslotclass =
        D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDINPUTSLOTCLASS.0,
    CreateInputLayoutSteprateslotclassmismatch =
        D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_STEPRATESLOTCLASSMISMATCH.0,
    CreateInputLayoutInvalidslotclasschange =
        D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSLOTCLASSCHANGE.0,
    CreateInputLayoutInvalidstepratechange =
        D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSTEPRATECHANGE.0,
    CreateInputLayoutInvalidalignment = D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDALIGNMENT.0,
    CreateInputLayoutDuplicatesemantic = D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_DUPLICATESEMANTIC.0,
    CreateInputLayoutUnparseableinputsignature =
        D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_UNPARSEABLEINPUTSIGNATURE.0,
    CreateInputLayoutNullsemantic = D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_NULLSEMANTIC.0,
    CreateInputLayoutMissingelement = D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_MISSINGELEMENT.0,
    CreateVertexShaderOutOfMemory = D3D12_MESSAGE_ID_CREATEVERTEXSHADER_OUTOFMEMORY.0,
    CreateVertexShaderInvalidShaderBytecode =
        D3D12_MESSAGE_ID_CREATEVERTEXSHADER_INVALIDSHADERBYTECODE.0,
    CreateVertexShaderInvalidshadertype = D3D12_MESSAGE_ID_CREATEVERTEXSHADER_INVALIDSHADERTYPE.0,
    CreateGeometryShaderOutOfMemory = D3D12_MESSAGE_ID_CREATEGEOMETRYSHADER_OUTOFMEMORY.0,
    CreateGeometryShaderInvalidShaderBytecode =
        D3D12_MESSAGE_ID_CREATEGEOMETRYSHADER_INVALIDSHADERBYTECODE.0,
    CreateGeometryShaderInvalidshadertype =
        D3D12_MESSAGE_ID_CREATEGEOMETRYSHADER_INVALIDSHADERTYPE.0,
    CreateGeometryShaderWithStreamOutputOutOfMemory =
        D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTOFMEMORY.0,
    CreateGeometryShaderWithStreamOutputInvalidShaderBytecode =
        D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSHADERBYTECODE.0,
    CreateGeometryShaderWithStreamOutputInvalidshadertype =
        D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSHADERTYPE.0,
    CreateGeometryShaderWithStreamOutputInvalidnumentries =
        D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDNUMENTRIES.0,
    CreateGeometryShaderWithStreamOutputOutputstreamstrideunused =
        D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTPUTSTREAMSTRIDEUNUSED.0,
    CreateGeometryShaderWithStreamOutputOutputslot0Expected =
        D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTPUTSLOT0EXPECTED.0,
    CreateGeometryShaderWithStreamOutputInvalidoutputslot =
        D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDOUTPUTSLOT.0,
    CreateGeometryShaderWithStreamOutputOnlyoneelementperslot =
        D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_ONLYONEELEMENTPERSLOT.0,
    CreateGeometryShaderWithStreamOutputInvalidcomponentcount =
        D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDCOMPONENTCOUNT.0,
    CreateGeometryShaderWithStreamOutputInvalidstartcomponentandcomponentcount = D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSTARTCOMPONENTANDCOMPONENTCOUNT.0,
    CreateGeometryShaderWithStreamOutputInvalidgapdefinition =
        D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDGAPDEFINITION.0,
    CreateGeometryShaderWithStreamOutputRepeatedOutput =
        D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_REPEATEDOUTPUT.0,
    CreateGeometryShaderWithStreamOutputInvalidOutputStreamStride =
        D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDOUTPUTSTREAMSTRIDE.0,
    CreateGeometryShaderWithStreamOutputMissingSemantic =
        D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MISSINGSEMANTIC.0,
    CreateGeometryShaderWithStreamOutputMaskMismatch =
        D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MASKMISMATCH.0,
    CreateGeometryShaderWithStreamOutputCantHaveOnlyGaps =
        D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_CANTHAVEONLYGAPS.0,
    CreateGeometryShaderWithStreamOutputDeclTooComplex =
        D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_DECLTOOCOMPLEX.0,
    CreateGeometryShaderWithStreamOutputMissingOutputSignature =
        D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MISSINGOUTPUTSIGNATURE.0,
    CreatePixelShaderOutOfMemory = D3D12_MESSAGE_ID_CREATEPIXELSHADER_OUTOFMEMORY.0,
    CreatePixelShaderInvalidShaderBytecode =
        D3D12_MESSAGE_ID_CREATEPIXELSHADER_INVALIDSHADERBYTECODE.0,
    CreatePixelShaderInvalidshadertype = D3D12_MESSAGE_ID_CREATEPIXELSHADER_INVALIDSHADERTYPE.0,
    CreateRasterizerStateInvalidfillmode = D3D12_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDFILLMODE.0,
    CreateRasterizerStateInvalidcullmode = D3D12_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDCULLMODE.0,
    CreateRasterizerStateInvaliddepthbiasclamp =
        D3D12_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDDEPTHBIASCLAMP.0,
    CreateRasterizerStateInvalidslopescaleddepthbias =
        D3D12_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDSLOPESCALEDDEPTHBIAS.0,
    CreatedepthstencilstateInvaliddepthwritemask =
        D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDDEPTHWRITEMASK.0,
    CreatedepthstencilstateInvaliddepthfunc =
        D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDDEPTHFUNC.0,
    CreatedepthstencilstateInvalidfrontfacestencilfailop =
        D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILFAILOP.0,
    CreatedepthstencilstateInvalidfrontfacestencilzfailop =
        D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILZFAILOP.0,
    CreatedepthstencilstateInvalidfrontfacestencilpassop =
        D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILPASSOP.0,
    CreatedepthstencilstateInvalidfrontfacestencilfunc =
        D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILFUNC.0,
    CreatedepthstencilstateInvalidbackfacestencilfailop =
        D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILFAILOP.0,
    CreatedepthstencilstateInvalidbackfacestencilzfailop =
        D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILZFAILOP.0,
    CreatedepthstencilstateInvalidbackfacestencilpassop =
        D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILPASSOP.0,
    CreatedepthstencilstateInvalidbackfacestencilfunc =
        D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILFUNC.0,
    CreateblendstateInvalidsrcblend = D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDSRCBLEND.0,
    CreateblendstateInvaliddestblend = D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDDESTBLEND.0,
    CreateblendstateInvalidblendop = D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDBLENDOP.0,
    CreateblendstateInvalidsrcblendalpha = D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDSRCBLENDALPHA.0,
    CreateblendstateInvaliddestblendalpha =
        D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDDESTBLENDALPHA.0,
    CreateblendstateInvalidblendopalpha = D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDBLENDOPALPHA.0,
    CreateblendstateInvalidrendertargetwritemask =
        D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDRENDERTARGETWRITEMASK.0,
    CleardepthstencilviewInvalid = D3D12_MESSAGE_ID_CLEARDEPTHSTENCILVIEW_INVALID.0,
    CommandListDrawRootSignatureNotSet =
        D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_ROOT_SIGNATURE_NOT_SET.0,
    CommandListDrawRootSignatureMismatch =
        D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_ROOT_SIGNATURE_MISMATCH.0,
    CommandListDrawVertexBufferNotSet = D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_VERTEX_BUFFER_NOT_SET.0,
    CommandListDrawVertexBufferStrideTooSmall =
        D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_VERTEX_BUFFER_STRIDE_TOO_SMALL.0,
    CommandListDrawVertexBufferTooSmall =
        D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_VERTEX_BUFFER_TOO_SMALL.0,
    CommandListDrawIndexBufferNotSet = D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_INDEX_BUFFER_NOT_SET.0,
    CommandListDrawIndexBufferFormatInvalid =
        D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_INDEX_BUFFER_FORMAT_INVALID.0,
    CommandListDrawIndexBufferTooSmall =
        D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_INDEX_BUFFER_TOO_SMALL.0,
    CommandListDrawInvalidPrimitivetopology =
        D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_INVALID_PRIMITIVETOPOLOGY.0,
    CommandListDrawVertexStrideUnaligned =
        D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_VERTEX_STRIDE_UNALIGNED.0,
    CommandListDrawIndexOffsetUnaligned =
        D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_INDEX_OFFSET_UNALIGNED.0,
    DeviceRemovalProcessAtFault = D3D12_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_AT_FAULT.0,
    DeviceRemovalProcessPossiblyAtFault =
        D3D12_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_POSSIBLY_AT_FAULT.0,
    DeviceRemovalProcessNotAtFault = D3D12_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_NOT_AT_FAULT.0,
    CreateInputLayoutTrailingDigitInSemantic =
        D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_TRAILING_DIGIT_IN_SEMANTIC.0,
    CreateGeometryShaderWithStreamOutputTrailingDigitInSemantic =
        D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_TRAILING_DIGIT_IN_SEMANTIC.0,
    CreateInputLayoutTypeMismatch = D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_TYPE_MISMATCH.0,
    CreateInputLayoutEmptyLayout = D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_EMPTY_LAYOUT.0,
    LiveObjectSummary = D3D12_MESSAGE_ID_LIVE_OBJECT_SUMMARY.0,
    LiveDevice = D3D12_MESSAGE_ID_LIVE_DEVICE.0,
    LiveSwapchain = D3D12_MESSAGE_ID_LIVE_SWAPCHAIN.0,
    CreateDepthStencilViewInvalidflags = D3D12_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDFLAGS.0,
    CreateVertexShaderInvalidclasslinkage =
        D3D12_MESSAGE_ID_CREATEVERTEXSHADER_INVALIDCLASSLINKAGE.0,
    CreateGeometryShaderInvalidclasslinkage =
        D3D12_MESSAGE_ID_CREATEGEOMETRYSHADER_INVALIDCLASSLINKAGE.0,
    CreateGeometryShaderWithStreamOutputInvalidstreamtorasterizer =
        D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSTREAMTORASTERIZER.0,
    CreatePixelShaderInvalidclasslinkage = D3D12_MESSAGE_ID_CREATEPIXELSHADER_INVALIDCLASSLINKAGE.0,
    CreateGeometryShaderWithStreamOutputInvalidstream =
        D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSTREAM.0,
    CreateGeometryShaderWithStreamOutputUnexpectedentries =
        D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_UNEXPECTEDENTRIES.0,
    CreateGeometryShaderWithStreamOutputUnexpectedstrides =
        D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_UNEXPECTEDSTRIDES.0,
    CreateGeometryShaderWithStreamOutputInvalidnumstrides =
        D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDNUMSTRIDES.0,
    CreatehullshaderOutOfMemory = D3D12_MESSAGE_ID_CREATEHULLSHADER_OUTOFMEMORY.0,
    CreatehullshaderInvalidShaderBytecode =
        D3D12_MESSAGE_ID_CREATEHULLSHADER_INVALIDSHADERBYTECODE.0,
    CreatehullshaderInvalidshadertype = D3D12_MESSAGE_ID_CREATEHULLSHADER_INVALIDSHADERTYPE.0,
    CreatehullshaderInvalidclasslinkage = D3D12_MESSAGE_ID_CREATEHULLSHADER_INVALIDCLASSLINKAGE.0,
    CreatedomainshaderOutOfMemory = D3D12_MESSAGE_ID_CREATEDOMAINSHADER_OUTOFMEMORY.0,
    CreatedomainshaderInvalidShaderBytecode =
        D3D12_MESSAGE_ID_CREATEDOMAINSHADER_INVALIDSHADERBYTECODE.0,
    CreatedomainshaderInvalidshadertype = D3D12_MESSAGE_ID_CREATEDOMAINSHADER_INVALIDSHADERTYPE.0,
    CreatedomainshaderInvalidclasslinkage =
        D3D12_MESSAGE_ID_CREATEDOMAINSHADER_INVALIDCLASSLINKAGE.0,
    ResourceUnmapNotmapped = D3D12_MESSAGE_ID_RESOURCE_UNMAP_NOTMAPPED.0,
    DeviceCheckfeaturesupportMismatchedDataSize =
        D3D12_MESSAGE_ID_DEVICE_CHECKFEATURESUPPORT_MISMATCHED_DATA_SIZE.0,
    CreateComputeShaderOutOfMemory = D3D12_MESSAGE_ID_CREATECOMPUTESHADER_OUTOFMEMORY.0,
    CreateComputeShaderInvalidShaderBytecode =
        D3D12_MESSAGE_ID_CREATECOMPUTESHADER_INVALIDSHADERBYTECODE.0,
    CreateComputeShaderInvalidclasslinkage =
        D3D12_MESSAGE_ID_CREATECOMPUTESHADER_INVALIDCLASSLINKAGE.0,
    DeviceCreateVertexShaderDoublefloatopsnotsupported =
        D3D12_MESSAGE_ID_DEVICE_CREATEVERTEXSHADER_DOUBLEFLOATOPSNOTSUPPORTED.0,
    DeviceCreatehullshaderDoublefloatopsnotsupported =
        D3D12_MESSAGE_ID_DEVICE_CREATEHULLSHADER_DOUBLEFLOATOPSNOTSUPPORTED.0,
    DeviceCreatedomainshaderDoublefloatopsnotsupported =
        D3D12_MESSAGE_ID_DEVICE_CREATEDOMAINSHADER_DOUBLEFLOATOPSNOTSUPPORTED.0,
    DeviceCreateGeometryShaderDoublefloatopsnotsupported =
        D3D12_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADER_DOUBLEFLOATOPSNOTSUPPORTED.0,
    DeviceCreateGeometryShaderWithStreamOutputDoublefloatopsnotsupported =
        D3D12_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_DOUBLEFLOATOPSNOTSUPPORTED.0,
    DeviceCreatePixelShaderDoublefloatopsnotsupported =
        D3D12_MESSAGE_ID_DEVICE_CREATEPIXELSHADER_DOUBLEFLOATOPSNOTSUPPORTED.0,
    DeviceCreateComputeShaderDoublefloatopsnotsupported =
        D3D12_MESSAGE_ID_DEVICE_CREATECOMPUTESHADER_DOUBLEFLOATOPSNOTSUPPORTED.0,
    CreateunorderedaccessviewInvalidresource =
        D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDRESOURCE.0,
    CreateunorderedaccessviewInvaliddesc = D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDDESC.0,
    CreateunorderedaccessviewInvalidformat =
        D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDFORMAT.0,
    CreateunorderedaccessviewInvalidvideoplaneslice =
        D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDVIDEOPLANESLICE.0,
    CreateunorderedaccessviewInvalidplaneslice =
        D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDPLANESLICE.0,
    CreateunorderedaccessviewInvaliddimensions =
        D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDDIMENSIONS.0,
    CreateunorderedaccessviewUnrecognizedformat =
        D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_UNRECOGNIZEDFORMAT.0,
    CreateunorderedaccessviewInvalidflags =
        D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDFLAGS.0,
    CreateRasterizerStateInvalidforcedsamplecount =
        D3D12_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDFORCEDSAMPLECOUNT.0,
    CreateblendstateInvalidlogicops = D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDLOGICOPS.0,
    DeviceCreateVertexShaderDoubleextensionsnotsupported =
        D3D12_MESSAGE_ID_DEVICE_CREATEVERTEXSHADER_DOUBLEEXTENSIONSNOTSUPPORTED.0,
    DeviceCreatehullshaderDoubleextensionsnotsupported =
        D3D12_MESSAGE_ID_DEVICE_CREATEHULLSHADER_DOUBLEEXTENSIONSNOTSUPPORTED.0,
    DeviceCreatedomainshaderDoubleextensionsnotsupported =
        D3D12_MESSAGE_ID_DEVICE_CREATEDOMAINSHADER_DOUBLEEXTENSIONSNOTSUPPORTED.0,
    DeviceCreateGeometryShaderDoubleextensionsnotsupported =
        D3D12_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADER_DOUBLEEXTENSIONSNOTSUPPORTED.0,
    DeviceCreateGeometryShaderWithStreamOutputDoubleextensionsnotsupported =
        D3D12_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_DOUBLEEXTENSIONSNOTSUPPORTED.0,
    DeviceCreatePixelShaderDoubleextensionsnotsupported =
        D3D12_MESSAGE_ID_DEVICE_CREATEPIXELSHADER_DOUBLEEXTENSIONSNOTSUPPORTED.0,
    DeviceCreateComputeShaderDoubleextensionsnotsupported =
        D3D12_MESSAGE_ID_DEVICE_CREATECOMPUTESHADER_DOUBLEEXTENSIONSNOTSUPPORTED.0,
    DeviceCreateVertexShaderUavsnotsupported =
        D3D12_MESSAGE_ID_DEVICE_CREATEVERTEXSHADER_UAVSNOTSUPPORTED.0,
    DeviceCreatehullshaderUavsnotsupported =
        D3D12_MESSAGE_ID_DEVICE_CREATEHULLSHADER_UAVSNOTSUPPORTED.0,
    DeviceCreatedomainshaderUavsnotsupported =
        D3D12_MESSAGE_ID_DEVICE_CREATEDOMAINSHADER_UAVSNOTSUPPORTED.0,
    DeviceCreateGeometryShaderUavsnotsupported =
        D3D12_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADER_UAVSNOTSUPPORTED.0,
    DeviceCreateGeometryShaderWithStreamOutputUavsnotsupported =
        D3D12_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_UAVSNOTSUPPORTED.0,
    DeviceCreatePixelShaderUavsnotsupported =
        D3D12_MESSAGE_ID_DEVICE_CREATEPIXELSHADER_UAVSNOTSUPPORTED.0,
    DeviceCreateComputeShaderUavsnotsupported =
        D3D12_MESSAGE_ID_DEVICE_CREATECOMPUTESHADER_UAVSNOTSUPPORTED.0,
    DeviceClearviewInvalidsourcerect = D3D12_MESSAGE_ID_DEVICE_CLEARVIEW_INVALIDSOURCERECT.0,
    DeviceClearviewEmptyrect = D3D12_MESSAGE_ID_DEVICE_CLEARVIEW_EMPTYRECT.0,
    UpdatetilemappingsInvalidParameter = D3D12_MESSAGE_ID_UPDATETILEMAPPINGS_INVALID_PARAMETER.0,
    CopytilemappingsInvalidParameter = D3D12_MESSAGE_ID_COPYTILEMAPPINGS_INVALID_PARAMETER.0,
    CreatedeviceInvalidargs = D3D12_MESSAGE_ID_CREATEDEVICE_INVALIDARGS.0,
    CreatedeviceWarning = D3D12_MESSAGE_ID_CREATEDEVICE_WARNING.0,
    ResourceBarrierInvalidType = D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_TYPE.0,
    ResourceBarrierNullPointer = D3D12_MESSAGE_ID_RESOURCE_BARRIER_NULL_POINTER.0,
    ResourceBarrierInvalidSubresource = D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_SUBRESOURCE.0,
    ResourceBarrierReservedBits = D3D12_MESSAGE_ID_RESOURCE_BARRIER_RESERVED_BITS.0,
    ResourceBarrierMissingBindFlags = D3D12_MESSAGE_ID_RESOURCE_BARRIER_MISSING_BIND_FLAGS.0,
    ResourceBarrierMismatchingMiscFlags =
        D3D12_MESSAGE_ID_RESOURCE_BARRIER_MISMATCHING_MISC_FLAGS.0,
    ResourceBarrierMatchingStates = D3D12_MESSAGE_ID_RESOURCE_BARRIER_MATCHING_STATES.0,
    ResourceBarrierInvalidCombination = D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_COMBINATION.0,
    ResourceBarrierBeforeAfterMismatch = D3D12_MESSAGE_ID_RESOURCE_BARRIER_BEFORE_AFTER_MISMATCH.0,
    ResourceBarrierInvalidResource = D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_RESOURCE.0,
    ResourceBarrierSampleCount = D3D12_MESSAGE_ID_RESOURCE_BARRIER_SAMPLE_COUNT.0,
    ResourceBarrierInvalidFlags = D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_FLAGS.0,
    ResourceBarrierInvalidCombinedFlags =
        D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_COMBINED_FLAGS.0,
    ResourceBarrierInvalidFlagsForFormat =
        D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_FLAGS_FOR_FORMAT.0,
    ResourceBarrierInvalidSplitBarrier = D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_SPLIT_BARRIER.0,
    ResourceBarrierUnmatchedEnd = D3D12_MESSAGE_ID_RESOURCE_BARRIER_UNMATCHED_END.0,
    ResourceBarrierUnmatchedBegin = D3D12_MESSAGE_ID_RESOURCE_BARRIER_UNMATCHED_BEGIN.0,
    ResourceBarrierInvalidFlag = D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_FLAG.0,
    ResourceBarrierInvalidCommandListType =
        D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_COMMAND_LIST_TYPE.0,
    InvalidSubresourceState = D3D12_MESSAGE_ID_INVALID_SUBRESOURCE_STATE.0,
    CommandAllocatorContention = D3D12_MESSAGE_ID_COMMAND_ALLOCATOR_CONTENTION.0,
    CommandAllocatorReset = D3D12_MESSAGE_ID_COMMAND_ALLOCATOR_RESET.0,
    CommandAllocatorResetBundle = D3D12_MESSAGE_ID_COMMAND_ALLOCATOR_RESET_BUNDLE.0,
    CommandAllocatorCannotReset = D3D12_MESSAGE_ID_COMMAND_ALLOCATOR_CANNOT_RESET.0,
    CommandListOpen = D3D12_MESSAGE_ID_COMMAND_LIST_OPEN.0,
    InvalidBundleApi = D3D12_MESSAGE_ID_INVALID_BUNDLE_API.0,
    CommandListClosed = D3D12_MESSAGE_ID_COMMAND_LIST_CLOSED.0,
    WrongCommandAllocatorType = D3D12_MESSAGE_ID_WRONG_COMMAND_ALLOCATOR_TYPE.0,
    CommandAllocatorSync = D3D12_MESSAGE_ID_COMMAND_ALLOCATOR_SYNC.0,
    CommandListSync = D3D12_MESSAGE_ID_COMMAND_LIST_SYNC.0,
    SetDescriptorHeapInvalid = D3D12_MESSAGE_ID_SET_DESCRIPTOR_HEAP_INVALID.0,
    CreateCommandqueue = D3D12_MESSAGE_ID_CREATE_COMMANDQUEUE.0,
    CreateCommandallocator = D3D12_MESSAGE_ID_CREATE_COMMANDALLOCATOR.0,
    CreatePipelinestate = D3D12_MESSAGE_ID_CREATE_PIPELINESTATE.0,
    CreateCommandlist12 = D3D12_MESSAGE_ID_CREATE_COMMANDLIST12.0,
    CreateResource = D3D12_MESSAGE_ID_CREATE_RESOURCE.0,
    CreateDescriptorheap = D3D12_MESSAGE_ID_CREATE_DESCRIPTORHEAP.0,
    CreateRootsignature = D3D12_MESSAGE_ID_CREATE_ROOTSIGNATURE.0,
    CreateLibrary = D3D12_MESSAGE_ID_CREATE_LIBRARY.0,
    CreateHeap = D3D12_MESSAGE_ID_CREATE_HEAP.0,
    CreateMonitoredfence = D3D12_MESSAGE_ID_CREATE_MONITOREDFENCE.0,
    CreateQueryheap = D3D12_MESSAGE_ID_CREATE_QUERYHEAP.0,
    CreateCommandsignature = D3D12_MESSAGE_ID_CREATE_COMMANDSIGNATURE.0,
    LiveCommandqueue = D3D12_MESSAGE_ID_LIVE_COMMANDQUEUE.0,
    LiveCommandallocator = D3D12_MESSAGE_ID_LIVE_COMMANDALLOCATOR.0,
    LivePipelinestate = D3D12_MESSAGE_ID_LIVE_PIPELINESTATE.0,
    LiveCommandlist12 = D3D12_MESSAGE_ID_LIVE_COMMANDLIST12.0,
    LiveResource = D3D12_MESSAGE_ID_LIVE_RESOURCE.0,
    LiveDescriptorheap = D3D12_MESSAGE_ID_LIVE_DESCRIPTORHEAP.0,
    LiveRootsignature = D3D12_MESSAGE_ID_LIVE_ROOTSIGNATURE.0,
    LiveLibrary = D3D12_MESSAGE_ID_LIVE_LIBRARY.0,
    LiveHeap = D3D12_MESSAGE_ID_LIVE_HEAP.0,
    LiveMonitoredfence = D3D12_MESSAGE_ID_LIVE_MONITOREDFENCE.0,
    LiveQueryheap = D3D12_MESSAGE_ID_LIVE_QUERYHEAP.0,
    LiveCommandsignature = D3D12_MESSAGE_ID_LIVE_COMMANDSIGNATURE.0,
    DestroyCommandqueue = D3D12_MESSAGE_ID_DESTROY_COMMANDQUEUE.0,
    DestroyCommandallocator = D3D12_MESSAGE_ID_DESTROY_COMMANDALLOCATOR.0,
    DestroyPipelinestate = D3D12_MESSAGE_ID_DESTROY_PIPELINESTATE.0,
    DestroyCommandlist12 = D3D12_MESSAGE_ID_DESTROY_COMMANDLIST12.0,
    DestroyResource = D3D12_MESSAGE_ID_DESTROY_RESOURCE.0,
    DestroyDescriptorheap = D3D12_MESSAGE_ID_DESTROY_DESCRIPTORHEAP.0,
    DestroyRootsignature = D3D12_MESSAGE_ID_DESTROY_ROOTSIGNATURE.0,
    DestroyLibrary = D3D12_MESSAGE_ID_DESTROY_LIBRARY.0,
    DestroyHeap = D3D12_MESSAGE_ID_DESTROY_HEAP.0,
    DestroyMonitoredfence = D3D12_MESSAGE_ID_DESTROY_MONITOREDFENCE.0,
    DestroyQueryheap = D3D12_MESSAGE_ID_DESTROY_QUERYHEAP.0,
    DestroyCommandsignature = D3D12_MESSAGE_ID_DESTROY_COMMANDSIGNATURE.0,
    CreateResourceInvalidDimensions = D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDDIMENSIONS.0,
    CreateResourceInvalidMiscFlags = D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDMISCFLAGS.0,
    CreateResourceInvalidArgReturn = D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDARG_RETURN.0,
    CreateResourceOutOfMemoryReturn = D3D12_MESSAGE_ID_CREATERESOURCE_OUTOFMEMORY_RETURN.0,
    CreateResourceInvalidDesc = D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDDESC.0,
    PossiblyInvalidSubresourceState = D3D12_MESSAGE_ID_POSSIBLY_INVALID_SUBRESOURCE_STATE.0,
    InvalidUseOfNonResidentResource = D3D12_MESSAGE_ID_INVALID_USE_OF_NON_RESIDENT_RESOURCE.0,
    PossibleInvalidUseOfNonResidentResource =
        D3D12_MESSAGE_ID_POSSIBLE_INVALID_USE_OF_NON_RESIDENT_RESOURCE.0,
    BundlePipelineStateMismatch = D3D12_MESSAGE_ID_BUNDLE_PIPELINE_STATE_MISMATCH.0,
    PrimitiveTopologyMismatchPipelineState =
        D3D12_MESSAGE_ID_PRIMITIVE_TOPOLOGY_MISMATCH_PIPELINE_STATE.0,
    RenderTargetFormatMismatchPipelineState =
        D3D12_MESSAGE_ID_RENDER_TARGET_FORMAT_MISMATCH_PIPELINE_STATE.0,
    RenderTargetSampleDescMismatchPipelineState =
        D3D12_MESSAGE_ID_RENDER_TARGET_SAMPLE_DESC_MISMATCH_PIPELINE_STATE.0,
    DepthStencilFormatMismatchPipelineState =
        D3D12_MESSAGE_ID_DEPTH_STENCIL_FORMAT_MISMATCH_PIPELINE_STATE.0,
    DepthStencilSampleDescMismatchPipelineState =
        D3D12_MESSAGE_ID_DEPTH_STENCIL_SAMPLE_DESC_MISMATCH_PIPELINE_STATE.0,
    CreateshaderInvalidbytecode = D3D12_MESSAGE_ID_CREATESHADER_INVALIDBYTECODE.0,
    CreateHeapNulldesc = D3D12_MESSAGE_ID_CREATEHEAP_NULLDESC.0,
    CreateHeapInvalidsize = D3D12_MESSAGE_ID_CREATEHEAP_INVALIDSIZE.0,
    CreateHeapUnrecognizedheaptype = D3D12_MESSAGE_ID_CREATEHEAP_UNRECOGNIZEDHEAPTYPE.0,
    CreateHeapUnrecognizedcpupageproperties =
        D3D12_MESSAGE_ID_CREATEHEAP_UNRECOGNIZEDCPUPAGEPROPERTIES.0,
    CreateHeapUnrecognizedmemorypool = D3D12_MESSAGE_ID_CREATEHEAP_UNRECOGNIZEDMEMORYPOOL.0,
    CreateHeapInvalidproperties = D3D12_MESSAGE_ID_CREATEHEAP_INVALIDPROPERTIES.0,
    CreateHeapInvalidalignment = D3D12_MESSAGE_ID_CREATEHEAP_INVALIDALIGNMENT.0,
    CreateHeapUnrecognizedmiscflags = D3D12_MESSAGE_ID_CREATEHEAP_UNRECOGNIZEDMISCFLAGS.0,
    CreateHeapInvalidmiscflags = D3D12_MESSAGE_ID_CREATEHEAP_INVALIDMISCFLAGS.0,
    CreateHeapInvalidargReturn = D3D12_MESSAGE_ID_CREATEHEAP_INVALIDARG_RETURN.0,
    CreateHeapOutOfMemoryReturn = D3D12_MESSAGE_ID_CREATEHEAP_OUTOFMEMORY_RETURN.0,
    CreateResourceAndHeapNullheapproperties =
        D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_NULLHEAPPROPERTIES.0,
    CreateResourceAndHeapUnrecognizedheaptype =
        D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_UNRECOGNIZEDHEAPTYPE.0,
    CreateResourceAndHeapUnrecognizedcpupageproperties =
        D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_UNRECOGNIZEDCPUPAGEPROPERTIES.0,
    CreateResourceAndHeapUnrecognizedmemorypool =
        D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_UNRECOGNIZEDMEMORYPOOL.0,
    CreateResourceAndHeapInvalidheapproperties =
        D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_INVALIDHEAPPROPERTIES.0,
    CreateResourceAndHeapUnrecognizedheapmiscflags =
        D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_UNRECOGNIZEDHEAPMISCFLAGS.0,
    CreateResourceAndHeapInvalidheapmiscflags =
        D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_INVALIDHEAPMISCFLAGS.0,
    CreateResourceAndHeapInvalidargReturn =
        D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_INVALIDARG_RETURN.0,
    CreateResourceAndHeapOutOfMemoryReturn =
        D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_OUTOFMEMORY_RETURN.0,
    GetCustomHeapPropertiesUnrecognizedheaptype =
        D3D12_MESSAGE_ID_GETCUSTOMHEAPPROPERTIES_UNRECOGNIZEDHEAPTYPE.0,
    GetCustomHeapPropertiesInvalidheaptype =
        D3D12_MESSAGE_ID_GETCUSTOMHEAPPROPERTIES_INVALIDHEAPTYPE.0,
    CreateDescriptorHeapInvalidDesc = D3D12_MESSAGE_ID_CREATE_DESCRIPTOR_HEAP_INVALID_DESC.0,
    InvalidDescriptorHandle = D3D12_MESSAGE_ID_INVALID_DESCRIPTOR_HANDLE.0,
    CreateRasterizerStateInvalidConservativerastermode =
        D3D12_MESSAGE_ID_CREATERASTERIZERSTATE_INVALID_CONSERVATIVERASTERMODE.0,
    CreateConstantBufferViewInvalidResource =
        D3D12_MESSAGE_ID_CREATE_CONSTANT_BUFFER_VIEW_INVALID_RESOURCE.0,
    CreateConstantBufferViewInvalidDesc =
        D3D12_MESSAGE_ID_CREATE_CONSTANT_BUFFER_VIEW_INVALID_DESC.0,
    CreateUnorderedaccessViewInvalidCounterUsage =
        D3D12_MESSAGE_ID_CREATE_UNORDEREDACCESS_VIEW_INVALID_COUNTER_USAGE.0,
    CopyDescriptorsInvalidRanges = D3D12_MESSAGE_ID_COPY_DESCRIPTORS_INVALID_RANGES.0,
    CopyDescriptorsWriteOnlyDescriptor = D3D12_MESSAGE_ID_COPY_DESCRIPTORS_WRITE_ONLY_DESCRIPTOR.0,
    CreateGraphicsPipelineStateRtvFormatNotUnknown =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_RTV_FORMAT_NOT_UNKNOWN.0,
    CreateGraphicsPipelineStateInvalidRenderTargetCount =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INVALID_RENDER_TARGET_COUNT.0,
    CreateGraphicsPipelineStateVertexShaderNotSet =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_VERTEX_SHADER_NOT_SET.0,
    CreateGraphicsPipelineStateInputlayoutNotSet =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INPUTLAYOUT_NOT_SET.0,
    CreateGraphicsPipelineStateShaderLinkageHsDsSignatureMismatch =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_HS_DS_SIGNATURE_MISMATCH.0,
    CreateGraphicsPipelineStateShaderLinkageRegisterindex =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_REGISTERINDEX.0,
    CreateGraphicsPipelineStateShaderLinkageComponenttype =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_COMPONENTTYPE.0,
    CreateGraphicsPipelineStateShaderLinkageRegistermask =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_REGISTERMASK.0,
    CreateGraphicsPipelineStateShaderLinkageSystemvalue =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_SYSTEMVALUE.0,
    CreateGraphicsPipelineStateShaderLinkageNeverwrittenAlwaysreads =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_NEVERWRITTEN_ALWAYSREADS.0,
    CreateGraphicsPipelineStateShaderLinkageMinprecision =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_MINPRECISION.0,
    CreateGraphicsPipelineStateShaderLinkageSemanticnameNotFound =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_SEMANTICNAME_NOT_FOUND.0,
    CreateGraphicsPipelineStateHsXorDsMismatch =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_HS_XOR_DS_MISMATCH.0,
    CreateGraphicsPipelineStateHullShaderInputTopologyMismatch =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_HULL_SHADER_INPUT_TOPOLOGY_MISMATCH.0,
    CreateGraphicsPipelineStateHsDsControlPointCountMismatch =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_HS_DS_CONTROL_POINT_COUNT_MISMATCH.0,
    CreateGraphicsPipelineStateHsDsTessellatorDomainMismatch =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_HS_DS_TESSELLATOR_DOMAIN_MISMATCH.0,
    CreateGraphicsPipelineStateInvalidUseOfCenterMultisamplePattern =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INVALID_USE_OF_CENTER_MULTISAMPLE_PATTERN.0,
    CreateGraphicsPipelineStateInvalidUseOfForcedSampleCount =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INVALID_USE_OF_FORCED_SAMPLE_COUNT.0,
    CreateGraphicsPipelineStateInvalidPrimitivetopology =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INVALID_PRIMITIVETOPOLOGY.0,
    CreateGraphicsPipelineStateInvalidSystemvalue =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INVALID_SYSTEMVALUE.0,
    CreateGraphicsPipelineStateOmDualSourceBlendingCanOnlyHaveRenderTarget0 = D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_OM_DUAL_SOURCE_BLENDING_CAN_ONLY_HAVE_RENDER_TARGET_0.0,
    CreateGraphicsPipelineStateOmRenderTargetDoesNotSupportBlending =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_OM_RENDER_TARGET_DOES_NOT_SUPPORT_BLENDING.0,
    CreateGraphicsPipelineStatePsOutputTypeMismatch =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_PS_OUTPUT_TYPE_MISMATCH.0,
    CreateGraphicsPipelineStateOmRenderTargetDoesNotSupportLogicOps =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_OM_RENDER_TARGET_DOES_NOT_SUPPORT_LOGIC_OPS.0,
    CreateGraphicsPipelineStateRendertargetviewNotSet =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_RENDERTARGETVIEW_NOT_SET.0,
    CreateGraphicsPipelineStateDepthstencilviewNotSet =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_DEPTHSTENCILVIEW_NOT_SET.0,
    CreateGraphicsPipelineStateGsInputPrimitiveMismatch =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_GS_INPUT_PRIMITIVE_MISMATCH.0,
    CreateGraphicsPipelineStatePositionNotPresent =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_POSITION_NOT_PRESENT.0,
    CreateGraphicsPipelineStateMissingRootSignatureFlags =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_MISSING_ROOT_SIGNATURE_FLAGS.0,
    CreateGraphicsPipelineStateInvalidIndexBufferProperties =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INVALID_INDEX_BUFFER_PROPERTIES.0,
    CreateGraphicsPipelineStateInvalidSampleDesc =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INVALID_SAMPLE_DESC.0,
    CreateGraphicsPipelineStateHsRootSignatureMismatch =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_HS_ROOT_SIGNATURE_MISMATCH.0,
    CreateGraphicsPipelineStateDsRootSignatureMismatch =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_DS_ROOT_SIGNATURE_MISMATCH.0,
    CreateGraphicsPipelineStateVsRootSignatureMismatch =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_VS_ROOT_SIGNATURE_MISMATCH.0,
    CreateGraphicsPipelineStateGsRootSignatureMismatch =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_GS_ROOT_SIGNATURE_MISMATCH.0,
    CreateGraphicsPipelineStatePsRootSignatureMismatch =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_PS_ROOT_SIGNATURE_MISMATCH.0,
    CreateGraphicsPipelineStateMissingRootSignature =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_MISSING_ROOT_SIGNATURE.0,
    ExecuteBundleOpenBundle = D3D12_MESSAGE_ID_EXECUTE_BUNDLE_OPEN_BUNDLE.0,
    ExecuteBundleDescriptorHeapMismatch =
        D3D12_MESSAGE_ID_EXECUTE_BUNDLE_DESCRIPTOR_HEAP_MISMATCH.0,
    ExecuteBundleType = D3D12_MESSAGE_ID_EXECUTE_BUNDLE_TYPE.0,
    DrawEmptyScissorRectangle = D3D12_MESSAGE_ID_DRAW_EMPTY_SCISSOR_RECTANGLE.0,
    CreateRootSignatureBlobNotFound = D3D12_MESSAGE_ID_CREATE_ROOT_SIGNATURE_BLOB_NOT_FOUND.0,
    CreateRootSignatureDeserializeFailed =
        D3D12_MESSAGE_ID_CREATE_ROOT_SIGNATURE_DESERIALIZE_FAILED.0,
    CreateRootSignatureInvalidConfiguration =
        D3D12_MESSAGE_ID_CREATE_ROOT_SIGNATURE_INVALID_CONFIGURATION.0,
    CreateRootSignatureNotSupportedOnDevice =
        D3D12_MESSAGE_ID_CREATE_ROOT_SIGNATURE_NOT_SUPPORTED_ON_DEVICE.0,
    CreateResourceAndHeapNullresourceproperties =
        D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_NULLRESOURCEPROPERTIES.0,
    CreateResourceAndHeapNullheap = D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_NULLHEAP.0,
    GetresourceallocationinfoInvalidrdescs =
        D3D12_MESSAGE_ID_GETRESOURCEALLOCATIONINFO_INVALIDRDESCS.0,
    MakeresidentNullobjectarray = D3D12_MESSAGE_ID_MAKERESIDENT_NULLOBJECTARRAY.0,
    EvictNullobjectarray = D3D12_MESSAGE_ID_EVICT_NULLOBJECTARRAY.0,
    SetDescriptorTableInvalid = D3D12_MESSAGE_ID_SET_DESCRIPTOR_TABLE_INVALID.0,
    SetRootConstantInvalid = D3D12_MESSAGE_ID_SET_ROOT_CONSTANT_INVALID.0,
    SetRootConstantBufferViewInvalid = D3D12_MESSAGE_ID_SET_ROOT_CONSTANT_BUFFER_VIEW_INVALID.0,
    SetRootShaderResourceViewInvalid = D3D12_MESSAGE_ID_SET_ROOT_SHADER_RESOURCE_VIEW_INVALID.0,
    SetRootUnorderedAccessViewInvalid = D3D12_MESSAGE_ID_SET_ROOT_UNORDERED_ACCESS_VIEW_INVALID.0,
    SetVertexBuffersInvalidDesc = D3D12_MESSAGE_ID_SET_VERTEX_BUFFERS_INVALID_DESC.0,
    SetIndexBufferInvalidDesc = D3D12_MESSAGE_ID_SET_INDEX_BUFFER_INVALID_DESC.0,
    SetStreamOutputBuffersInvalidDesc = D3D12_MESSAGE_ID_SET_STREAM_OUTPUT_BUFFERS_INVALID_DESC.0,
    CreateResourceUnrecognizeddimensionality =
        D3D12_MESSAGE_ID_CREATERESOURCE_UNRECOGNIZEDDIMENSIONALITY.0,
    CreateResourceUnrecognizedlayout = D3D12_MESSAGE_ID_CREATERESOURCE_UNRECOGNIZEDLAYOUT.0,
    CreateResourceInvaliddimensionality = D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDDIMENSIONALITY.0,
    CreateResourceInvalidalignment = D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDALIGNMENT.0,
    CreateResourceInvalidmiplevels = D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDMIPLEVELS.0,
    CreateResourceInvalidsampledesc = D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDSAMPLEDESC.0,
    CreateResourceInvalidlayout = D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDLAYOUT.0,
    SetIndexBufferInvalid = D3D12_MESSAGE_ID_SET_INDEX_BUFFER_INVALID.0,
    SetVertexBuffersInvalid = D3D12_MESSAGE_ID_SET_VERTEX_BUFFERS_INVALID.0,
    SetStreamOutputBuffersInvalid = D3D12_MESSAGE_ID_SET_STREAM_OUTPUT_BUFFERS_INVALID.0,
    SetRenderTargetsInvalid = D3D12_MESSAGE_ID_SET_RENDER_TARGETS_INVALID.0,
    CreatequeryHeapInvalidParameters = D3D12_MESSAGE_ID_CREATEQUERY_HEAP_INVALID_PARAMETERS.0,
    BeginEndQueryInvalidParameters = D3D12_MESSAGE_ID_BEGIN_END_QUERY_INVALID_PARAMETERS.0,
    CloseCommandListOpenQuery = D3D12_MESSAGE_ID_CLOSE_COMMAND_LIST_OPEN_QUERY.0,
    ResolveQueryDataInvalidParameters = D3D12_MESSAGE_ID_RESOLVE_QUERY_DATA_INVALID_PARAMETERS.0,
    SetPredicationInvalidParameters = D3D12_MESSAGE_ID_SET_PREDICATION_INVALID_PARAMETERS.0,
    TimestampsNotSupported = D3D12_MESSAGE_ID_TIMESTAMPS_NOT_SUPPORTED.0,
    CreateResourceUnrecognizedformat = D3D12_MESSAGE_ID_CREATERESOURCE_UNRECOGNIZEDFORMAT.0,
    CreateResourceInvalidformat = D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDFORMAT.0,
    GetCopyableFootprintsOrCopyableLayoutInvalidSubresourcerange =
        D3D12_MESSAGE_ID_GETCOPYABLEFOOTPRINTS_INVALIDSUBRESOURCERANGE.0,
    GetCopyableFootprintsOrCopyableLayoutInvalidbaseoffset =
        D3D12_MESSAGE_ID_GETCOPYABLEFOOTPRINTS_INVALIDBASEOFFSET.0,
    ResourceBarrierInvalidHeap = D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_HEAP.0,
    CreateSamplerInvalid = D3D12_MESSAGE_ID_CREATE_SAMPLER_INVALID.0,
    CreatecommandsignatureInvalid = D3D12_MESSAGE_ID_CREATECOMMANDSIGNATURE_INVALID.0,
    ExecuteIndirectInvalidParameters = D3D12_MESSAGE_ID_EXECUTE_INDIRECT_INVALID_PARAMETERS.0,
    GetgpuvirtualaddressInvalidResourceDimension =
        D3D12_MESSAGE_ID_GETGPUVIRTUALADDRESS_INVALID_RESOURCE_DIMENSION.0,
    CreateResourceInvalidclearvalue = D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDCLEARVALUE.0,
    CreateResourceUnrecognizedclearvalueformat =
        D3D12_MESSAGE_ID_CREATERESOURCE_UNRECOGNIZEDCLEARVALUEFORMAT.0,
    CreateResourceInvalidclearvalueformat =
        D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDCLEARVALUEFORMAT.0,
    CreateResourceClearvaluedenormflush = D3D12_MESSAGE_ID_CREATERESOURCE_CLEARVALUEDENORMFLUSH.0,
    ClearrendertargetviewMismatchingclearvalue =
        D3D12_MESSAGE_ID_CLEARRENDERTARGETVIEW_MISMATCHINGCLEARVALUE.0,
    CleardepthstencilviewMismatchingclearvalue =
        D3D12_MESSAGE_ID_CLEARDEPTHSTENCILVIEW_MISMATCHINGCLEARVALUE.0,
    MapInvalidheap = D3D12_MESSAGE_ID_MAP_INVALIDHEAP.0,
    UnmapInvalidheap = D3D12_MESSAGE_ID_UNMAP_INVALIDHEAP.0,
    MapInvalidresource = D3D12_MESSAGE_ID_MAP_INVALIDRESOURCE.0,
    UnmapInvalidresource = D3D12_MESSAGE_ID_UNMAP_INVALIDRESOURCE.0,
    MapInvalidSubresource = D3D12_MESSAGE_ID_MAP_INVALIDSUBRESOURCE.0,
    UnmapInvalidSubresource = D3D12_MESSAGE_ID_UNMAP_INVALIDSUBRESOURCE.0,
    MapInvalidrange = D3D12_MESSAGE_ID_MAP_INVALIDRANGE.0,
    UnmapInvalidrange = D3D12_MESSAGE_ID_UNMAP_INVALIDRANGE.0,
    MapInvaliddatapointer = D3D12_MESSAGE_ID_MAP_INVALIDDATAPOINTER.0,
    MapInvalidargReturn = D3D12_MESSAGE_ID_MAP_INVALIDARG_RETURN.0,
    MapOutOfMemoryReturn = D3D12_MESSAGE_ID_MAP_OUTOFMEMORY_RETURN.0,
    ExecuteCommandListsBundlenotsupported =
        D3D12_MESSAGE_ID_EXECUTECOMMANDLISTS_BUNDLENOTSUPPORTED.0,
    ExecuteCommandListsCommandlistmismatch =
        D3D12_MESSAGE_ID_EXECUTECOMMANDLISTS_COMMANDLISTMISMATCH.0,
    ExecuteCommandListsOpenCommandList = D3D12_MESSAGE_ID_EXECUTECOMMANDLISTS_OPENCOMMANDLIST.0,
    ExecuteCommandListsFailedCommandList = D3D12_MESSAGE_ID_EXECUTECOMMANDLISTS_FAILEDCOMMANDLIST.0,
    CopyBufferRegionNulldst = D3D12_MESSAGE_ID_COPYBUFFERREGION_NULLDST.0,
    CopyBufferRegionInvaliddstresourcedimension =
        D3D12_MESSAGE_ID_COPYBUFFERREGION_INVALIDDSTRESOURCEDIMENSION.0,
    CopyBufferRegionDstrangeoutofbounds = D3D12_MESSAGE_ID_COPYBUFFERREGION_DSTRANGEOUTOFBOUNDS.0,
    CopyBufferRegionNullsrc = D3D12_MESSAGE_ID_COPYBUFFERREGION_NULLSRC.0,
    CopyBufferRegionInvalidsrcresourcedimension =
        D3D12_MESSAGE_ID_COPYBUFFERREGION_INVALIDSRCRESOURCEDIMENSION.0,
    CopyBufferRegionSrcrangeoutofbounds = D3D12_MESSAGE_ID_COPYBUFFERREGION_SRCRANGEOUTOFBOUNDS.0,
    CopyBufferRegionInvalidcopyflags = D3D12_MESSAGE_ID_COPYBUFFERREGION_INVALIDCOPYFLAGS.0,
    CopyTextureRegionNulldst = D3D12_MESSAGE_ID_COPYTEXTUREREGION_NULLDST.0,
    CopyTextureRegionUnrecognizeddsttype = D3D12_MESSAGE_ID_COPYTEXTUREREGION_UNRECOGNIZEDDSTTYPE.0,
    CopyTextureRegionInvaliddstresourcedimension =
        D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTRESOURCEDIMENSION.0,
    CopyTextureRegionInvaliddstresource = D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTRESOURCE.0,
    CopyTextureRegionInvaliddstSubresource =
        D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTSUBRESOURCE.0,
    CopyTextureRegionInvaliddstoffset = D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTOFFSET.0,
    CopyTextureRegionUnrecognizeddstformat =
        D3D12_MESSAGE_ID_COPYTEXTUREREGION_UNRECOGNIZEDDSTFORMAT.0,
    CopyTextureRegionInvaliddstformat = D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTFORMAT.0,
    CopyTextureRegionInvaliddstdimensions =
        D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTDIMENSIONS.0,
    CopyTextureRegionInvaliddstrowpitch = D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTROWPITCH.0,
    CopyTextureRegionInvaliddstplacement = D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTPLACEMENT.0,
    CopyTextureRegionInvaliddstdsplacedfootprintformat =
        D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTDSPLACEDFOOTPRINTFORMAT.0,
    CopyTextureRegionDstregionoutofbounds =
        D3D12_MESSAGE_ID_COPYTEXTUREREGION_DSTREGIONOUTOFBOUNDS.0,
    CopyTextureRegionNullsrc = D3D12_MESSAGE_ID_COPYTEXTUREREGION_NULLSRC.0,
    CopyTextureRegionUnrecognizedsrctype = D3D12_MESSAGE_ID_COPYTEXTUREREGION_UNRECOGNIZEDSRCTYPE.0,
    CopyTextureRegionInvalidsrcresourcedimension =
        D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCRESOURCEDIMENSION.0,
    CopyTextureRegionInvalidsrcresource = D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCRESOURCE.0,
    CopyTextureRegionInvalidsrcSubresource =
        D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCSUBRESOURCE.0,
    CopyTextureRegionInvalidsrcoffset = D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCOFFSET.0,
    CopyTextureRegionUnrecognizedsrcformat =
        D3D12_MESSAGE_ID_COPYTEXTUREREGION_UNRECOGNIZEDSRCFORMAT.0,
    CopyTextureRegionInvalidsrcformat = D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCFORMAT.0,
    CopyTextureRegionInvalidsrcdimensions =
        D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCDIMENSIONS.0,
    CopyTextureRegionInvalidsrcrowpitch = D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCROWPITCH.0,
    CopyTextureRegionInvalidsrcplacement = D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCPLACEMENT.0,
    CopyTextureRegionInvalidsrcdsplacedfootprintformat =
        D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCDSPLACEDFOOTPRINTFORMAT.0,
    CopyTextureRegionSrcregionoutofbounds =
        D3D12_MESSAGE_ID_COPYTEXTUREREGION_SRCREGIONOUTOFBOUNDS.0,
    CopyTextureRegionInvaliddstcoordinates =
        D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTCOORDINATES.0,
    CopyTextureRegionInvalidsrcbox = D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCBOX.0,
    CopyTextureRegionFormatmismatch = D3D12_MESSAGE_ID_COPYTEXTUREREGION_FORMATMISMATCH.0,
    CopyTextureRegionEmptybox = D3D12_MESSAGE_ID_COPYTEXTUREREGION_EMPTYBOX.0,
    CopyTextureRegionInvalidcopyflags = D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDCOPYFLAGS.0,
    ResolveSubresourceInvalidSubresourceIndex =
        D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_INVALID_SUBRESOURCE_INDEX.0,
    ResolveSubresourceInvalidFormat = D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_INVALID_FORMAT.0,
    ResolveSubresourceResourceMismatch = D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_RESOURCE_MISMATCH.0,
    ResolveSubresourceInvalidSampleCount =
        D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_INVALID_SAMPLE_COUNT.0,
    CreateComputePipelineStateInvalidShader =
        D3D12_MESSAGE_ID_CREATECOMPUTEPIPELINESTATE_INVALID_SHADER.0,
    CreateComputePipelineStateCsRootSignatureMismatch =
        D3D12_MESSAGE_ID_CREATECOMPUTEPIPELINESTATE_CS_ROOT_SIGNATURE_MISMATCH.0,
    CreateComputePipelineStateMissingRootSignature =
        D3D12_MESSAGE_ID_CREATECOMPUTEPIPELINESTATE_MISSING_ROOT_SIGNATURE.0,
    CreatePipelineStateInvalidcachedblob = D3D12_MESSAGE_ID_CREATEPIPELINESTATE_INVALIDCACHEDBLOB.0,
    CreatePipelineStateCachedblobadaptermismatch =
        D3D12_MESSAGE_ID_CREATEPIPELINESTATE_CACHEDBLOBADAPTERMISMATCH.0,
    CreatePipelineStateCachedblobdriverversionmismatch =
        D3D12_MESSAGE_ID_CREATEPIPELINESTATE_CACHEDBLOBDRIVERVERSIONMISMATCH.0,
    CreatePipelineStateCachedblobdescmismatch =
        D3D12_MESSAGE_ID_CREATEPIPELINESTATE_CACHEDBLOBDESCMISMATCH.0,
    CreatePipelineStateCachedblobignored = D3D12_MESSAGE_ID_CREATEPIPELINESTATE_CACHEDBLOBIGNORED.0,
    WriteToSubresourceInvalidheap = D3D12_MESSAGE_ID_WRITETOSUBRESOURCE_INVALIDHEAP.0,
    WriteToSubresourceInvalidresource = D3D12_MESSAGE_ID_WRITETOSUBRESOURCE_INVALIDRESOURCE.0,
    WriteToSubresourceInvalidbox = D3D12_MESSAGE_ID_WRITETOSUBRESOURCE_INVALIDBOX.0,
    WriteToSubresourceInvalidSubresource = D3D12_MESSAGE_ID_WRITETOSUBRESOURCE_INVALIDSUBRESOURCE.0,
    WriteToSubresourceEmptybox = D3D12_MESSAGE_ID_WRITETOSUBRESOURCE_EMPTYBOX.0,
    ReadFromSubresourceInvalidheap = D3D12_MESSAGE_ID_READFROMSUBRESOURCE_INVALIDHEAP.0,
    ReadFromSubresourceInvalidresource = D3D12_MESSAGE_ID_READFROMSUBRESOURCE_INVALIDRESOURCE.0,
    ReadFromSubresourceInvalidbox = D3D12_MESSAGE_ID_READFROMSUBRESOURCE_INVALIDBOX.0,
    ReadFromSubresourceInvalidSubresource =
        D3D12_MESSAGE_ID_READFROMSUBRESOURCE_INVALIDSUBRESOURCE.0,
    ReadFromSubresourceEmptybox = D3D12_MESSAGE_ID_READFROMSUBRESOURCE_EMPTYBOX.0,
    TooManyNodesSpecified = D3D12_MESSAGE_ID_TOO_MANY_NODES_SPECIFIED.0,
    InvalidNodeIndex = D3D12_MESSAGE_ID_INVALID_NODE_INDEX.0,
    GetheappropertiesInvalidresource = D3D12_MESSAGE_ID_GETHEAPPROPERTIES_INVALIDRESOURCE.0,
    NodeMaskMismatch = D3D12_MESSAGE_ID_NODE_MASK_MISMATCH.0,
    CommandListOutOfMemory = D3D12_MESSAGE_ID_COMMAND_LIST_OUTOFMEMORY.0,
    CommandListMultipleSwapchainBufferReferences =
        D3D12_MESSAGE_ID_COMMAND_LIST_MULTIPLE_SWAPCHAIN_BUFFER_REFERENCES.0,
    CommandListTooManySwapchainReferences =
        D3D12_MESSAGE_ID_COMMAND_LIST_TOO_MANY_SWAPCHAIN_REFERENCES.0,
    CommandQueueTooManySwapchainReferences =
        D3D12_MESSAGE_ID_COMMAND_QUEUE_TOO_MANY_SWAPCHAIN_REFERENCES.0,
    ExecuteCommandListsWrongswapchainbufferreference =
        D3D12_MESSAGE_ID_EXECUTECOMMANDLISTS_WRONGSWAPCHAINBUFFERREFERENCE.0,
    CommandListSetrendertargetsInvalidnumrendertargets =
        D3D12_MESSAGE_ID_COMMAND_LIST_SETRENDERTARGETS_INVALIDNUMRENDERTARGETS.0,
    CreateQueueInvalidType = D3D12_MESSAGE_ID_CREATE_QUEUE_INVALID_TYPE.0,
    CreateQueueInvalidFlags = D3D12_MESSAGE_ID_CREATE_QUEUE_INVALID_FLAGS.0,
    CreateSharedResourceInvalidflags = D3D12_MESSAGE_ID_CREATESHAREDRESOURCE_INVALIDFLAGS.0,
    CreateSharedResourceInvalidformat = D3D12_MESSAGE_ID_CREATESHAREDRESOURCE_INVALIDFORMAT.0,
    CreateSharedHeapInvalidflags = D3D12_MESSAGE_ID_CREATESHAREDHEAP_INVALIDFLAGS.0,
    ReflectsharedpropertiesUnrecognizedproperties =
        D3D12_MESSAGE_ID_REFLECTSHAREDPROPERTIES_UNRECOGNIZEDPROPERTIES.0,
    ReflectsharedpropertiesInvalidsize = D3D12_MESSAGE_ID_REFLECTSHAREDPROPERTIES_INVALIDSIZE.0,
    ReflectsharedpropertiesInvalidobject = D3D12_MESSAGE_ID_REFLECTSHAREDPROPERTIES_INVALIDOBJECT.0,
    KeyedmutexInvalidobject = D3D12_MESSAGE_ID_KEYEDMUTEX_INVALIDOBJECT.0,
    KeyedmutexInvalidkey = D3D12_MESSAGE_ID_KEYEDMUTEX_INVALIDKEY.0,
    KeyedmutexWrongstate = D3D12_MESSAGE_ID_KEYEDMUTEX_WRONGSTATE.0,
    CreateQueueInvalidPriority = D3D12_MESSAGE_ID_CREATE_QUEUE_INVALID_PRIORITY.0,
    ObjectDeletedWhileStillInUse = D3D12_MESSAGE_ID_OBJECT_DELETED_WHILE_STILL_IN_USE.0,
    CreatePipelineStateInvalidFlags = D3D12_MESSAGE_ID_CREATEPIPELINESTATE_INVALID_FLAGS.0,
    HeapAddressRangeHasNoResource = D3D12_MESSAGE_ID_HEAP_ADDRESS_RANGE_HAS_NO_RESOURCE.0,
    CommandListDrawRenderTargetDeleted = D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_RENDER_TARGET_DELETED.0,
    CreateGraphicsPipelineStateAllRenderTargetsHaveUnknownFormat =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_ALL_RENDER_TARGETS_HAVE_UNKNOWN_FORMAT.0,
    HeapAddressRangeIntersectsMultipleBuffers =
        D3D12_MESSAGE_ID_HEAP_ADDRESS_RANGE_INTERSECTS_MULTIPLE_BUFFERS.0,
    ExecuteCommandListsGpuWrittenReadbackResourceMapped =
        D3D12_MESSAGE_ID_EXECUTECOMMANDLISTS_GPU_WRITTEN_READBACK_RESOURCE_MAPPED.0,
    UnmapRangeNotEmpty = D3D12_MESSAGE_ID_UNMAP_RANGE_NOT_EMPTY.0,
    MapInvalidNullrange = D3D12_MESSAGE_ID_MAP_INVALID_NULLRANGE.0,
    UnmapInvalidNullrange = D3D12_MESSAGE_ID_UNMAP_INVALID_NULLRANGE.0,
    NoGraphicsApiSupport = D3D12_MESSAGE_ID_NO_GRAPHICS_API_SUPPORT.0,
    NoComputeApiSupport = D3D12_MESSAGE_ID_NO_COMPUTE_API_SUPPORT.0,
    ResolveSubresourceResourceFlagsNotSupported =
        D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_RESOURCE_FLAGS_NOT_SUPPORTED.0,
    GpuBasedValidationRootArgumentUninitialized =
        D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_ROOT_ARGUMENT_UNINITIALIZED.0,
    GpuBasedValidationDescriptorHeapIndexOutOfBounds =
        D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_DESCRIPTOR_HEAP_INDEX_OUT_OF_BOUNDS.0,
    GpuBasedValidationDescriptorTableRegisterIndexOutOfBounds =
        D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_DESCRIPTOR_TABLE_REGISTER_INDEX_OUT_OF_BOUNDS.0,
    GpuBasedValidationDescriptorUninitialized =
        D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_DESCRIPTOR_UNINITIALIZED.0,
    GpuBasedValidationDescriptorTypeMismatch =
        D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_DESCRIPTOR_TYPE_MISMATCH.0,
    GpuBasedValidationSrvResourceDimensionMismatch =
        D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_SRV_RESOURCE_DIMENSION_MISMATCH.0,
    GpuBasedValidationUavResourceDimensionMismatch =
        D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_UAV_RESOURCE_DIMENSION_MISMATCH.0,
    GpuBasedValidationIncompatibleResourceState =
        D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_INCOMPATIBLE_RESOURCE_STATE.0,
    CopyresourceNulldst = D3D12_MESSAGE_ID_COPYRESOURCE_NULLDST.0,
    CopyresourceInvaliddstresource = D3D12_MESSAGE_ID_COPYRESOURCE_INVALIDDSTRESOURCE.0,
    CopyresourceNullsrc = D3D12_MESSAGE_ID_COPYRESOURCE_NULLSRC.0,
    CopyresourceInvalidsrcresource = D3D12_MESSAGE_ID_COPYRESOURCE_INVALIDSRCRESOURCE.0,
    ResolveSubresourceNulldst = D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_NULLDST.0,
    ResolveSubresourceInvaliddstresource = D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_INVALIDDSTRESOURCE.0,
    ResolveSubresourceNullsrc = D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_NULLSRC.0,
    ResolveSubresourceInvalidsrcresource = D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_INVALIDSRCRESOURCE.0,
    PipelineStateTypeMismatch = D3D12_MESSAGE_ID_PIPELINE_STATE_TYPE_MISMATCH.0,
    CommandListDispatchRootSignatureNotSet =
        D3D12_MESSAGE_ID_COMMAND_LIST_DISPATCH_ROOT_SIGNATURE_NOT_SET.0,
    CommandListDispatchRootSignatureMismatch =
        D3D12_MESSAGE_ID_COMMAND_LIST_DISPATCH_ROOT_SIGNATURE_MISMATCH.0,
    ResourceBarrierZeroBarriers = D3D12_MESSAGE_ID_RESOURCE_BARRIER_ZERO_BARRIERS.0,
    BeginEndEventMismatch = D3D12_MESSAGE_ID_BEGIN_END_EVENT_MISMATCH.0,
    ResourceBarrierPossibleBeforeAfterMismatch =
        D3D12_MESSAGE_ID_RESOURCE_BARRIER_POSSIBLE_BEFORE_AFTER_MISMATCH.0,
    ResourceBarrierMismatchingBeginEnd = D3D12_MESSAGE_ID_RESOURCE_BARRIER_MISMATCHING_BEGIN_END.0,
    GpuBasedValidationInvalidResource = D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_INVALID_RESOURCE.0,
    UseOfZeroRefcountObject = D3D12_MESSAGE_ID_USE_OF_ZERO_REFCOUNT_OBJECT.0,
    ObjectEvictedWhileStillInUse = D3D12_MESSAGE_ID_OBJECT_EVICTED_WHILE_STILL_IN_USE.0,
    GpuBasedValidationRootDescriptorAccessOutOfBounds =
        D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_ROOT_DESCRIPTOR_ACCESS_OUT_OF_BOUNDS.0,
    CreatepipelinelibraryInvalidlibraryblob =
        D3D12_MESSAGE_ID_CREATEPIPELINELIBRARY_INVALIDLIBRARYBLOB.0,
    CreatepipelinelibraryDriverversionmismatch =
        D3D12_MESSAGE_ID_CREATEPIPELINELIBRARY_DRIVERVERSIONMISMATCH.0,
    CreatepipelinelibraryAdapterversionmismatch =
        D3D12_MESSAGE_ID_CREATEPIPELINELIBRARY_ADAPTERVERSIONMISMATCH.0,
    CreatepipelinelibraryUnsupported = D3D12_MESSAGE_ID_CREATEPIPELINELIBRARY_UNSUPPORTED.0,
    CreatePipelinelibrary = D3D12_MESSAGE_ID_CREATE_PIPELINELIBRARY.0,
    LivePipelinelibrary = D3D12_MESSAGE_ID_LIVE_PIPELINELIBRARY.0,
    DestroyPipelinelibrary = D3D12_MESSAGE_ID_DESTROY_PIPELINELIBRARY.0,
    StorepipelineNoname = D3D12_MESSAGE_ID_STOREPIPELINE_NONAME.0,
    StorepipelineDuplicatename = D3D12_MESSAGE_ID_STOREPIPELINE_DUPLICATENAME.0,
    LoadpipelineNamenotfound = D3D12_MESSAGE_ID_LOADPIPELINE_NAMENOTFOUND.0,
    LoadpipelineInvaliddesc = D3D12_MESSAGE_ID_LOADPIPELINE_INVALIDDESC.0,
    PipelinelibrarySerializeNotenoughmemory =
        D3D12_MESSAGE_ID_PIPELINELIBRARY_SERIALIZE_NOTENOUGHMEMORY.0,
    CreateGraphicsPipelineStatePsOutputRtOutputMismatch =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_PS_OUTPUT_RT_OUTPUT_MISMATCH.0,
    SeteventonmultiplefencecompletionInvalidflags =
        D3D12_MESSAGE_ID_SETEVENTONMULTIPLEFENCECOMPLETION_INVALIDFLAGS.0,
    CreateQueueVideoNotSupported = D3D12_MESSAGE_ID_CREATE_QUEUE_VIDEO_NOT_SUPPORTED.0,
    CreateCommandAllocatorVideoNotSupported =
        D3D12_MESSAGE_ID_CREATE_COMMAND_ALLOCATOR_VIDEO_NOT_SUPPORTED.0,
    CreatequeryHeapVideoDecodeStatisticsNotSupported =
        D3D12_MESSAGE_ID_CREATEQUERY_HEAP_VIDEO_DECODE_STATISTICS_NOT_SUPPORTED.0,
    CreateVideodecodeCommandList = D3D12_MESSAGE_ID_CREATE_VIDEODECODECOMMANDLIST.0,
    CreateVideodecoder = D3D12_MESSAGE_ID_CREATE_VIDEODECODER.0,
    CreateVideodecodestream = D3D12_MESSAGE_ID_CREATE_VIDEODECODESTREAM.0,
    LiveVideodecodeCommandList = D3D12_MESSAGE_ID_LIVE_VIDEODECODECOMMANDLIST.0,
    LiveVideodecoder = D3D12_MESSAGE_ID_LIVE_VIDEODECODER.0,
    LiveVideodecodestream = D3D12_MESSAGE_ID_LIVE_VIDEODECODESTREAM.0,
    DestroyVideodecodeCommandList = D3D12_MESSAGE_ID_DESTROY_VIDEODECODECOMMANDLIST.0,
    DestroyVideodecoder = D3D12_MESSAGE_ID_DESTROY_VIDEODECODER.0,
    DestroyVideodecodestream = D3D12_MESSAGE_ID_DESTROY_VIDEODECODESTREAM.0,
    DecodeFrameInvalidParameters = D3D12_MESSAGE_ID_DECODE_FRAME_INVALID_PARAMETERS.0,
    DeprecatedApi = D3D12_MESSAGE_ID_DEPRECATED_API.0,
    ResourceBarrierMismatchingCommandListType =
        D3D12_MESSAGE_ID_RESOURCE_BARRIER_MISMATCHING_COMMAND_LIST_TYPE.0,
    CommandListDescriptorTableNotSet = D3D12_MESSAGE_ID_COMMAND_LIST_DESCRIPTOR_TABLE_NOT_SET.0,
    CommandListRootConstantBufferViewNotSet =
        D3D12_MESSAGE_ID_COMMAND_LIST_ROOT_CONSTANT_BUFFER_VIEW_NOT_SET.0,
    CommandListRootShaderResourceViewNotSet =
        D3D12_MESSAGE_ID_COMMAND_LIST_ROOT_SHADER_RESOURCE_VIEW_NOT_SET.0,
    CommandListRootUnorderedAccessViewNotSet =
        D3D12_MESSAGE_ID_COMMAND_LIST_ROOT_UNORDERED_ACCESS_VIEW_NOT_SET.0,
    DiscardInvalidSubresourceRange = D3D12_MESSAGE_ID_DISCARD_INVALID_SUBRESOURCE_RANGE.0,
    DiscardOneSubresourceForMipsWithRects =
        D3D12_MESSAGE_ID_DISCARD_ONE_SUBRESOURCE_FOR_MIPS_WITH_RECTS.0,
    DiscardNoRectsForNonTexture2D = D3D12_MESSAGE_ID_DISCARD_NO_RECTS_FOR_NON_TEXTURE2D.0,
    CopyOnSameSubresource = D3D12_MESSAGE_ID_COPY_ON_SAME_SUBRESOURCE.0,
    SetresidencypriorityInvalidPageable = D3D12_MESSAGE_ID_SETRESIDENCYPRIORITY_INVALID_PAGEABLE.0,
    GpuBasedValidationUnsupported = D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_UNSUPPORTED.0,
    StaticDescriptorInvalidDescriptorChange =
        D3D12_MESSAGE_ID_STATIC_DESCRIPTOR_INVALID_DESCRIPTOR_CHANGE.0,
    DataStaticDescriptorInvalidDataChange =
        D3D12_MESSAGE_ID_DATA_STATIC_DESCRIPTOR_INVALID_DATA_CHANGE.0,
    DataStaticWhileSetAtExecuteDescriptorInvalidDataChange =
        D3D12_MESSAGE_ID_DATA_STATIC_WHILE_SET_AT_EXECUTE_DESCRIPTOR_INVALID_DATA_CHANGE.0,
    ExecuteBundleStaticDescriptorDataStaticNotSet =
        D3D12_MESSAGE_ID_EXECUTE_BUNDLE_STATIC_DESCRIPTOR_DATA_STATIC_NOT_SET.0,
    GpuBasedValidationResourceAccessOutOfBounds =
        D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_RESOURCE_ACCESS_OUT_OF_BOUNDS.0,
    GpuBasedValidationSamplerModeMismatch =
        D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_SAMPLER_MODE_MISMATCH.0,
    CreateFenceInvalidFlags = D3D12_MESSAGE_ID_CREATE_FENCE_INVALID_FLAGS.0,
    ResourceBarrierDuplicateSubresourceTransitions =
        D3D12_MESSAGE_ID_RESOURCE_BARRIER_DUPLICATE_SUBRESOURCE_TRANSITIONS.0,
    SetresidencypriorityInvalidPriority = D3D12_MESSAGE_ID_SETRESIDENCYPRIORITY_INVALID_PRIORITY.0,
    CreateDescriptorHeapLargeNumDescriptors =
        D3D12_MESSAGE_ID_CREATE_DESCRIPTOR_HEAP_LARGE_NUM_DESCRIPTORS.0,
    BeginEvent = D3D12_MESSAGE_ID_BEGIN_EVENT.0,
    EndEvent = D3D12_MESSAGE_ID_END_EVENT.0,
    CreatedeviceDebugLayerStartupOptions =
        D3D12_MESSAGE_ID_CREATEDEVICE_DEBUG_LAYER_STARTUP_OPTIONS.0,
    CreatedepthstencilstateDepthboundstestUnsupported =
        D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_DEPTHBOUNDSTEST_UNSUPPORTED.0,
    CreatePipelineStateDuplicateSubobject =
        D3D12_MESSAGE_ID_CREATEPIPELINESTATE_DUPLICATE_SUBOBJECT.0,
    CreatePipelineStateUnknownSubobject = D3D12_MESSAGE_ID_CREATEPIPELINESTATE_UNKNOWN_SUBOBJECT.0,
    CreatePipelineStateZeroSizeStream = D3D12_MESSAGE_ID_CREATEPIPELINESTATE_ZERO_SIZE_STREAM.0,
    CreatePipelineStateInvalidStream = D3D12_MESSAGE_ID_CREATEPIPELINESTATE_INVALID_STREAM.0,
    CreatePipelineStateCannotDeduceType = D3D12_MESSAGE_ID_CREATEPIPELINESTATE_CANNOT_DEDUCE_TYPE.0,
    CommandListStaticDescriptorResourceDimensionMismatch =
        D3D12_MESSAGE_ID_COMMAND_LIST_STATIC_DESCRIPTOR_RESOURCE_DIMENSION_MISMATCH.0,
    CreateCommandQueueInsufficientPrivilegeForGlobalRealtime =
        D3D12_MESSAGE_ID_CREATE_COMMAND_QUEUE_INSUFFICIENT_PRIVILEGE_FOR_GLOBAL_REALTIME.0,
    CreateCommandQueueInsufficientHardwareSupportForGlobalRealtime =
        D3D12_MESSAGE_ID_CREATE_COMMAND_QUEUE_INSUFFICIENT_HARDWARE_SUPPORT_FOR_GLOBAL_REALTIME.0,
    AtomiccopybufferInvalidArchitecture = D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_ARCHITECTURE.0,
    AtomiccopybufferNullDst = D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_NULL_DST.0,
    AtomiccopybufferInvalidDstResourceDimension =
        D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_DST_RESOURCE_DIMENSION.0,
    AtomiccopybufferDstRangeOutOfBounds =
        D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_DST_RANGE_OUT_OF_BOUNDS.0,
    AtomiccopybufferNullSrc = D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_NULL_SRC.0,
    AtomiccopybufferInvalidSrcResourceDimension =
        D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_SRC_RESOURCE_DIMENSION.0,
    AtomiccopybufferSrcRangeOutOfBounds =
        D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_SRC_RANGE_OUT_OF_BOUNDS.0,
    AtomiccopybufferInvalidOffsetAlignment =
        D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_OFFSET_ALIGNMENT.0,
    AtomiccopybufferNullDependentResources =
        D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_NULL_DEPENDENT_RESOURCES.0,
    AtomiccopybufferNullDependentSubresourceRanges =
        D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_NULL_DEPENDENT_SUBRESOURCE_RANGES.0,
    AtomiccopybufferInvalidDependentResource =
        D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_DEPENDENT_RESOURCE.0,
    AtomiccopybufferInvalidDependentSubresourceRange =
        D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_DEPENDENT_SUBRESOURCE_RANGE.0,
    AtomiccopybufferDependentSubresourceOutOfBounds =
        D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_DEPENDENT_SUBRESOURCE_OUT_OF_BOUNDS.0,
    AtomiccopybufferDependentRangeOutOfBounds =
        D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_DEPENDENT_RANGE_OUT_OF_BOUNDS.0,
    AtomiccopybufferZeroDependencies = D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_ZERO_DEPENDENCIES.0,
    DeviceCreateSharedHandleInvalidarg = D3D12_MESSAGE_ID_DEVICE_CREATE_SHARED_HANDLE_INVALIDARG.0,
    DescriptorHandleWithInvalidResource =
        D3D12_MESSAGE_ID_DESCRIPTOR_HANDLE_WITH_INVALID_RESOURCE.0,
    SetdepthboundsInvalidargs = D3D12_MESSAGE_ID_SETDEPTHBOUNDS_INVALIDARGS.0,
    GpuBasedValidationResourceStateImprecise =
        D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_RESOURCE_STATE_IMPRECISE.0,
    CommandListPipelineStateNotSet = D3D12_MESSAGE_ID_COMMAND_LIST_PIPELINE_STATE_NOT_SET.0,
    CreateGraphicsPipelineStateShaderModelMismatch =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_MODEL_MISMATCH.0,
    ObjectAccessedWhileStillInUse = D3D12_MESSAGE_ID_OBJECT_ACCESSED_WHILE_STILL_IN_USE.0,
    ProgrammableMsaaUnsupported = D3D12_MESSAGE_ID_PROGRAMMABLE_MSAA_UNSUPPORTED.0,
    SetsamplepositionsInvalidargs = D3D12_MESSAGE_ID_SETSAMPLEPOSITIONS_INVALIDARGS.0,
    ResolveSubresourceregionInvalidRect = D3D12_MESSAGE_ID_RESOLVESUBRESOURCEREGION_INVALID_RECT.0,
    CreateVideodecodecommandqueue = D3D12_MESSAGE_ID_CREATE_VIDEODECODECOMMANDQUEUE.0,
    CreateVideoprocessCommandList = D3D12_MESSAGE_ID_CREATE_VIDEOPROCESSCOMMANDLIST.0,
    CreateVideoprocesscommandqueue = D3D12_MESSAGE_ID_CREATE_VIDEOPROCESSCOMMANDQUEUE.0,
    LiveVideodecodecommandqueue = D3D12_MESSAGE_ID_LIVE_VIDEODECODECOMMANDQUEUE.0,
    LiveVideoprocessCommandList = D3D12_MESSAGE_ID_LIVE_VIDEOPROCESSCOMMANDLIST.0,
    LiveVideoprocesscommandqueue = D3D12_MESSAGE_ID_LIVE_VIDEOPROCESSCOMMANDQUEUE.0,
    DestroyVideodecodecommandqueue = D3D12_MESSAGE_ID_DESTROY_VIDEODECODECOMMANDQUEUE.0,
    DestroyVideoprocessCommandList = D3D12_MESSAGE_ID_DESTROY_VIDEOPROCESSCOMMANDLIST.0,
    DestroyVideoprocesscommandqueue = D3D12_MESSAGE_ID_DESTROY_VIDEOPROCESSCOMMANDQUEUE.0,
    CreateVideoprocessor = D3D12_MESSAGE_ID_CREATE_VIDEOPROCESSOR.0,
    CreateVideoprocessstream = D3D12_MESSAGE_ID_CREATE_VIDEOPROCESSSTREAM.0,
    LiveVideoprocessor = D3D12_MESSAGE_ID_LIVE_VIDEOPROCESSOR.0,
    LiveVideoprocessstream = D3D12_MESSAGE_ID_LIVE_VIDEOPROCESSSTREAM.0,
    DestroyVideoprocessor = D3D12_MESSAGE_ID_DESTROY_VIDEOPROCESSOR.0,
    DestroyVideoprocessstream = D3D12_MESSAGE_ID_DESTROY_VIDEOPROCESSSTREAM.0,
    ProcessFrameInvalidParameters = D3D12_MESSAGE_ID_PROCESS_FRAME_INVALID_PARAMETERS.0,
    CopyInvalidlayout = D3D12_MESSAGE_ID_COPY_INVALIDLAYOUT.0,
    CreateCryptoSession = D3D12_MESSAGE_ID_CREATE_CRYPTO_SESSION.0,
    CreateCryptoSessionPolicy = D3D12_MESSAGE_ID_CREATE_CRYPTO_SESSION_POLICY.0,
    CreateProtectedResourceSession = D3D12_MESSAGE_ID_CREATE_PROTECTED_RESOURCE_SESSION.0,
    LiveCryptoSession = D3D12_MESSAGE_ID_LIVE_CRYPTO_SESSION.0,
    LiveCryptoSessionPolicy = D3D12_MESSAGE_ID_LIVE_CRYPTO_SESSION_POLICY.0,
    LiveProtectedResourceSession = D3D12_MESSAGE_ID_LIVE_PROTECTED_RESOURCE_SESSION.0,
    DestroyCryptoSession = D3D12_MESSAGE_ID_DESTROY_CRYPTO_SESSION.0,
    DestroyCryptoSessionPolicy = D3D12_MESSAGE_ID_DESTROY_CRYPTO_SESSION_POLICY.0,
    DestroyProtectedResourceSession = D3D12_MESSAGE_ID_DESTROY_PROTECTED_RESOURCE_SESSION.0,
    ProtectedResourceSessionUnsupported = D3D12_MESSAGE_ID_PROTECTED_RESOURCE_SESSION_UNSUPPORTED.0,
    FenceInvalidoperation = D3D12_MESSAGE_ID_FENCE_INVALIDOPERATION.0,
    CreatequeryHeapCopyQueueTimestampsNotSupported =
        D3D12_MESSAGE_ID_CREATEQUERY_HEAP_COPY_QUEUE_TIMESTAMPS_NOT_SUPPORTED.0,
    SamplepositionsMismatchDeferred = D3D12_MESSAGE_ID_SAMPLEPOSITIONS_MISMATCH_DEFERRED.0,
    SamplepositionsMismatchRecordtimeAssumedfromfirstuse =
        D3D12_MESSAGE_ID_SAMPLEPOSITIONS_MISMATCH_RECORDTIME_ASSUMEDFROMFIRSTUSE.0,
    SamplepositionsMismatchRecordtimeAssumedfromclear =
        D3D12_MESSAGE_ID_SAMPLEPOSITIONS_MISMATCH_RECORDTIME_ASSUMEDFROMCLEAR.0,
    CreateVideodecoderheap = D3D12_MESSAGE_ID_CREATE_VIDEODECODERHEAP.0,
    LiveVideodecoderheap = D3D12_MESSAGE_ID_LIVE_VIDEODECODERHEAP.0,
    DestroyVideodecoderheap = D3D12_MESSAGE_ID_DESTROY_VIDEODECODERHEAP.0,
    OpenexistingheapInvalidargReturn = D3D12_MESSAGE_ID_OPENEXISTINGHEAP_INVALIDARG_RETURN.0,
    OpenexistingheapOutOfMemoryReturn = D3D12_MESSAGE_ID_OPENEXISTINGHEAP_OUTOFMEMORY_RETURN.0,
    OpenexistingheapInvalidaddress = D3D12_MESSAGE_ID_OPENEXISTINGHEAP_INVALIDADDRESS.0,
    OpenexistingheapInvalidhandle = D3D12_MESSAGE_ID_OPENEXISTINGHEAP_INVALIDHANDLE.0,
    WritebufferimmediateInvalidDest = D3D12_MESSAGE_ID_WRITEBUFFERIMMEDIATE_INVALID_DEST.0,
    WritebufferimmediateInvalidMode = D3D12_MESSAGE_ID_WRITEBUFFERIMMEDIATE_INVALID_MODE.0,
    WritebufferimmediateInvalidAlignment =
        D3D12_MESSAGE_ID_WRITEBUFFERIMMEDIATE_INVALID_ALIGNMENT.0,
    WritebufferimmediateNotSupported = D3D12_MESSAGE_ID_WRITEBUFFERIMMEDIATE_NOT_SUPPORTED.0,
    SetviewinstancemaskInvalidargs = D3D12_MESSAGE_ID_SETVIEWINSTANCEMASK_INVALIDARGS.0,
    ViewInstancingUnsupported = D3D12_MESSAGE_ID_VIEW_INSTANCING_UNSUPPORTED.0,
    ViewInstancingInvalidargs = D3D12_MESSAGE_ID_VIEW_INSTANCING_INVALIDARGS.0,
    CopyTextureRegionMismatchDecodeReferenceOnlyFlag =
        D3D12_MESSAGE_ID_COPYTEXTUREREGION_MISMATCH_DECODE_REFERENCE_ONLY_FLAG.0,
    CopyresourceMismatchDecodeReferenceOnlyFlag =
        D3D12_MESSAGE_ID_COPYRESOURCE_MISMATCH_DECODE_REFERENCE_ONLY_FLAG.0,
    CreateVideoDecodeHeapCapsFailure = D3D12_MESSAGE_ID_CREATE_VIDEO_DECODE_HEAP_CAPS_FAILURE.0,
    CreateVideoDecodeHeapCapsUnsupported =
        D3D12_MESSAGE_ID_CREATE_VIDEO_DECODE_HEAP_CAPS_UNSUPPORTED.0,
    VideoDecodeSupportInvalidInput = D3D12_MESSAGE_ID_VIDEO_DECODE_SUPPORT_INVALID_INPUT.0,
    CreateVideoDecoderUnsupported = D3D12_MESSAGE_ID_CREATE_VIDEO_DECODER_UNSUPPORTED.0,
    CreateGraphicsPipelineStateMetadataError =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_METADATA_ERROR.0,
    CreateGraphicsPipelineStateViewInstancingVertexSizeExceeded =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_VIEW_INSTANCING_VERTEX_SIZE_EXCEEDED.0,
    CreateGraphicsPipelineStateRuntimeInternalError =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_RUNTIME_INTERNAL_ERROR.0,
    NoVideoApiSupport = D3D12_MESSAGE_ID_NO_VIDEO_API_SUPPORT.0,
    VideoProcessSupportInvalidInput = D3D12_MESSAGE_ID_VIDEO_PROCESS_SUPPORT_INVALID_INPUT.0,
    CreateVideoProcessorCapsFailure = D3D12_MESSAGE_ID_CREATE_VIDEO_PROCESSOR_CAPS_FAILURE.0,
    VideoProcessSupportUnsupportedFormat =
        D3D12_MESSAGE_ID_VIDEO_PROCESS_SUPPORT_UNSUPPORTED_FORMAT.0,
    VideoDecodeFrameInvalidArgument = D3D12_MESSAGE_ID_VIDEO_DECODE_FRAME_INVALID_ARGUMENT.0,
    EnqueueMakeResidentInvalidFlags = D3D12_MESSAGE_ID_ENQUEUE_MAKE_RESIDENT_INVALID_FLAGS.0,
    OpenexistingheapUnsupported = D3D12_MESSAGE_ID_OPENEXISTINGHEAP_UNSUPPORTED.0,
    VideoProcessFramesInvalidArgument = D3D12_MESSAGE_ID_VIDEO_PROCESS_FRAMES_INVALID_ARGUMENT.0,
    VideoDecodeSupportUnsupported = D3D12_MESSAGE_ID_VIDEO_DECODE_SUPPORT_UNSUPPORTED.0,
    CreateCommandrecorder = D3D12_MESSAGE_ID_CREATE_COMMANDRECORDER.0,
    LiveCommandrecorder = D3D12_MESSAGE_ID_LIVE_COMMANDRECORDER.0,
    DestroyCommandrecorder = D3D12_MESSAGE_ID_DESTROY_COMMANDRECORDER.0,
    CreateCommandRecorderVideoNotSupported =
        D3D12_MESSAGE_ID_CREATE_COMMAND_RECORDER_VIDEO_NOT_SUPPORTED.0,
    CreateCommandRecorderInvalidSupportFlags =
        D3D12_MESSAGE_ID_CREATE_COMMAND_RECORDER_INVALID_SUPPORT_FLAGS.0,
    CreateCommandRecorderInvalidFlags = D3D12_MESSAGE_ID_CREATE_COMMAND_RECORDER_INVALID_FLAGS.0,
    CreateCommandRecorderMoreRecordersThanLogicalProcessors =
        D3D12_MESSAGE_ID_CREATE_COMMAND_RECORDER_MORE_RECORDERS_THAN_LOGICAL_PROCESSORS.0,
    CreateCommandpool = D3D12_MESSAGE_ID_CREATE_COMMANDPOOL.0,
    LiveCommandpool = D3D12_MESSAGE_ID_LIVE_COMMANDPOOL.0,
    DestroyCommandpool = D3D12_MESSAGE_ID_DESTROY_COMMANDPOOL.0,
    CreateCommandPoolInvalidFlags = D3D12_MESSAGE_ID_CREATE_COMMAND_POOL_INVALID_FLAGS.0,
    CreateCommandListVideoNotSupported = D3D12_MESSAGE_ID_CREATE_COMMAND_LIST_VIDEO_NOT_SUPPORTED.0,
    CommandRecorderSupportFlagsMismatch =
        D3D12_MESSAGE_ID_COMMAND_RECORDER_SUPPORT_FLAGS_MISMATCH.0,
    CommandRecorderContention = D3D12_MESSAGE_ID_COMMAND_RECORDER_CONTENTION.0,
    CommandRecorderUsageWithCreateCommandListCommandList =
        D3D12_MESSAGE_ID_COMMAND_RECORDER_USAGE_WITH_CREATECOMMANDLIST_COMMAND_LIST.0,
    CommandAllocatorUsageWithCreateCommandList1CommandList =
        D3D12_MESSAGE_ID_COMMAND_ALLOCATOR_USAGE_WITH_CREATECOMMANDLIST1_COMMAND_LIST.0,
    CannotExecuteEmptyCommandList = D3D12_MESSAGE_ID_CANNOT_EXECUTE_EMPTY_COMMAND_LIST.0,
    CannotResetCommandPoolWithOpenCommandLists =
        D3D12_MESSAGE_ID_CANNOT_RESET_COMMAND_POOL_WITH_OPEN_COMMAND_LISTS.0,
    CannotUseCommandRecorderWithoutCurrentTarget =
        D3D12_MESSAGE_ID_CANNOT_USE_COMMAND_RECORDER_WITHOUT_CURRENT_TARGET.0,
    CannotChangeCommandRecorderTargetWhileRecording =
        D3D12_MESSAGE_ID_CANNOT_CHANGE_COMMAND_RECORDER_TARGET_WHILE_RECORDING.0,
    CommandPoolSync = D3D12_MESSAGE_ID_COMMAND_POOL_SYNC.0,
    EvictUnderflow = D3D12_MESSAGE_ID_EVICT_UNDERFLOW.0,
    CreateMetaCommand = D3D12_MESSAGE_ID_CREATE_META_COMMAND.0,
    LiveMetaCommand = D3D12_MESSAGE_ID_LIVE_META_COMMAND.0,
    DestroyMetaCommand = D3D12_MESSAGE_ID_DESTROY_META_COMMAND.0,
    CopyBufferRegionInvalidDstResource = D3D12_MESSAGE_ID_COPYBUFFERREGION_INVALID_DST_RESOURCE.0,
    CopyBufferRegionInvalidSrcResource = D3D12_MESSAGE_ID_COPYBUFFERREGION_INVALID_SRC_RESOURCE.0,
    AtomiccopybufferInvalidDstResource = D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_DST_RESOURCE.0,
    AtomiccopybufferInvalidSrcResource = D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_SRC_RESOURCE.0,
    CreateplacedresourceonbufferNullBuffer =
        D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_NULL_BUFFER.0,
    CreateplacedresourceonbufferNullResourceDesc =
        D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_NULL_RESOURCE_DESC.0,
    CreateplacedresourceonbufferUnsupported =
        D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_UNSUPPORTED.0,
    CreateplacedresourceonbufferInvalidBufferDimension =
        D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_INVALID_BUFFER_DIMENSION.0,
    CreateplacedresourceonbufferInvalidBufferFlags =
        D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_INVALID_BUFFER_FLAGS.0,
    CreateplacedresourceonbufferInvalidBufferOffset =
        D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_INVALID_BUFFER_OFFSET.0,
    CreateplacedresourceonbufferInvalidResourceDimension =
        D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_INVALID_RESOURCE_DIMENSION.0,
    CreateplacedresourceonbufferInvalidResourceFlags =
        D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_INVALID_RESOURCE_FLAGS.0,
    CreateplacedresourceonbufferOutOfMemoryReturn =
        D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_OUTOFMEMORY_RETURN.0,
    CannotCreateGraphicsAndVideoCommandRecorder =
        D3D12_MESSAGE_ID_CANNOT_CREATE_GRAPHICS_AND_VIDEO_COMMAND_RECORDER.0,
    UpdatetilemappingsPossiblyMismatchingProperties =
        D3D12_MESSAGE_ID_UPDATETILEMAPPINGS_POSSIBLY_MISMATCHING_PROPERTIES.0,
    CreateCommandListInvalidCommandListType =
        D3D12_MESSAGE_ID_CREATE_COMMAND_LIST_INVALID_COMMAND_LIST_TYPE.0,
    ClearunorderedaccessviewIncompatibleWithStructuredBuffers =
        D3D12_MESSAGE_ID_CLEARUNORDEREDACCESSVIEW_INCOMPATIBLE_WITH_STRUCTURED_BUFFERS.0,
    ComputeOnlyDeviceOperationUnsupported =
        D3D12_MESSAGE_ID_COMPUTE_ONLY_DEVICE_OPERATION_UNSUPPORTED.0,
    BuildRaytracingAccelerationStructureInvalid =
        D3D12_MESSAGE_ID_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INVALID.0,
    EmitRaytracingAccelerationStructurePostbuildInfoInvalid =
        D3D12_MESSAGE_ID_EMIT_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_INVALID.0,
    CopyRaytracingAccelerationStructureInvalid =
        D3D12_MESSAGE_ID_COPY_RAYTRACING_ACCELERATION_STRUCTURE_INVALID.0,
    DispatchRaysInvalid = D3D12_MESSAGE_ID_DISPATCH_RAYS_INVALID.0,
    GetRaytracingAccelerationStructurePrebuildInfoInvalid =
        D3D12_MESSAGE_ID_GET_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO_INVALID.0,
    CreateLifetimetracker = D3D12_MESSAGE_ID_CREATE_LIFETIMETRACKER.0,
    LiveLifetimetracker = D3D12_MESSAGE_ID_LIVE_LIFETIMETRACKER.0,
    DestroyLifetimetracker = D3D12_MESSAGE_ID_DESTROY_LIFETIMETRACKER.0,
    DestroyownedobjectObjectnotowned = D3D12_MESSAGE_ID_DESTROYOWNEDOBJECT_OBJECTNOTOWNED.0,
    CreateTrackedworkload = D3D12_MESSAGE_ID_CREATE_TRACKEDWORKLOAD.0,
    LiveTrackedworkload = D3D12_MESSAGE_ID_LIVE_TRACKEDWORKLOAD.0,
    DestroyTrackedworkload = D3D12_MESSAGE_ID_DESTROY_TRACKEDWORKLOAD.0,
    RenderPassError = D3D12_MESSAGE_ID_RENDER_PASS_ERROR.0,
    MetaCommandIdInvalid = D3D12_MESSAGE_ID_META_COMMAND_ID_INVALID.0,
    MetaCommandUnsupportedParams = D3D12_MESSAGE_ID_META_COMMAND_UNSUPPORTED_PARAMS.0,
    MetaCommandFailedEnumeration = D3D12_MESSAGE_ID_META_COMMAND_FAILED_ENUMERATION.0,
    MetaCommandParameterSizeMismatch = D3D12_MESSAGE_ID_META_COMMAND_PARAMETER_SIZE_MISMATCH.0,
    UninitializedMetaCommand = D3D12_MESSAGE_ID_UNINITIALIZED_META_COMMAND.0,
    MetaCommandInvalidGpuVirtualAddress =
        D3D12_MESSAGE_ID_META_COMMAND_INVALID_GPU_VIRTUAL_ADDRESS.0,
    CreateVideoencodeCommandList = D3D12_MESSAGE_ID_CREATE_VIDEOENCODECOMMANDLIST.0,
    LiveVideoencodeCommandList = D3D12_MESSAGE_ID_LIVE_VIDEOENCODECOMMANDLIST.0,
    DestroyVideoencodeCommandList = D3D12_MESSAGE_ID_DESTROY_VIDEOENCODECOMMANDLIST.0,
    CreateVideoencodecommandqueue = D3D12_MESSAGE_ID_CREATE_VIDEOENCODECOMMANDQUEUE.0,
    LiveVideoencodecommandqueue = D3D12_MESSAGE_ID_LIVE_VIDEOENCODECOMMANDQUEUE.0,
    DestroyVideoencodecommandqueue = D3D12_MESSAGE_ID_DESTROY_VIDEOENCODECOMMANDQUEUE.0,
    CreateVideomotionestimator = D3D12_MESSAGE_ID_CREATE_VIDEOMOTIONESTIMATOR.0,
    LiveVideomotionestimator = D3D12_MESSAGE_ID_LIVE_VIDEOMOTIONESTIMATOR.0,
    DestroyVideomotionestimator = D3D12_MESSAGE_ID_DESTROY_VIDEOMOTIONESTIMATOR.0,
    CreateVideomotionvectorheap = D3D12_MESSAGE_ID_CREATE_VIDEOMOTIONVECTORHEAP.0,
    LiveVideomotionvectorheap = D3D12_MESSAGE_ID_LIVE_VIDEOMOTIONVECTORHEAP.0,
    DestroyVideomotionvectorheap = D3D12_MESSAGE_ID_DESTROY_VIDEOMOTIONVECTORHEAP.0,
    MultipleTrackedWorkloads = D3D12_MESSAGE_ID_MULTIPLE_TRACKED_WORKLOADS.0,
    MultipleTrackedWorkloadPairs = D3D12_MESSAGE_ID_MULTIPLE_TRACKED_WORKLOAD_PAIRS.0,
    OutOfOrderTrackedWorkloadPair = D3D12_MESSAGE_ID_OUT_OF_ORDER_TRACKED_WORKLOAD_PAIR.0,
    CannotAddTrackedWorkload = D3D12_MESSAGE_ID_CANNOT_ADD_TRACKED_WORKLOAD.0,
    IncompleteTrackedWorkloadPair = D3D12_MESSAGE_ID_INCOMPLETE_TRACKED_WORKLOAD_PAIR.0,
    CreateStateObjectError = D3D12_MESSAGE_ID_CREATE_STATE_OBJECT_ERROR.0,
    GetShaderIdentifierError = D3D12_MESSAGE_ID_GET_SHADER_IDENTIFIER_ERROR.0,
    GetShaderStackSizeError = D3D12_MESSAGE_ID_GET_SHADER_STACK_SIZE_ERROR.0,
    GetPipelineStackSizeError = D3D12_MESSAGE_ID_GET_PIPELINE_STACK_SIZE_ERROR.0,
    SetPipelineStackSizeError = D3D12_MESSAGE_ID_SET_PIPELINE_STACK_SIZE_ERROR.0,
    GetShaderIdentifierSizeInvalid = D3D12_MESSAGE_ID_GET_SHADER_IDENTIFIER_SIZE_INVALID.0,
    CheckDriverMatchingIdentifierInvalid =
        D3D12_MESSAGE_ID_CHECK_DRIVER_MATCHING_IDENTIFIER_INVALID.0,
    CheckDriverMatchingIdentifierDriverReportedIssue =
        D3D12_MESSAGE_ID_CHECK_DRIVER_MATCHING_IDENTIFIER_DRIVER_REPORTED_ISSUE.0,
    RenderPassInvalidResourceBarrier = D3D12_MESSAGE_ID_RENDER_PASS_INVALID_RESOURCE_BARRIER.0,
    RenderPassDisallowedApiCalled = D3D12_MESSAGE_ID_RENDER_PASS_DISALLOWED_API_CALLED.0,
    RenderPassCannotNestRenderPasses = D3D12_MESSAGE_ID_RENDER_PASS_CANNOT_NEST_RENDER_PASSES.0,
    RenderPassCannotEndWithoutBegin = D3D12_MESSAGE_ID_RENDER_PASS_CANNOT_END_WITHOUT_BEGIN.0,
    RenderPassCannotCloseCommandList = D3D12_MESSAGE_ID_RENDER_PASS_CANNOT_CLOSE_COMMAND_LIST.0,
    RenderPassGpuWorkWhileSuspended = D3D12_MESSAGE_ID_RENDER_PASS_GPU_WORK_WHILE_SUSPENDED.0,
    RenderPassMismatchingSuspendResume = D3D12_MESSAGE_ID_RENDER_PASS_MISMATCHING_SUSPEND_RESUME.0,
    RenderPassNoPriorSuspendWithinExecuteCommandLists =
        D3D12_MESSAGE_ID_RENDER_PASS_NO_PRIOR_SUSPEND_WITHIN_EXECUTECOMMANDLISTS.0,
    RenderPassNoSubsequentResumeWithinExecuteCommandLists =
        D3D12_MESSAGE_ID_RENDER_PASS_NO_SUBSEQUENT_RESUME_WITHIN_EXECUTECOMMANDLISTS.0,
    TrackedWorkloadCommandQueueMismatch =
        D3D12_MESSAGE_ID_TRACKED_WORKLOAD_COMMAND_QUEUE_MISMATCH.0,
    TrackedWorkloadNotSupported = D3D12_MESSAGE_ID_TRACKED_WORKLOAD_NOT_SUPPORTED.0,
    RenderPassMismatchingNoAccess = D3D12_MESSAGE_ID_RENDER_PASS_MISMATCHING_NO_ACCESS.0,
    RenderPassUnsupportedResolve = D3D12_MESSAGE_ID_RENDER_PASS_UNSUPPORTED_RESOLVE.0,
    ClearunorderedaccessviewInvalidResourcePtr =
        D3D12_MESSAGE_ID_CLEARUNORDEREDACCESSVIEW_INVALID_RESOURCE_PTR.0,
    Windows7FenceOutoforderSignal = D3D12_MESSAGE_ID_WINDOWS7_FENCE_OUTOFORDER_SIGNAL.0,
    Windows7FenceOutoforderWait = D3D12_MESSAGE_ID_WINDOWS7_FENCE_OUTOFORDER_WAIT.0,
    VideoCreateMotionEstimatorInvalidArgument =
        D3D12_MESSAGE_ID_VIDEO_CREATE_MOTION_ESTIMATOR_INVALID_ARGUMENT.0,
    VideoCreateMotionVectorHeapInvalidArgument =
        D3D12_MESSAGE_ID_VIDEO_CREATE_MOTION_VECTOR_HEAP_INVALID_ARGUMENT.0,
    EstimateMotionInvalidArgument = D3D12_MESSAGE_ID_ESTIMATE_MOTION_INVALID_ARGUMENT.0,
    ResolveMotionVectorHeapInvalidArgument =
        D3D12_MESSAGE_ID_RESOLVE_MOTION_VECTOR_HEAP_INVALID_ARGUMENT.0,
    GetgpuvirtualaddressInvalidHeapType = D3D12_MESSAGE_ID_GETGPUVIRTUALADDRESS_INVALID_HEAP_TYPE.0,
    SetBackgroundProcessingModeInvalidArgument =
        D3D12_MESSAGE_ID_SET_BACKGROUND_PROCESSING_MODE_INVALID_ARGUMENT.0,
    CreateCommandListInvalidCommandListTypeForFeatureLevel =
        D3D12_MESSAGE_ID_CREATE_COMMAND_LIST_INVALID_COMMAND_LIST_TYPE_FOR_FEATURE_LEVEL.0,
    CreateVideoextensioncommand = D3D12_MESSAGE_ID_CREATE_VIDEOEXTENSIONCOMMAND.0,
    LiveVideoextensioncommand = D3D12_MESSAGE_ID_LIVE_VIDEOEXTENSIONCOMMAND.0,
    DestroyVideoextensioncommand = D3D12_MESSAGE_ID_DESTROY_VIDEOEXTENSIONCOMMAND.0,
    InvalidVideoExtensionCommandId = D3D12_MESSAGE_ID_INVALID_VIDEO_EXTENSION_COMMAND_ID.0,
    VideoExtensionCommandInvalidArgument =
        D3D12_MESSAGE_ID_VIDEO_EXTENSION_COMMAND_INVALID_ARGUMENT.0,
    CreateRootSignatureNotUniqueInDxilLibrary =
        D3D12_MESSAGE_ID_CREATE_ROOT_SIGNATURE_NOT_UNIQUE_IN_DXIL_LIBRARY.0,
    VariableShadingRateNotAllowedWithTir =
        D3D12_MESSAGE_ID_VARIABLE_SHADING_RATE_NOT_ALLOWED_WITH_TIR.0,
    GeometryShaderOutputtingBothViewportArrayIndexAndShadingRateNotSupportedOnDevice = D3D12_MESSAGE_ID_GEOMETRY_SHADER_OUTPUTTING_BOTH_VIEWPORT_ARRAY_INDEX_AND_SHADING_RATE_NOT_SUPPORTED_ON_DEVICE.0,
    RssetshadingRateInvalidShadingRate = D3D12_MESSAGE_ID_RSSETSHADING_RATE_INVALID_SHADING_RATE.0,
    RssetshadingRateShadingRateNotPermittedByCap =
        D3D12_MESSAGE_ID_RSSETSHADING_RATE_SHADING_RATE_NOT_PERMITTED_BY_CAP.0,
    RssetshadingRateInvalidCombiner = D3D12_MESSAGE_ID_RSSETSHADING_RATE_INVALID_COMBINER.0,
    RssetshadingrateimageRequiresTier2 = D3D12_MESSAGE_ID_RSSETSHADINGRATEIMAGE_REQUIRES_TIER_2.0,
    RssetshadingrateRequiresTier1 = D3D12_MESSAGE_ID_RSSETSHADINGRATE_REQUIRES_TIER_1.0,
    ShadingRateImageIncorrectFormat = D3D12_MESSAGE_ID_SHADING_RATE_IMAGE_INCORRECT_FORMAT.0,
    ShadingRateImageIncorrectArraySize = D3D12_MESSAGE_ID_SHADING_RATE_IMAGE_INCORRECT_ARRAY_SIZE.0,
    ShadingRateImageIncorrectMipLevel = D3D12_MESSAGE_ID_SHADING_RATE_IMAGE_INCORRECT_MIP_LEVEL.0,
    ShadingRateImageIncorrectSampleCount =
        D3D12_MESSAGE_ID_SHADING_RATE_IMAGE_INCORRECT_SAMPLE_COUNT.0,
    ShadingRateImageIncorrectSampleQuality =
        D3D12_MESSAGE_ID_SHADING_RATE_IMAGE_INCORRECT_SAMPLE_QUALITY.0,
    NonRetailShaderModelWontValidate = D3D12_MESSAGE_ID_NON_RETAIL_SHADER_MODEL_WONT_VALIDATE.0,
    CreateGraphicsPipelineStateAsRootSignatureMismatch =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_AS_ROOT_SIGNATURE_MISMATCH.0,
    CreateGraphicsPipelineStateMsRootSignatureMismatch =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_MS_ROOT_SIGNATURE_MISMATCH.0,
    AddToStateObjectError = D3D12_MESSAGE_ID_ADD_TO_STATE_OBJECT_ERROR.0,
    CreateProtectedResourceSessionInvalidArgument =
        D3D12_MESSAGE_ID_CREATE_PROTECTED_RESOURCE_SESSION_INVALID_ARGUMENT.0,
    CreateGraphicsPipelineStateMsPsoDescMismatch =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_MS_PSO_DESC_MISMATCH.0,
    CreatePipelineStateMsIncompleteType = D3D12_MESSAGE_ID_CREATEPIPELINESTATE_MS_INCOMPLETE_TYPE.0,
    CreateGraphicsPipelineStateAsNotMsMismatch =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_AS_NOT_MS_MISMATCH.0,
    CreateGraphicsPipelineStateMsNotPsMismatch =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_MS_NOT_PS_MISMATCH.0,
    NonzeroSamplerFeedbackMipRegionWithIncompatibleFormat =
        D3D12_MESSAGE_ID_NONZERO_SAMPLER_FEEDBACK_MIP_REGION_WITH_INCOMPATIBLE_FORMAT.0,
    CreateGraphicsPipelineStateInputlayoutShaderMismatch =
        D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INPUTLAYOUT_SHADER_MISMATCH.0,
    EmptyDispatch = D3D12_MESSAGE_ID_EMPTY_DISPATCH.0,
    ResourceFormatRequiresSamplerFeedbackCapability =
        D3D12_MESSAGE_ID_RESOURCE_FORMAT_REQUIRES_SAMPLER_FEEDBACK_CAPABILITY.0,
    SamplerFeedbackMapInvalidMipRegion = D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_MAP_INVALID_MIP_REGION.0,
    SamplerFeedbackMapInvalidDimension = D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_MAP_INVALID_DIMENSION.0,
    SamplerFeedbackMapInvalidSampleCount =
        D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_MAP_INVALID_SAMPLE_COUNT.0,
    SamplerFeedbackMapInvalidSampleQuality =
        D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_MAP_INVALID_SAMPLE_QUALITY.0,
    SamplerFeedbackMapInvalidLayout = D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_MAP_INVALID_LAYOUT.0,
    SamplerFeedbackMapRequiresUnorderedAccessFlag =
        D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_MAP_REQUIRES_UNORDERED_ACCESS_FLAG.0,
    SamplerFeedbackCreateUavNullArguments =
        D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_CREATE_UAV_NULL_ARGUMENTS.0,
    SamplerFeedbackUavRequiresSamplerFeedbackCapability =
        D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_UAV_REQUIRES_SAMPLER_FEEDBACK_CAPABILITY.0,
    SamplerFeedbackCreateUavRequiresFeedbackMapFormat =
        D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_CREATE_UAV_REQUIRES_FEEDBACK_MAP_FORMAT.0,
    CreateMeshShaderInvalidShaderBytecode =
        D3D12_MESSAGE_ID_CREATEMESHSHADER_INVALIDSHADERBYTECODE.0,
    CreateMeshShaderOutOfMemory = D3D12_MESSAGE_ID_CREATEMESHSHADER_OUTOFMEMORY.0,
    CreateMeshShaderWithStreamOutputInvalidshadertype =
        D3D12_MESSAGE_ID_CREATEMESHSHADERWITHSTREAMOUTPUT_INVALIDSHADERTYPE.0,
    ResolveSubresourceSamplerFeedbackTranscodeInvalidFormat =
        D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_SAMPLER_FEEDBACK_TRANSCODE_INVALID_FORMAT.0,
    ResolveSubresourceSamplerFeedbackInvalidMipLevelCount =
        D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_SAMPLER_FEEDBACK_INVALID_MIP_LEVEL_COUNT.0,
    ResolveSubresourceSamplerFeedbackTranscodeArraySizeMismatch =
        D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_SAMPLER_FEEDBACK_TRANSCODE_ARRAY_SIZE_MISMATCH.0,
    SamplerFeedbackCreateUavMismatchingTargetedResource =
        D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_CREATE_UAV_MISMATCHING_TARGETED_RESOURCE.0,
    CreateMeshShaderOutputexceedsmaxsize = D3D12_MESSAGE_ID_CREATEMESHSHADER_OUTPUTEXCEEDSMAXSIZE.0,
    CreateMeshShaderGroupsharedexceedsmaxsize =
        D3D12_MESSAGE_ID_CREATEMESHSHADER_GROUPSHAREDEXCEEDSMAXSIZE.0,
    VertexShaderOutputtingBothViewportArrayIndexAndShadingRateNotSupportedOnDevice = D3D12_MESSAGE_ID_VERTEX_SHADER_OUTPUTTING_BOTH_VIEWPORT_ARRAY_INDEX_AND_SHADING_RATE_NOT_SUPPORTED_ON_DEVICE.0,
    MeshShaderOutputtingBothViewportArrayIndexAndShadingRateNotSupportedOnDevice = D3D12_MESSAGE_ID_MESH_SHADER_OUTPUTTING_BOTH_VIEWPORT_ARRAY_INDEX_AND_SHADING_RATE_NOT_SUPPORTED_ON_DEVICE.0,
    CreateMeshShaderMismatchedAsMsPayloadSize =
        D3D12_MESSAGE_ID_CREATEMESHSHADER_MISMATCHEDASMSPAYLOADSIZE.0,
    CreateRootSignatureUnboundedStaticDescriptors =
        D3D12_MESSAGE_ID_CREATE_ROOT_SIGNATURE_UNBOUNDED_STATIC_DESCRIPTORS.0,
    CreateAmplificationShaderInvalidShaderBytecode =
        D3D12_MESSAGE_ID_CREATEAMPLIFICATIONSHADER_INVALIDSHADERBYTECODE.0,
    CreateAmplificationShaderOutOfMemory = D3D12_MESSAGE_ID_CREATEAMPLIFICATIONSHADER_OUTOFMEMORY.0,
    CreateShaderCacheSession = D3D12_MESSAGE_ID_CREATE_SHADERCACHESESSION.0,
    LiveShaderCacheSession = D3D12_MESSAGE_ID_LIVE_SHADERCACHESESSION.0,
    DestroyShaderCacheSession = D3D12_MESSAGE_ID_DESTROY_SHADERCACHESESSION.0,
    CreateShaderCacheSessionInvalidargs = D3D12_MESSAGE_ID_CREATESHADERCACHESESSION_INVALIDARGS.0,
    CreateShaderCacheSessionDisabled = D3D12_MESSAGE_ID_CREATESHADERCACHESESSION_DISABLED.0,
    CreateShaderCacheSessionAlreadyopen = D3D12_MESSAGE_ID_CREATESHADERCACHESESSION_ALREADYOPEN.0,
    ShaderCacheControlDeveloperMode = D3D12_MESSAGE_ID_SHADERCACHECONTROL_DEVELOPERMODE.0,
    ShaderCacheControlInvalidFlags = D3D12_MESSAGE_ID_SHADERCACHECONTROL_INVALIDFLAGS.0,
    ShaderCacheControlStatealReadySet = D3D12_MESSAGE_ID_SHADERCACHECONTROL_STATEALREADYSET.0,
    ShaderCacheControlIgnoredFlag = D3D12_MESSAGE_ID_SHADERCACHECONTROL_IGNOREDFLAG.0,
    ShaderCacheSessionStoreValueAlreadyPresent =
        D3D12_MESSAGE_ID_SHADERCACHESESSION_STOREVALUE_ALREADYPRESENT.0,
    ShadercachesessionStorevalueHashCollision =
        D3D12_MESSAGE_ID_SHADERCACHESESSION_STOREVALUE_HASHCOLLISION.0,
    ShaderCacheSessionStoreValueCacheFull =
        D3D12_MESSAGE_ID_SHADERCACHESESSION_STOREVALUE_CACHEFULL.0,
    ShaderCacheSessionFindValueNotFound = D3D12_MESSAGE_ID_SHADERCACHESESSION_FINDVALUE_NOTFOUND.0,
    ShaderCacheSessionCorrupt = D3D12_MESSAGE_ID_SHADERCACHESESSION_CORRUPT.0,
    ShaderCacheSessionDisabled = D3D12_MESSAGE_ID_SHADERCACHESESSION_DISABLED.0,
    OversizedDispatch = D3D12_MESSAGE_ID_OVERSIZED_DISPATCH.0,
    CreateVideoEncoder = D3D12_MESSAGE_ID_CREATE_VIDEOENCODER.0,
    LiveVideoEncoder = D3D12_MESSAGE_ID_LIVE_VIDEOENCODER.0,
    DestroyVideoEncoder = D3D12_MESSAGE_ID_DESTROY_VIDEOENCODER.0,
    CreateVideoEncoderheap = D3D12_MESSAGE_ID_CREATE_VIDEOENCODERHEAP.0,
    LiveVideoEncoderheap = D3D12_MESSAGE_ID_LIVE_VIDEOENCODERHEAP.0,
    DestroyVideoEncoderheap = D3D12_MESSAGE_ID_DESTROY_VIDEOENCODERHEAP.0,
    CopyTextureRegionMismatchEncodeReferenceOnlyFlag =
        D3D12_MESSAGE_ID_COPYTEXTUREREGION_MISMATCH_ENCODE_REFERENCE_ONLY_FLAG.0,
    CopyresourceMismatchEncodeReferenceOnlyFlag =
        D3D12_MESSAGE_ID_COPYRESOURCE_MISMATCH_ENCODE_REFERENCE_ONLY_FLAG.0,
    EncodeFrameInvalidParameters = D3D12_MESSAGE_ID_ENCODE_FRAME_INVALID_PARAMETERS.0,
    EncodeFrameUnsupportedParameters = D3D12_MESSAGE_ID_ENCODE_FRAME_UNSUPPORTED_PARAMETERS.0,
    ResolveEncoderOutputMetadataInvalidParameters =
        D3D12_MESSAGE_ID_RESOLVE_ENCODER_OUTPUT_METADATA_INVALID_PARAMETERS.0,
    ResolveEncoderOutputMetadataUnsupportedParameters =
        D3D12_MESSAGE_ID_RESOLVE_ENCODER_OUTPUT_METADATA_UNSUPPORTED_PARAMETERS.0,
    CreateVideoEncoderInvalidParameters =
        D3D12_MESSAGE_ID_CREATE_VIDEO_ENCODER_INVALID_PARAMETERS.0,
    CreateVideoEncoderUnsupportedParameters =
        D3D12_MESSAGE_ID_CREATE_VIDEO_ENCODER_UNSUPPORTED_PARAMETERS.0,
    CreateVideoEncoderHeapInvalidParameters =
        D3D12_MESSAGE_ID_CREATE_VIDEO_ENCODER_HEAP_INVALID_PARAMETERS.0,
    CreateVideoEncoderHeapUnsupportedParameters =
        D3D12_MESSAGE_ID_CREATE_VIDEO_ENCODER_HEAP_UNSUPPORTED_PARAMETERS.0,
    CreateCommandListNullCommandallocator =
        D3D12_MESSAGE_ID_CREATECOMMANDLIST_NULL_COMMANDALLOCATOR.0,
    ClearUnorderedAccessViewInvalidDescriptorHandle =
        D3D12_MESSAGE_ID_CLEAR_UNORDERED_ACCESS_VIEW_INVALID_DESCRIPTOR_HANDLE.0,
    DescriptorHeapNotShaderVisible = D3D12_MESSAGE_ID_DESCRIPTOR_HEAP_NOT_SHADER_VISIBLE.0,
    CreateblendstateBlendopWarning = D3D12_MESSAGE_ID_CREATEBLENDSTATE_BLENDOP_WARNING.0,
    CreateblendstateBlendopalphaWarning = D3D12_MESSAGE_ID_CREATEBLENDSTATE_BLENDOPALPHA_WARNING.0,
    WriteCombinePerformanceWarning = D3D12_MESSAGE_ID_WRITE_COMBINE_PERFORMANCE_WARNING.0,
    ResolveQueryInvalidQueryState = D3D12_MESSAGE_ID_RESOLVE_QUERY_INVALID_QUERY_STATE.0,
    SetPrivateDataNoAccess = D3D12_MESSAGE_ID_SETPRIVATEDATA_NO_ACCESS.0,
    D3D12MessagesEnd = D3D12_MESSAGE_ID_D3D12_MESSAGES_END.0,
}

/// Debug message severity levels for an information queue.
///
/// For more information: [`D3D12_MESSAGE_SEVERITY  enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/ne-d3d12sdklayers-d3d12_message_severity)
#[derive(Clone, Copy, Debug, FromRepr, Hash, PartialEq, Eq)]
#[repr(i32)]
pub enum MessageSeverity {
    /// Indicates a corruption error.
    Corruption = D3D12_MESSAGE_SEVERITY_CORRUPTION.0,

    /// Indicates an error.
    Error = D3D12_MESSAGE_SEVERITY_ERROR.0,

    /// Indicates a warning.
    Warning = D3D12_MESSAGE_SEVERITY_WARNING.0,

    /// Indicates an information message.
    Info = D3D12_MESSAGE_SEVERITY_INFO.0,

    /// Indicates a message other than corruption, error, warning or information.
    Message = D3D12_MESSAGE_SEVERITY_MESSAGE.0,
}

/// Describes minimum precision support options for shaders in the current graphics driver.
///
/// For more information: [`D3D12_SHADER_MIN_PRECISION_SUPPORT enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_shader_min_precision_support)
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
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
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
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

/// Specifies the predication operation to apply.
///
/// For more information: [`D3D12_PREDICATION_OP enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_predication_op)
#[derive(Clone, Copy, Debug, FromRepr, Hash, PartialEq, Eq)]
#[repr(i32)]
pub enum PredicationOp {
    /// Enables predication if all 64-bits are zero.
    EqualZero = D3D12_PREDICATION_OP_EQUAL_ZERO.0,

    /// Enables predication if at least one of the 64-bits are not zero.
    NotEqualZero = D3D12_PREDICATION_OP_NOT_EQUAL_ZERO.0,
}

/// Values that indicate how the pipeline interprets vertex data that is bound to the input-assembler stage. These primitive topology values determine how the vertex data is rendered on screen.
///
/// For more information: [`D3D_PRIMITIVE_TOPOLOGY enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_primitive_topology)
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
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
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
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
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
#[repr(i32)]
pub enum QueryHeapType {
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

/// Specifies the type of query.
///
/// For more information: [`D3D12_QUERY_TYPE enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_query_type)
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
#[repr(i32)]
pub enum QueryType {
    /// Indicates the query is for depth/stencil occlusion counts.
    #[default]
    Occlusion = D3D12_QUERY_TYPE_OCCLUSION.0,

    /// Indicates the query is for a binary depth/stencil occlusion statistics.
    BinaryOcclusion = D3D12_QUERY_TYPE_BINARY_OCCLUSION.0,

    /// Indicates the query is for high definition GPU and CPU timestamps.
    Timestamp = D3D12_QUERY_TYPE_TIMESTAMP.0,

    /// Indicates the query type is for graphics pipeline statistics.
    PipelineStatistics = D3D12_QUERY_TYPE_PIPELINE_STATISTICS.0,

    /// Stream 0 output statistics. In Direct3D 12 there is no single stream output (SO) overflow query for all the output streams.
    /// Apps need to issue multiple single-stream queries, and then correlate the results.
    /// Stream output is the ability of the GPU to write vertices to a buffer. The stream output counters monitor progress.
    SoStatisticsStream0 = D3D12_QUERY_TYPE_SO_STATISTICS_STREAM0.0,

    /// Stream 1 output statistics.
    SoStatisticsStream1 = D3D12_QUERY_TYPE_SO_STATISTICS_STREAM1.0,

    /// Stream 2 output statistics.
    SoStatisticsStream2 = D3D12_QUERY_TYPE_SO_STATISTICS_STREAM2.0,

    /// Stream 3 output statistics.
    SoStatisticsStream3 = D3D12_QUERY_TYPE_SO_STATISTICS_STREAM3.0,

    /// Video decode statistics.
    VideoDecodeStatistics = D3D12_QUERY_TYPE_VIDEO_DECODE_STATISTICS.0,

    /// TBD
    PipelineStatistics1 = D3D12_QUERY_TYPE_PIPELINE_STATISTICS1.0,
}

/// Specifies the level of ray tracing support on the graphics device.
///
/// For more information: [`D3D12_RAYTRACING_TIER enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_raytracing_tier)
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
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
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
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
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
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
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
#[repr(i32)]
pub enum ResourceDimension {
    /// Resource is of unknown type.
    #[default]
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
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
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
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
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

/// Specifies the version of root signature layout.
///
/// For more information: [`D3D_ROOT_SIGNATURE_VERSION enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d_root_signature_version)
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
#[repr(i32)]
pub enum RotationMode {
    /// Unspecified rotation.
    #[default]
    Unspecified = DXGI_MODE_ROTATION_UNSPECIFIED.0,

    /// Specifies no rotation.
    Identity = DXGI_MODE_ROTATION_IDENTITY.0,

    /// Specifies 90 degrees of rotation.
    Rotate90 = DXGI_MODE_ROTATION_ROTATE90.0,

    /// Specifies 180 degrees of rotation.
    Rotate180 = DXGI_MODE_ROTATION_ROTATE180.0,

    /// Specifies 270 degrees of rotation.
    Rotate270 = DXGI_MODE_ROTATION_ROTATE270.0,
}

/// Defines constants that specify sampler feedback support.
///
/// For more information: [`D3D12_SAMPLER_FEEDBACK_TIER enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_sampler_feedback_tier)
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
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

/// Identifies resize behavior when the back-buffer size does not match the size of the target output.
///
/// For more information: [`DXGI_SCALING enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_2/ne-dxgi1_2-dxgi_scaling)
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
#[repr(i32)]
pub enum Scaling {
    /// Directs DXGI to make the back-buffer contents scale to fit the presentation target size.
    #[default]
    Stretch = DXGI_SCALING_STRETCH.0,

    /// Directs DXGI to make the back-buffer contents appear without any scaling when the presentation target size is not equal to the back-buffer size.
    None = DXGI_SCALING_NONE.0,

    /// Directs DXGI to make the back-buffer contents scale to fit the presentation target size, while preserving the aspect ratio of the back-buffer.
    /// If the scaled back-buffer does not fill the presentation area, it will be centered with black borders.
    AspectRatioStretch = DXGI_SCALING_ASPECT_RATIO_STRETCH.0,
}

/// Flags indicating how an image is stretched to fit a given monitor's resolution.
///
/// For more information: [`DXGI_MODE_SCALING enumeration`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/bb173066(v=vs.85))
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
#[repr(i32)]
pub enum ScalingMode {
    /// Unspecified scaling.
    #[default]
    Unspecified = DXGI_MODE_SCALING_UNSPECIFIED.0,

    /// Specifies no scaling. The image is centered on the display. This flag is typically used for a fixed-dot-pitch display (such as an LED display).
    Centered = DXGI_MODE_SCALING_CENTERED.0,

    /// Specifies stretched scaling.
    Stretched = DXGI_MODE_SCALING_STRETCHED.0,
}

/// Flags indicating the method the raster uses to create an image on a surface.
///
/// For more information: [`DXGI_MODE_SCANLINE_ORDER enumeration`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/bb173067(v=vs.85))
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
#[repr(i32)]
pub enum ScanlineOrdering {
    /// Scanline order is unspecified.
    #[default]
    Unspecified = DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED.0,

    /// The image is created from the first scanline to the last without skipping any.
    Progressive = DXGI_MODE_SCANLINE_ORDER_PROGRESSIVE.0,

    /// The image is created beginning with the upper field.
    UpperFieldFirst = DXGI_MODE_SCANLINE_ORDER_LOWER_FIELD_FIRST.0,

    /// The image is created beginning with the lower field.
    LowerFieldFirst = DXGI_MODE_SCANLINE_ORDER_UPPER_FIELD_FIRST.0,
}

/// Semantic HLSL name
///
/// For more information: ['Semantics'](https://learn.microsoft.com/en-us/windows/win32/direct3dhlsl/dx-graphics-hlsl-semantics)
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum SemanticName {
    /// Binormal
    Binormal(u8),

    /// Blend indices
    BlendIndices(u8),

    /// Blend weights
    BlendWeight(u8),

    /// Diffuse and specular color
    Color(u8),

    /// Normal vector
    Normal(u8),

    /// Vertex position in object space.
    Position(u8),

    /// Transformed vertex position.
    PositionT,

    /// Point size
    Psize(u8),

    /// Tangent
    Tangent(u8),

    /// Texture coordinates
    Texcoord(u8),
}

impl SemanticName {
    #[inline]
    pub(crate) fn name(&self) -> &'static CStr {
        match self {
            SemanticName::Binormal(_) => c"BINORMAL",
            SemanticName::BlendIndices(_) => c"BLENDINDICES",
            SemanticName::BlendWeight(_) => c"BLENDWEIGHT",
            SemanticName::Color(_) => c"COLOR",
            SemanticName::Normal(_) => c"NORMAL",
            SemanticName::Position(_) => c"POSITION",
            SemanticName::PositionT => c"POSITIONT",
            SemanticName::Psize(_) => c"PSIZE",
            SemanticName::Tangent(_) => c"TANGENT",
            SemanticName::Texcoord(_) => c"TEXCOORD",
        }
    }

    #[inline]
    pub(crate) fn index(&self) -> u8 {
        match *self {
            SemanticName::Binormal(n) => n,
            SemanticName::BlendIndices(n) => n,
            SemanticName::BlendWeight(n) => n,
            SemanticName::Color(n) => n,
            SemanticName::Normal(n) => n,
            SemanticName::Position(n) => n,
            SemanticName::PositionT => 0,
            SemanticName::Psize(n) => n,
            SemanticName::Tangent(n) => n,
            SemanticName::Texcoord(n) => n,
        }
    }
}

/// Specifies a shader model.
///
/// For more information: [`D3D_SHADER_MODEL enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d_shader_model)
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
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
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
#[repr(i32)]
pub enum ShaderVisibility {
    /// Specifies that all shader stages can access whatever is bound at the root signature slot.
    #[default]
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
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
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

/// Identifies the stencil operations that can be performed during depth-stencil testing.
///
/// For more information: [`D3D12_STENCIL_OP enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_stencil_op)
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
#[repr(i32)]
pub enum StencilOp {
    /// Keep the existing stencil data.
    #[default]
    Keep = D3D12_STENCIL_OP_KEEP.0,

    /// Set the stencil data to 0.
    Zero = D3D12_STENCIL_OP_ZERO.0,

    /// Set the stencil data to the reference value set by calling [`IGraphicsCommandList::om_set_stencil_ref`](crate::command_list::IGraphicsCommandList::om_set_stencil_ref).
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

/// Options for handling pixels in a display surface.
///
/// For more information: [`DXGI_SWAP_EFFECT enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/ne-dxgi-dxgi_swap_effect)
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
#[repr(i32)]
pub enum SwapEffect {
    /// Use this flag to specify the bit-block transfer (bitblt) model and to specify that DXGI discard the contents of the back buffer.
    #[default]
    Discard = DXGI_SWAP_EFFECT_DISCARD.0,

    /// Use this flag to specify the bitblt model and to specify that DXGI persist the contents of the back buffer.
    Sequential = DXGI_SWAP_EFFECT_SEQUENTIAL.0,

    /// Use this flag to specify the flip presentation model and to specify that DXGI persist the contents of the back buffer.
    FlipSequential = DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL.0,

    /// Use this flag to specify the flip presentation model and to specify that DXGI discard the contents of the back buffer after.
    FlipDiscard = DXGI_SWAP_EFFECT_FLIP_DISCARD.0,
}

/// Specifies texture layout options.
///
/// For more information: [`D3D12_TEXTURE_LAYOUT enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_texture_layout)
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
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
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
#[repr(i32)]
pub enum TiledResourcesTier {
    /// Indicates that textures cannot be created with the [`TextureLayout::UndefinedSwizzle64Kb`] layout.
    #[default]
    NotSupported = D3D12_TILED_RESOURCES_TIER_NOT_SUPPORTED.0,

    /// Indicates that 2D textures can be created with the [`TextureLayout::UndefinedSwizzle64Kb`] layout.
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
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
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
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
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
#[derive(Clone, Copy, Debug, Default, FromRepr, Hash, PartialEq, Eq)]
#[repr(i32)]
pub enum WaveMmaTier {
    /// Specifies that WaveMMA (wave_matrix) operations are not supported.
    #[default]
    NotSupported = D3D12_WAVE_MMA_TIER_NOT_SUPPORTED.0,

    /// Specifies that WaveMMA (wave_matrix) operations are supported.
    Tier1_0 = D3D12_WAVE_MMA_TIER_1_0.0,
}
