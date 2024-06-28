use crate::{FeatureObject, __Sealed};

use super::*;

/// Describes Direct3D 12 feature options in the current graphics driver.
///
/// For more information: [`D3D12_FEATURE_DATA_D3D12_OPTIONS structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_d3d12_options)
#[derive(Clone, Debug, Default)]
pub struct Options {
    double_precision_float_shader_ops: bool,
    output_merger_logic_op: bool,
    min_precision_support: MinPrecisionSupport,
    tiled_resources_tier: TiledResourcesTier,
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
    resource_heap_tier: ResourceHeapTier,
}

impl __Sealed for Options {}

impl FeatureObject for Options {
    const TYPE: FeatureType = FeatureType::Options;

    type Raw = D3D12_FEATURE_DATA_D3D12_OPTIONS;

    fn as_raw(&self) -> Self::Raw {
        todo!()
    }

    fn clone_from_raw(&mut self, raw: Self::Raw) {
        todo!()
    }
}
