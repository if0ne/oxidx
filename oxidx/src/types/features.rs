use std::num::NonZeroU32;

use smallvec::SmallVec;

use crate::{FeatureObject, __Sealed};

use super::*;

/// Describes Direct3D 12 feature options in the current graphics driver.
///
/// For more information: [`D3D12_FEATURE_DATA_D3D12_OPTIONS structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_d3d12_options)
#[derive(Debug)]
pub struct OptionsFeature;

impl __Sealed for OptionsFeature {}

/// Describes Direct3D 12 feature options in the current graphics driver.
///
/// For more information: [`D3D12_FEATURE_DATA_D3D12_OPTIONS structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_d3d12_options)
#[derive(Clone, Debug, Default)]
pub struct Options {
    /// Specifies whether double types are allowed for shader operations.
    pub double_precision_float_shader_ops: bool,

    /// Specifies whether logic operations are available in blend state.
    pub output_merger_logic_op: bool,

    /// A combination of [`MinPrecisionSupport`]-typed values that are combined by using a bitwise OR operation.
    /// The resulting value specifies minimum precision levels that the driver supports for shader stages.
    /// A value of zero indicates that the driver supports only full 32-bit precision for all shader stages.
    pub min_precision_support: MinPrecisionSupport,

    /// Specifies whether the hardware and driver support tiled resources.
    /// The runtime sets this member to a [`TiledResourcesTier`]-typed value that indicates if the hardware and driver support tiled resources and at what tier level.
    pub tiled_resources_tier: TiledResourcesTier,

    /// Specifies the level at which the hardware and driver support resource binding.
    /// The runtime sets this member to a [`ResourceBindingTier`]-typed value that indicates the tier level.
    pub resource_binding_tier: ResourceBindingTier,

    /// Specifies whether pixel shader stencil ref is supported.
    pub ps_specified_stencil_ref_supported: bool,

    /// Specifies whether the loading of additional formats for typed unordered-access views (UAVs) is supported.
    pub typed_uav_load_additional_formats: bool,

    /// Specifies whether Rasterizer Order Views (ROVs) are supported.
    pub rovs_supported: bool,

    /// Specifies the level at which the hardware and driver support conservative rasterization.
    pub conservative_rasterization_tier: ConservativeRasterizationTier,

    /// TRUE if the hardware supports textures with the 64KB standard swizzle pattern.
    /// Support for this pattern enables zero-copy texture optimizations while providing near-equilateral locality for each dimension within the texture.
    pub standard_swizzle_64kb_supported: bool,

    /// A [`CrossNodeSharingTier`] enumeration constant that specifies the level of sharing across nodes of an adapter that has multiple nodes, such as Tier 1 Emulated, Tier 1, or Tier 2.
    pub cross_node_sharing_tier: CrossNodeSharingTier,

    /// FALSE means the device only supports copy operations to and from cross-adapter row-major textures.
    /// TRUE means the device supports shader resource views, unordered access views, and render target views of cross-adapter row-major textures.
    pub cross_adapter_row_major_texture_supported: bool,

    /// Whether the viewport (VP) and Render Target (RT) array index from any shader feeding the rasterizer are supported without geometry shader emulation.
    pub vp_and_rt_array_index_from_any_shader_feeding_rasterizer_supported_without_gs_emulation:
        bool,

    /// Specifies the level at which the hardware and driver require heap attribution related to resource type.
    /// The runtime sets this member to a [`ResourceHeapTier`] enumeration constant.
    pub resource_heap_tier: ResourceHeapTier,
}

impl FeatureObject for OptionsFeature {
    const TYPE: FeatureType = FeatureType::Options;

    type Raw = D3D12_FEATURE_DATA_D3D12_OPTIONS;
    type Input<'a> = ();
    type Output = Options;

    #[inline]
    fn into_raw(_: Self::Input<'_>) -> Self::Raw {
        D3D12_FEATURE_DATA_D3D12_OPTIONS::default()
    }

    #[inline]
    fn from_raw(raw: Self::Raw) -> Self::Output {
        Self::Output {
            double_precision_float_shader_ops: raw.DoublePrecisionFloatShaderOps.into(),
            output_merger_logic_op: raw.OutputMergerLogicOp.into(),
            min_precision_support: raw.MinPrecisionSupport.into(),
            tiled_resources_tier: raw.TiledResourcesTier.into(),
            resource_binding_tier: raw.ResourceBindingTier.into(),
            ps_specified_stencil_ref_supported: raw.PSSpecifiedStencilRefSupported.into(),
            typed_uav_load_additional_formats: raw.TypedUAVLoadAdditionalFormats.into(),
            rovs_supported: raw.ROVsSupported.into(),
            conservative_rasterization_tier: raw.ConservativeRasterizationTier.into(),
            standard_swizzle_64kb_supported: raw.StandardSwizzle64KBSupported.into(),
            cross_node_sharing_tier: raw.CrossNodeSharingTier.into(),
            cross_adapter_row_major_texture_supported: raw.CrossAdapterRowMajorTextureSupported.into(),
            vp_and_rt_array_index_from_any_shader_feeding_rasterizer_supported_without_gs_emulation: raw.VPAndRTArrayIndexFromAnyShaderFeedingRasterizerSupportedWithoutGSEmulation.into(),
            resource_heap_tier: raw.ResourceHeapTier.into(),
        }
    }
}

/// Provides detail about the adapter architecture, so that your application can better optimize for certain adapter properties.
///
/// For more information: [`D3D12_FEATURE_DATA_ARCHITECTURE structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_architecture)
pub struct ArchitectureFeature;

impl __Sealed for ArchitectureFeature {}

/// Provides detail about the adapter architecture, so that your application can better optimize for certain adapter properties.
///
/// For more information: [`D3D12_FEATURE_DATA_ARCHITECTURE structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_architecture)
#[derive(Clone, Copy, Debug, Default)]
pub struct Architecture {
    /// Specifies whether the hardware and driver support a tile-based renderer.
    pub tile_based_renderer: bool,

    /// Specifies whether the hardware and driver support UMA.
    pub uma: bool,

    /// Specifies whether the hardware and driver support cache-coherent UMA.
    pub cache_coherent_uma: bool,
}

impl FeatureObject for ArchitectureFeature {
    const TYPE: FeatureType = FeatureType::Architecture;

    type Raw = D3D12_FEATURE_DATA_ARCHITECTURE;
    type Input<'a> = u32;
    type Output = Architecture;

    #[inline]
    fn into_raw(input: Self::Input<'_>) -> Self::Raw {
        D3D12_FEATURE_DATA_ARCHITECTURE {
            NodeIndex: input,
            ..Default::default()
        }
    }

    #[inline]
    fn from_raw(raw: Self::Raw) -> Self::Output {
        Self::Output {
            tile_based_renderer: raw.TileBasedRenderer.into(),
            uma: raw.UMA.into(),
            cache_coherent_uma: raw.CacheCoherentUMA.into(),
        }
    }
}

/// Describes info about the [`FeatureLevel`] supported by the current graphics driver.
///
/// For more information: [`D3D12_FEATURE_DATA_FEATURE_LEVELS structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_feature_levels)
#[derive(Debug)]
pub struct FeatureLevelsFeature;

impl __Sealed for FeatureLevelsFeature {}

impl FeatureObject for FeatureLevelsFeature {
    const TYPE: FeatureType = FeatureType::FeatureLevels;

    type Raw = D3D12_FEATURE_DATA_FEATURE_LEVELS;
    type Input<'a> = &'a [FeatureLevel];
    type Output = FeatureLevel;

    #[inline(always)]
    fn into_raw(input: Self::Input<'_>) -> Self::Raw {
        let raw = input
            .iter()
            .map(|feature| feature.as_raw())
            .collect::<SmallVec<[_; 8]>>();

        D3D12_FEATURE_DATA_FEATURE_LEVELS {
            NumFeatureLevels: raw.len() as u32,
            pFeatureLevelsRequested: raw.as_ptr() as *const _,
            ..Default::default()
        }
    }

    #[inline]
    fn from_raw(raw: Self::Raw) -> Self::Output {
        raw.MaxSupportedFeatureLevel.into()
    }
}

/// Describes which resources are supported by the current graphics driver for a given format.
///
/// For more information: [`D3D12_FEATURE_DATA_FORMAT_SUPPORT structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_format_support)
#[derive(Debug)]
pub struct FormatSupportFeature;

/// Describes which resources are supported by the current graphics driver for a given format.
///
/// For more information: [`D3D12_FEATURE_DATA_FORMAT_SUPPORT structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_format_support)
#[derive(Clone, Copy, Debug, Default)]
pub struct FormatSupport {
    /// A combination of [`FormatSupport1`]-typed values that are combined by using a bitwise OR operation. The resulting value specifies which resources are supported.
    pub support1: FormatSupport1,

    /// A combination of [`FormatSupport2`]-typed values that are combined by using a bitwise OR operation. The resulting value specifies which unordered resource options are supported.
    pub support2: FormatSupport2,
}

impl __Sealed for FormatSupportFeature {}

impl FeatureObject for FormatSupportFeature {
    const TYPE: FeatureType = FeatureType::FormatSupport;

    type Raw = D3D12_FEATURE_DATA_FORMAT_SUPPORT;
    type Input<'a> = Format;
    type Output = FormatSupport;

    #[inline]
    fn into_raw(input: Self::Input<'_>) -> Self::Raw {
        D3D12_FEATURE_DATA_FORMAT_SUPPORT {
            Format: input.as_raw(),
            ..Default::default()
        }
    }

    #[inline]
    fn from_raw(raw: Self::Raw) -> Self::Output {
        Self::Output {
            support1: raw.Support1.into(),
            support2: raw.Support2.into(),
        }
    }
}

/// Describes the multi-sampling image quality levels for a given format and sample count.
///
/// For more information: [`D3D12_FEATURE_DATA_MULTISAMPLE_QUALITY_LEVELS structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_multisample_quality_levels)
#[derive(Debug)]
pub struct MultisampleQualityLevelsFeature;

/// Describes the multi-sampling image quality levels for a given format and sample count.
///
/// For more information: [`D3D12_FEATURE_DATA_MULTISAMPLE_QUALITY_LEVELS structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_multisample_quality_levels)
pub struct MultisampleQualityLevelsInfo {
    /// A [`Format`]-typed value for the format to return info about.
    pub format: Format,

    /// The number of multi-samples per pixel to return info about.
    pub sample_count: NonZeroU32,
}

/// Describes the multi-sampling image quality levels for a given format and sample count.
///
/// For more information: [`D3D12_FEATURE_DATA_MULTISAMPLE_QUALITY_LEVELS structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_multisample_quality_levels)
#[derive(Clone, Copy, Debug, Default)]
pub struct MultisampleQualityLevels {
    /// Flags to control quality levels, as a bitwise-OR'd combination of [`MultisampleQualityLevelFlags`] enumeration constants.
    /// The resulting value specifies options for determining quality levels.
    pub flags: MultisampleQualityLevelFlags,

    /// The number of quality levels.
    pub num_quality_levels: u32,
}

impl __Sealed for MultisampleQualityLevelsFeature {}

impl FeatureObject for MultisampleQualityLevelsFeature {
    const TYPE: FeatureType = FeatureType::MultisampleQualityLevels;

    type Raw = D3D12_FEATURE_DATA_MULTISAMPLE_QUALITY_LEVELS;
    type Input<'a> = MultisampleQualityLevelsInfo;
    type Output = MultisampleQualityLevels;

    #[inline]
    fn into_raw(input: Self::Input<'_>) -> Self::Raw {
        D3D12_FEATURE_DATA_MULTISAMPLE_QUALITY_LEVELS {
            Format: input.format.as_raw(),
            SampleCount: input.sample_count.get(),
            ..Default::default()
        }
    }

    #[inline]
    fn from_raw(raw: Self::Raw) -> Self::Output {
        Self::Output {
            flags: raw.Flags.into(),
            num_quality_levels: raw.NumQualityLevels,
        }
    }
}

/// Describes a DXGI data format and plane count.
///
/// For more information: [`D3D12_FEATURE_DATA_FORMAT_INFO structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_format_info)
#[derive(Debug)]
pub struct FormatInfoFeature;

impl __Sealed for FormatInfoFeature {}

impl FeatureObject for FormatInfoFeature {
    const TYPE: FeatureType = FeatureType::FormatInfo;

    type Raw = D3D12_FEATURE_DATA_FORMAT_INFO;
    type Input<'a> = Format;
    type Output = u8;

    #[inline]
    fn into_raw(input: Self::Input<'_>) -> Self::Raw {
        D3D12_FEATURE_DATA_FORMAT_INFO {
            Format: input.as_raw(),
            ..Default::default()
        }
    }

    #[inline]
    fn from_raw(raw: Self::Raw) -> Self::Output {
        raw.PlaneCount
    }
}

/// Details the adapter's GPU virtual address space limitations, including maximum address bits per resource and per process.
///
/// For more information: [`D3D12_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_gpu_virtual_address_support)
#[derive(Debug)]
pub struct GpuVirtualAddressSupportFeature;

/// Details the adapter's GPU virtual address space limitations, including maximum address bits per resource and per process.
///
/// For more information: [`D3D12_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_gpu_virtual_address_support)
#[derive(Clone, Copy, Debug, Default)]
pub struct GpuVirtualAddressSupport {
    /// The maximum GPU virtual address bits per resource.
    pub max_gpu_virtual_address_bits_per_resource: u32,

    /// The maximum GPU virtual address bits per process.
    pub max_gpu_virtual_address_bits_per_process: u32,
}

impl __Sealed for GpuVirtualAddressSupportFeature {}

impl FeatureObject for GpuVirtualAddressSupportFeature {
    const TYPE: FeatureType = FeatureType::GpuVirtualAddressSupport;

    type Raw = D3D12_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT;
    type Input<'a> = ();
    type Output = GpuVirtualAddressSupport;

    #[inline]
    fn into_raw(_: Self::Input<'_>) -> Self::Raw {
        D3D12_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT::default()
    }

    #[inline]
    fn from_raw(raw: Self::Raw) -> Self::Output {
        Self::Output {
            max_gpu_virtual_address_bits_per_resource: raw.MaxGPUVirtualAddressBitsPerResource,
            max_gpu_virtual_address_bits_per_process: raw.MaxGPUVirtualAddressBitsPerProcess,
        }
    }
}

/// Contains the supported shader model.
///
/// For more information: [`D3D12_FEATURE_DATA_SHADER_MODEL structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_shader_model)
#[derive(Debug)]
pub struct ShaderModelFeature;

impl __Sealed for ShaderModelFeature {}

impl FeatureObject for ShaderModelFeature {
    const TYPE: FeatureType = FeatureType::ShaderModel;

    type Raw = D3D12_FEATURE_DATA_SHADER_MODEL;
    type Input<'a> = ();
    type Output = ShaderModel;

    #[inline]
    fn into_raw(_: Self::Input<'_>) -> Self::Raw {
        D3D12_FEATURE_DATA_SHADER_MODEL {
            HighestShaderModel: D3D_SHADER_MODEL_6_6,
        }
    }

    #[inline]
    fn from_raw(raw: Self::Raw) -> Self::Output {
        raw.HighestShaderModel.into()
    }
}

/// Describes the level of support for HLSL 6.0 wave operations.
///
/// For more information: [`D3D12_FEATURE_DATA_D3D12_OPTIONS1 structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_d3d12_options1)
#[derive(Debug)]
pub struct Options1Feature;

/// Describes the level of support for HLSL 6.0 wave operations.
///
/// For more information: [`D3D12_FEATURE_DATA_D3D12_OPTIONS1 structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_d3d12_options1)
#[derive(Clone, Copy, Debug, Default)]
pub struct Options1 {
    /// True if the driver supports HLSL 6.0 wave operations.
    pub wave_ops: bool,

    /// Specifies the baseline number of lanes in the SIMD wave that this implementation can support.
    /// This term is sometimes known as "wavefront size" or "warp width". Currently apps should rely only on this minimum value for sizing workloads.
    pub wave_lane_count_min: u32,

    /// Specifies the maximum number of lanes in the SIMD wave that this implementation can support.
    pub wave_lane_count_max: u32,

    /// Specifies the total number of SIMD lanes on the hardware.
    pub total_lane_count: u32,

    /// Indicates transitions are possible in and out of the CBV, and indirect argument states, on compute command lists.
    /// If [`DeviceInterface::check_feature_support`](crate::device::DeviceInterface::check_feature_support) succeeds this value will always be true.
    pub expanded_compute_resource_states: bool,

    /// Indicates that 64bit integer operations are supported.
    pub int64_shader_ops: bool,
}

impl __Sealed for Options1Feature {}

impl FeatureObject for Options1Feature {
    const TYPE: FeatureType = FeatureType::Options1;

    type Raw = D3D12_FEATURE_DATA_D3D12_OPTIONS1;
    type Input<'a> = ();
    type Output = Options1;

    #[inline]
    fn into_raw(_: Self::Input<'_>) -> Self::Raw {
        D3D12_FEATURE_DATA_D3D12_OPTIONS1::default()
    }

    #[inline]
    fn from_raw(raw: Self::Raw) -> Self::Output {
        Self::Output {
            wave_ops: raw.WaveOps.into(),
            wave_lane_count_min: raw.WaveLaneCountMin,
            wave_lane_count_max: raw.WaveLaneCountMax,
            total_lane_count: raw.TotalLaneCount,
            expanded_compute_resource_states: raw.ExpandedComputeResourceStates.into(),
            int64_shader_ops: raw.Int64ShaderOps.into(),
        }
    }
}

/// Indicates the level of support for protected resource sessions.
///
/// For more information: [`D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_SUPPORT structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_protected_resource_session_support)
#[derive(Debug)]
pub struct ProtectedResourceSessionSupportFeature;

impl __Sealed for ProtectedResourceSessionSupportFeature {}

impl FeatureObject for ProtectedResourceSessionSupportFeature {
    const TYPE: FeatureType = FeatureType::ProtectedResourceSessionSupport;

    type Raw = D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_SUPPORT;
    type Input<'a> = u32;
    type Output = ProtectedResourceSessionSupportFlags;

    #[inline]
    fn into_raw(input: Self::Input<'_>) -> Self::Raw {
        D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_SUPPORT {
            NodeIndex: input,
            ..Default::default()
        }
    }

    #[inline]
    fn from_raw(raw: Self::Raw) -> Self::Output {
        raw.Support.into()
    }
}

/// Indicates root signature version support.
///
/// For more information: [`D3D12_FEATURE_DATA_ROOT_SIGNATURE structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_root_signature)
#[derive(Debug)]
pub struct RootSignatureFeature;

impl __Sealed for RootSignatureFeature {}

impl FeatureObject for RootSignatureFeature {
    const TYPE: FeatureType = FeatureType::RootSignature;

    type Raw = D3D12_FEATURE_DATA_ROOT_SIGNATURE;
    type Input<'a> = ();
    type Output = RootSignatureVersion;

    #[inline]
    fn into_raw(_: Self::Input<'_>) -> Self::Raw {
        D3D12_FEATURE_DATA_ROOT_SIGNATURE {
            HighestVersion: D3D_ROOT_SIGNATURE_VERSION_1_1,
        }
    }

    #[inline]
    fn from_raw(raw: Self::Raw) -> Self::Output {
        raw.HighestVersion.into()
    }
}

/// Provides detail about each adapter's architectural details, so that your application can better optimize for certain adapter properties.
///
/// For more information: [`D3D12_FEATURE_DATA_ARCHITECTURE1 structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_architecture1)
#[derive(Debug)]
pub struct Architecture1Feature;

/// Provides detail about each adapter's architectural details, so that your application can better optimize for certain adapter properties.
///
/// For more information: [`D3D12_FEATURE_DATA_ARCHITECTURE1 structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_architecture1)
#[derive(Clone, Copy, Debug, Default)]
pub struct Architecture1 {
    /// Specifies whether the hardware and driver support a tile-based renderer.
    pub tile_based_renderer: bool,

    /// Specifies whether the hardware and driver support UMA.
    pub uma: bool,

    /// Specifies whether the hardware and driver support cache-coherent UMA.
    pub cache_coherent_uma: bool,

    /// Specifies whether the hardware and driver support isolated Memory Management Unit (MMU).
    pub isolated_mmu: bool,
}

impl __Sealed for Architecture1Feature {}

impl FeatureObject for Architecture1Feature {
    const TYPE: FeatureType = FeatureType::Architecture1;

    type Raw = D3D12_FEATURE_DATA_ARCHITECTURE1;
    type Input<'a> = u32;
    type Output = Architecture1;

    #[inline]
    fn into_raw(input: Self::Input<'_>) -> Self::Raw {
        D3D12_FEATURE_DATA_ARCHITECTURE1 {
            NodeIndex: input,
            ..Default::default()
        }
    }

    #[inline]
    fn from_raw(raw: Self::Raw) -> Self::Output {
        Self::Output {
            tile_based_renderer: raw.TileBasedRenderer.into(),
            uma: raw.UMA.into(),
            cache_coherent_uma: raw.CacheCoherentUMA.into(),
            isolated_mmu: raw.IsolatedMMU.into(),
        }
    }
}

/// Indicates the level of support that the adapter provides for depth-bounds tests and programmable sample positions.
///
/// For more information: [`D3D12_FEATURE_DATA_D3D12_OPTIONS2 structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_d3d12_options2)
#[derive(Debug)]
pub struct Options2Feature;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options2 {
    /// On return, contains true if depth-bounds tests are supported; otherwise, false.
    pub depth_bounds_test_supported: bool,

    /// On return, contains a value that indicates the level of support offered for programmable sample positions.
    pub programmable_sample_positions_tier: ProgrammableSamplePositionsTier,
}

impl __Sealed for Options2Feature {}

impl FeatureObject for Options2Feature {
    const TYPE: FeatureType = FeatureType::Options2;

    type Raw = D3D12_FEATURE_DATA_D3D12_OPTIONS2;
    type Input<'a> = ();
    type Output = Options2;

    #[inline]
    fn into_raw(_: Self::Input<'_>) -> Self::Raw {
        D3D12_FEATURE_DATA_D3D12_OPTIONS2::default()
    }

    #[inline]
    fn from_raw(raw: Self::Raw) -> Self::Output {
        Self::Output {
            depth_bounds_test_supported: raw.DepthBoundsTestSupported.into(),
            programmable_sample_positions_tier: raw.ProgrammableSamplePositionsTier.into(),
        }
    }
}

/// Describes the level of shader caching supported in the current graphics driver.
///
/// For more information: [`D3D12_FEATURE_DATA_SHADER_CACHE structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_shader_cache)
#[derive(Debug)]
pub struct ShaderCacheFeature;

impl __Sealed for ShaderCacheFeature {}

impl FeatureObject for ShaderCacheFeature {
    const TYPE: FeatureType = FeatureType::ShaderCache;

    type Raw = D3D12_FEATURE_DATA_SHADER_CACHE;
    type Input<'a> = ();
    type Output = CacheSupportFlags;

    #[inline]
    fn into_raw(_: Self::Input<'_>) -> Self::Raw {
        D3D12_FEATURE_DATA_SHADER_CACHE::default()
    }

    #[inline]
    fn from_raw(raw: Self::Raw) -> Self::Output {
        raw.SupportFlags.into()
    }
}

/// Details the adapter's support for prioritization of different command queue types.
///
/// For more information: [`D3D12_FEATURE_DATA_COMMAND_QUEUE_PRIORITY structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_command_queue_priority)
#[derive(Debug)]
pub struct CommandQueuePriorityFeature;

/// Describes the level of shader caching supported in the current graphics driver.
///
/// For more information: [`D3D12_FEATURE_DATA_SHADER_CACHE structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_shader_cache)
#[derive(Clone, Copy, Debug, Default)]
pub struct CommandQueuePriorityInput {
    /// The type of the command list you're interested in.
    pub command_list_type: CommandListType,

    /// The priority level you're interested in.
    pub priority: CommandQueuePriority,
}

impl __Sealed for CommandQueuePriorityFeature {}

impl FeatureObject for CommandQueuePriorityFeature {
    const TYPE: FeatureType = FeatureType::CommandQueuePriority;

    type Raw = D3D12_FEATURE_DATA_COMMAND_QUEUE_PRIORITY;
    type Input<'a> = CommandQueuePriorityInput;
    type Output = bool;

    #[inline]
    fn into_raw(input: Self::Input<'_>) -> Self::Raw {
        D3D12_FEATURE_DATA_COMMAND_QUEUE_PRIORITY {
            CommandListType: input.command_list_type.as_raw(),
            Priority: input.priority.as_raw() as u32,
            ..Default::default()
        }
    }

    #[inline]
    fn from_raw(raw: Self::Raw) -> Self::Output {
        raw.PriorityForTypeIsSupported.into()
    }
}

/// Indicates the level of support that the adapter provides for timestamp queries, format-casting, immediate write, view instancing, and barycentrics.
///
/// For more information: [`D3D12_FEATURE_DATA_D3D12_OPTIONS3 structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_d3d12_options3)
#[derive(Debug)]
pub struct Options3Feature;

/// Indicates the level of support that the adapter provides for timestamp queries, format-casting, immediate write, view instancing, and barycentrics.
///
/// For more information: [`D3D12_FEATURE_DATA_D3D12_OPTIONS3 structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_d3d12_options3)
#[derive(Clone, Copy, Debug, Default)]
pub struct Options3 {
    /// Indicates whether timestamp queries are supported on copy queues.
    pub copy_queue_timestamp_queries_supported: bool,

    /// Indicates whether casting from one fully typed format to another, compatible, format is supported.
    pub casting_fully_typed_format_supported: bool,

    /// Indicates the kinds of command lists that support the ability to write an immediate value directly from the command stream into a specified buffer.
    pub write_buffer_immediate_support_flags: CommandListSupportFlags,

    /// Indicates the level of support the adapter has for view instancing.
    pub view_instancing_tier: ViewInstancingTier,

    /// Indicates whether barycentrics are supported.
    pub barycentrics_supported: bool,
}

impl __Sealed for Options3Feature {}

impl FeatureObject for Options3Feature {
    const TYPE: FeatureType = FeatureType::Options3;

    type Raw = D3D12_FEATURE_DATA_D3D12_OPTIONS3;
    type Input<'a> = ();
    type Output = Options3;

    #[inline]
    fn into_raw(_: Self::Input<'_>) -> Self::Raw {
        D3D12_FEATURE_DATA_D3D12_OPTIONS3::default()
    }

    #[inline]
    fn from_raw(raw: Self::Raw) -> Self::Output {
        Self::Output {
            copy_queue_timestamp_queries_supported: raw.CopyQueueTimestampQueriesSupported.into(),
            casting_fully_typed_format_supported: raw.CastingFullyTypedFormatSupported.into(),
            write_buffer_immediate_support_flags: raw.WriteBufferImmediateSupportFlags.into(),
            view_instancing_tier: raw.ViewInstancingTier.into(),
            barycentrics_supported: raw.BarycentricsSupported.into(),
        }
    }
}

/// Provides detail about whether the adapter supports creating heaps from existing system memory.
/// Such heaps are not intended for general use, but are exceptionally useful for diagnostic purposes,
/// because they are guaranteed to persist even after the adapter faults or experiences a device-removal event.
///
/// For more information: [`D3D12_FEATURE_DATA_EXISTING_HEAPS structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_existing_heaps)
#[derive(Debug)]
pub struct ExistingHeapsFeature;

impl __Sealed for ExistingHeapsFeature {}

impl FeatureObject for ExistingHeapsFeature {
    const TYPE: FeatureType = FeatureType::ExistingHeaps;

    type Raw = D3D12_FEATURE_DATA_EXISTING_HEAPS;
    type Input<'a> = ();
    type Output = bool;

    #[inline]
    fn into_raw(_: Self::Input<'_>) -> Self::Raw {
        D3D12_FEATURE_DATA_EXISTING_HEAPS::default()
    }

    #[inline]
    fn from_raw(raw: Self::Raw) -> Self::Output {
        raw.Supported.into()
    }
}

/// Indicates the level of support for 64KB-aligned MSAA textures, cross-API sharing, and native 16-bit shader operations.
///
/// For more information: [`D3D12_FEATURE_DATA_D3D12_OPTIONS4 structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_d3d12_options4)
#[derive(Debug)]
pub struct Options4Feature;

/// Indicates the level of support for 64KB-aligned MSAA textures, cross-API sharing, and native 16-bit shader operations.
///
/// For more information: [`D3D12_FEATURE_DATA_D3D12_OPTIONS4 structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_d3d12_options4)
#[derive(Clone, Copy, Debug, Default)]
pub struct Options4 {
    /// Indicates whether 64KB-aligned MSAA textures are supported.
    pub msaa_64kb_aligned_texture_supported: bool,

    /// Indicates the tier of cross-API sharing support
    pub shared_resource_compatibility_tier: SharedResourceCompatibilityTier,

    /// Indicates native 16-bit shader operations are supported. These operations require shader model 6_2.
    pub native_16bit_shader_ops_supported: bool,
}

impl __Sealed for Options4Feature {}

impl FeatureObject for Options4Feature {
    const TYPE: FeatureType = FeatureType::Options4;

    type Raw = D3D12_FEATURE_DATA_D3D12_OPTIONS4;
    type Input<'a> = ();
    type Output = Options4;

    #[inline]
    fn into_raw(_: Self::Input<'_>) -> Self::Raw {
        D3D12_FEATURE_DATA_D3D12_OPTIONS4::default()
    }

    #[inline]
    fn from_raw(raw: Self::Raw) -> Self::Output {
        Self::Output {
            msaa_64kb_aligned_texture_supported: raw.MSAA64KBAlignedTextureSupported.into(),
            shared_resource_compatibility_tier: raw.SharedResourceCompatibilityTier.into(),
            native_16bit_shader_ops_supported: raw.Native16BitShaderOpsSupported.into(),
        }
    }
}

/// Indicates the level of support for heap serialization.
///
/// For more information: [`D3D12_FEATURE_DATA_SERIALIZATION structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_serialization)
#[derive(Debug)]
pub struct SerializationFeature;

impl __Sealed for SerializationFeature {}

impl FeatureObject for SerializationFeature {
    const TYPE: FeatureType = FeatureType::Serialization;

    type Raw = D3D12_FEATURE_DATA_SERIALIZATION;
    type Input<'a> = u32;
    type Output = HeapSerializationTier;

    #[inline]
    fn into_raw(input: Self::Input<'_>) -> Self::Raw {
        D3D12_FEATURE_DATA_SERIALIZATION {
            NodeIndex: input,
            ..Default::default()
        }
    }

    #[inline]
    fn from_raw(raw: Self::Raw) -> Self::Output {
        raw.HeapSerializationTier.into()
    }
}