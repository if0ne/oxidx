use crate::FeatureObject;

use super::*;

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

impl FeatureObject for Options {
    const TYPE: FeatureType = FeatureType::Options;

    type Raw = D3D12_FEATURE_DATA_D3D12_OPTIONS;

    #[inline]
    fn as_raw(&self) -> Self::Raw {
        D3D12_FEATURE_DATA_D3D12_OPTIONS {
            DoublePrecisionFloatShaderOps: self.double_precision_float_shader_ops.into(),
            OutputMergerLogicOp: self.output_merger_logic_op.into(),
            MinPrecisionSupport: self.min_precision_support.as_raw(),
            TiledResourcesTier: self.tiled_resources_tier.as_raw(),
            ResourceBindingTier: self.resource_binding_tier.as_raw(),
            PSSpecifiedStencilRefSupported: self.ps_specified_stencil_ref_supported.into(),
            TypedUAVLoadAdditionalFormats: self.typed_uav_load_additional_formats.into(),
            ROVsSupported: self.rovs_supported.into(),
            ConservativeRasterizationTier: self.conservative_rasterization_tier.as_raw(),
            StandardSwizzle64KBSupported: self.standard_swizzle_64kb_supported.into(),
            CrossNodeSharingTier: self.cross_node_sharing_tier.as_raw(),
            CrossAdapterRowMajorTextureSupported: self.cross_adapter_row_major_texture_supported.into(),
            VPAndRTArrayIndexFromAnyShaderFeedingRasterizerSupportedWithoutGSEmulation: self.vp_and_rt_array_index_from_any_shader_feeding_rasterizer_supported_without_gs_emulation.into(),
            ResourceHeapTier: self.resource_heap_tier.as_raw(),
            MaxGPUVirtualAddressBitsPerResource: 0,
        }
    }

    #[inline]
    fn from_raw(raw: Self::Raw) -> Self {
        Self {
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
#[derive(Clone, Debug, Default)]
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

impl FeatureObject for Architecture {
    const TYPE: FeatureType = FeatureType::Architecture;

    type Raw = D3D12_FEATURE_DATA_ARCHITECTURE;

    #[inline]
    fn as_raw(&self) -> Self::Raw {
        D3D12_FEATURE_DATA_ARCHITECTURE {
            NodeIndex: self.node_index,
            TileBasedRenderer: self.tile_based_renderer.into(),
            UMA: self.uma.into(),
            CacheCoherentUMA: self.cache_coherent_uma.into(),
        }
    }

    #[inline]
    fn from_raw(raw: Self::Raw) -> Self {
        Self {
            node_index: raw.NodeIndex,
            tile_based_renderer: raw.TileBasedRenderer.into(),
            uma: raw.UMA.into(),
            cache_coherent_uma: raw.CacheCoherentUMA.into(),
        }
    }
}