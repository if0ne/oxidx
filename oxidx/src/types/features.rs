use crate::{FeatureObject, __Sealed};

use super::*;

/// Describes Direct3D 12 feature options in the current graphics driver.
///
/// For more information: [`D3D12_FEATURE_DATA_D3D12_OPTIONS structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_d3d12_options)
#[derive(Clone, Debug, Default)]
pub struct Options {
    /// Specifies whether double types are allowed for shader operations.
    double_precision_float_shader_ops: bool,

    /// Specifies whether logic operations are available in blend state.
    output_merger_logic_op: bool,

    /// A combination of [`MinPrecisionSupport`]-typed values that are combined by using a bitwise OR operation.
    /// The resulting value specifies minimum precision levels that the driver supports for shader stages.
    /// A value of zero indicates that the driver supports only full 32-bit precision for all shader stages.
    min_precision_support: MinPrecisionSupport,

    /// Specifies whether the hardware and driver support tiled resources.
    /// The runtime sets this member to a [`TiledResourcesTier`]-typed value that indicates if the hardware and driver support tiled resources and at what tier level.
    tiled_resources_tier: TiledResourcesTier,

    /// Specifies the level at which the hardware and driver support resource binding.
    /// The runtime sets this member to a [`ResourceBindingTier`]-typed value that indicates the tier level.
    resource_binding_tier: ResourceBindingTier,
    ps_specified_stencil_ref_supported: bool,
    typed_uav_load_additional_formats: bool,
    rovs_supported: bool,
    conservative_rasterization_tier: ConservativeRasterizationTier,
    max_gpu_virtual_address_bits_per_resource: u32,
    standard_swizzle_64kb_supported: bool,
    cross_node_sharing_tier: CrossNodeSharingTier,
    cross_adapter_row_major_texture_supported: bool,
    vp_and_rt_array_index_from_any_shader_feeding_rasterizer_supported_without_gs_emulation: bool,

    /// Specifies the level at which the hardware and driver require heap attribution related to resource type.
    /// The runtime sets this member to a [`ResourceHeapTier`] enumeration constant.
    resource_heap_tier: ResourceHeapTier,
}

impl __Sealed for Options {}

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
            MaxGPUVirtualAddressBitsPerResource: self.max_gpu_virtual_address_bits_per_resource,
            StandardSwizzle64KBSupported: self.standard_swizzle_64kb_supported.into(),
            CrossNodeSharingTier: self.cross_node_sharing_tier.as_raw(),
            CrossAdapterRowMajorTextureSupported: self.cross_adapter_row_major_texture_supported.into(),
            VPAndRTArrayIndexFromAnyShaderFeedingRasterizerSupportedWithoutGSEmulation: self.vp_and_rt_array_index_from_any_shader_feeding_rasterizer_supported_without_gs_emulation.into(),
            ResourceHeapTier: self.resource_heap_tier.as_raw(),
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
            max_gpu_virtual_address_bits_per_resource: raw.MaxGPUVirtualAddressBitsPerResource,
            standard_swizzle_64kb_supported: raw.StandardSwizzle64KBSupported.into(),
            cross_node_sharing_tier: raw.CrossNodeSharingTier.into(),
            cross_adapter_row_major_texture_supported: raw.CrossAdapterRowMajorTextureSupported.into(),
            vp_and_rt_array_index_from_any_shader_feeding_rasterizer_supported_without_gs_emulation: raw.VPAndRTArrayIndexFromAnyShaderFeedingRasterizerSupportedWithoutGSEmulation.into(),
            resource_heap_tier: raw.ResourceHeapTier.into(),
        }
    }
}
