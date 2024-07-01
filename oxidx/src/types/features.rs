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
    /// In multi-adapter operation, this indicates which physical adapter of the device is relevant.
    pub node_index: u32,

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
    type Input<'a> = ();
    type Output = Architecture;

    #[inline]
    fn into_raw(_: Self::Input<'_>) -> Self::Raw {
        D3D12_FEATURE_DATA_ARCHITECTURE::default()
    }

    #[inline]
    fn from_raw(raw: Self::Raw) -> Self::Output {
        Self::Output {
            node_index: raw.NodeIndex,
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
