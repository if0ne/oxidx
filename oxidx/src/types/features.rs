use std::marker::PhantomData;

use crate::{FeatureObject, __Sealed};

use super::*;

/// Describes Direct3D 12 feature options in the current graphics driver.
///
/// For more information: [`D3D12_FEATURE_DATA_D3D12_OPTIONS structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_d3d12_options)
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(transparent)]
pub struct OptionsFeature(pub(crate) D3D12_FEATURE_DATA_D3D12_OPTIONS);

impl OptionsFeature {
    #[inline]
    pub fn double_precision_float_shader_ops(&self) -> bool {
        self.0.DoublePrecisionFloatShaderOps.into()
    }

    #[inline]
    pub fn output_merger_logic_op(&self) -> bool {
        self.0.OutputMergerLogicOp.into()
    }

    #[inline]
    pub fn min_precision_support(&self) -> MinPrecisionSupport {
        self.0.MinPrecisionSupport.into()
    }

    #[inline]
    pub fn tiled_resources_tier(&self) -> TiledResourcesTier {
        self.0.TiledResourcesTier.into()
    }

    #[inline]
    pub fn resource_binding_tier(&self) -> ResourceBindingTier {
        self.0.ResourceBindingTier.into()
    }

    #[inline]
    pub fn ps_specified_stencil_ref_supported(&self) -> bool {
        self.0.PSSpecifiedStencilRefSupported.into()
    }

    #[inline]
    pub fn typed_uav_load_additional_formats(&self) -> bool {
        self.0.TypedUAVLoadAdditionalFormats.into()
    }

    #[inline]
    pub fn rovs_supported(&self) -> bool {
        self.0.ROVsSupported.into()
    }

    #[inline]
    pub fn conservative_rasterization_tier(&self) -> ConservativeRasterizationTier {
        self.0.ConservativeRasterizationTier.into()
    }

    #[inline]
    pub fn standard_swizzle_64kb_supported(&self) -> bool {
        self.0.StandardSwizzle64KBSupported.into()
    }

    #[inline]
    pub fn cross_node_sharing_tier(&self) -> CrossNodeSharingTier {
        self.0.CrossNodeSharingTier.into()
    }

    #[inline]
    pub fn cross_adapter_row_major_texture_supported(&self) -> bool {
        self.0.CrossAdapterRowMajorTextureSupported.into()
    }

    #[inline]
    pub fn vp_and_rt_array_index_from_any_shader_feeding_rasterizer_supported_without_gs_emulation(
        &self,
    ) -> bool {
        self.0
            .VPAndRTArrayIndexFromAnyShaderFeedingRasterizerSupportedWithoutGSEmulation
            .into()
    }

    #[inline]
    pub fn resource_heap_tier(&self) -> ResourceHeapTier {
        self.0.ResourceHeapTier.into()
    }
}

impl __Sealed for OptionsFeature {}

impl FeatureObject for OptionsFeature {
    const TYPE: FeatureType = FeatureType::Options;
}

/// Provides detail about the adapter architecture, so that your application can better optimize for certain adapter properties.
///
/// For more information: [`D3D12_FEATURE_DATA_ARCHITECTURE structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_architecture)
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct ArchitectureFeature(pub(crate) D3D12_FEATURE_DATA_ARCHITECTURE);

impl ArchitectureFeature {
    #[inline]
    pub fn new(node_index: u32) -> Self {
        Self(D3D12_FEATURE_DATA_ARCHITECTURE {
            NodeIndex: node_index,
            ..Default::default()
        })
    }

    #[inline]
    pub fn tile_based_renderer(&self) -> bool {
        self.0.TileBasedRenderer.into()
    }

    #[inline]
    pub fn uma(&self) -> bool {
        self.0.UMA.into()
    }

    #[inline]
    pub fn cache_coherent_uma(&self) -> bool {
        self.0.CacheCoherentUMA.into()
    }
}

impl __Sealed for ArchitectureFeature {}

impl FeatureObject for ArchitectureFeature {
    const TYPE: FeatureType = FeatureType::Architecture;
}

/// Describes info about the [`feature levels`](https://learn.microsoft.com/en-us/windows/win32/direct3d11/overviews-direct3d-11-devices-downlevel-intro) supported by the current graphics driver.
///
/// For more information: [`D3D12_FEATURE_DATA_FEATURE_LEVELS structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_feature_levels)
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct FeatureLevelsFeature<'a>(
    pub(crate) D3D12_FEATURE_DATA_FEATURE_LEVELS,
    PhantomData<&'a ()>,
);

impl<'a> FeatureLevelsFeature<'a> {
    #[inline]
    pub fn new(feature_levels_requested: &'a [FeatureLevel]) -> Self {
        Self(
            D3D12_FEATURE_DATA_FEATURE_LEVELS {
                NumFeatureLevels: feature_levels_requested.len() as u32,
                pFeatureLevelsRequested: feature_levels_requested.as_ptr() as *const _,
                ..Default::default()
            },
            Default::default(),
        )
    }

    #[inline]
    pub fn max_supported_feature_level(&self) -> FeatureLevel {
        self.0.MaxSupportedFeatureLevel.into()
    }
}

impl __Sealed for FeatureLevelsFeature<'_> {}

impl FeatureObject for FeatureLevelsFeature<'_> {
    const TYPE: FeatureType = FeatureType::FeatureLevels;
}

/// Describes which resources are supported by the current graphics driver for a given format.
///
/// For more information: [`D3D12_FEATURE_DATA_FORMAT_SUPPORT structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_format_support)
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct FormatSupportFeature(pub(crate) D3D12_FEATURE_DATA_FORMAT_SUPPORT);

impl FormatSupportFeature {
    #[inline]
    pub fn new(format: Format) -> Self {
        Self(D3D12_FEATURE_DATA_FORMAT_SUPPORT {
            Format: format.as_raw(),
            ..Default::default()
        })
    }

    #[inline]
    pub fn support1(&self) -> FormatSupport1 {
        self.0.Support1.into()
    }

    #[inline]
    pub fn support2(&self) -> FormatSupport2 {
        self.0.Support2.into()
    }
}

impl __Sealed for FormatSupportFeature {}

impl FeatureObject for FormatSupportFeature {
    const TYPE: FeatureType = FeatureType::FormatSupport;
}

/// Describes the multi-sampling image quality levels for a given format and sample count.
///
/// For more information: [`D3D12_FEATURE_DATA_MULTISAMPLE_QUALITY_LEVELS structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_multisample_quality_levels)
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct MultisampleQualityLevelsFeature(
    pub(crate) D3D12_FEATURE_DATA_MULTISAMPLE_QUALITY_LEVELS,
);

impl MultisampleQualityLevelsFeature {
    #[inline]
    pub fn new(format: Format, sample_count: u32) -> Self {
        Self(D3D12_FEATURE_DATA_MULTISAMPLE_QUALITY_LEVELS {
            Format: format.as_raw(),
            SampleCount: sample_count,
            ..Default::default()
        })
    }

    #[inline]
    pub fn num_quality_levels(&self) -> u32 {
        self.0.NumQualityLevels
    }

    #[inline]
    pub fn flags(&self) -> MultisampleQualityLevelFlags {
        self.0.Flags.into()
    }
}

impl __Sealed for MultisampleQualityLevelsFeature {}

impl FeatureObject for MultisampleQualityLevelsFeature {
    const TYPE: FeatureType = FeatureType::MultisampleQualityLevels;
}

/// Describes a DXGI data format and plane count.
///
/// For more information: [`D3D12_FEATURE_DATA_FORMAT_INFO structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_format_info)
#[derive(Debug)]
#[repr(transparent)]
pub struct FormatInfoFeature(pub(crate) D3D12_FEATURE_DATA_FORMAT_INFO);

impl FormatInfoFeature {
    #[inline]
    pub fn new(format: Format) -> Self {
        Self(D3D12_FEATURE_DATA_FORMAT_INFO {
            Format: format.as_raw(),
            ..Default::default()
        })
    }

    #[inline]
    pub fn plane_count(&self) -> u8 {
        self.0.PlaneCount
    }
}

impl __Sealed for FormatInfoFeature {}

impl FeatureObject for FormatInfoFeature {
    const TYPE: FeatureType = FeatureType::FormatInfo;
}

/// Details the adapter's GPU virtual address space limitations, including maximum address bits per resource and per process.
///
/// For more information: [`D3D12_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_gpu_virtual_address_support)
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(transparent)]
pub struct GpuVirtualAddressSupportFeature(
    pub(crate) D3D12_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT,
);

impl GpuVirtualAddressSupportFeature {
    #[inline]
    pub fn max_gpu_virtual_address_bits_per_resource(&self) -> u32 {
        self.0.MaxGPUVirtualAddressBitsPerResource
    }

    #[inline]
    pub fn max_gpu_virtual_address_bits_per_process(&self) -> u32 {
        self.0.MaxGPUVirtualAddressBitsPerProcess
    }
}

impl __Sealed for GpuVirtualAddressSupportFeature {}

impl FeatureObject for GpuVirtualAddressSupportFeature {
    const TYPE: FeatureType = FeatureType::GpuVirtualAddressSupport;
}

/// Contains the supported shader model.
///
/// For more information: [`D3D12_FEATURE_DATA_SHADER_MODEL structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_shader_model)
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct ShaderModelFeature(pub(crate) D3D12_FEATURE_DATA_SHADER_MODEL);

impl ShaderModelFeature {
    #[inline]
    pub fn new(shader_model: ShaderModel) -> Self {
        Self(D3D12_FEATURE_DATA_SHADER_MODEL {
            HighestShaderModel: shader_model.as_raw(),
        })
    }

    #[inline]
    pub fn highest_shader_model(&self) -> ShaderModel {
        self.0.HighestShaderModel.into()
    }
}

impl __Sealed for ShaderModelFeature {}

impl FeatureObject for ShaderModelFeature {
    const TYPE: FeatureType = FeatureType::ShaderModel;
}

/// Describes the level of support for HLSL 6.0 wave operations.
///
/// For more information: [`D3D12_FEATURE_DATA_D3D12_OPTIONS1 structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_d3d12_options1)
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(transparent)]
pub struct Options1Feature(pub(crate) D3D12_FEATURE_DATA_D3D12_OPTIONS1);

impl Options1Feature {
    #[inline]
    pub fn wave_ops(&self) -> bool {
        self.0.WaveOps.into()
    }

    #[inline]
    pub fn wave_lane_count_min(&self) -> u32 {
        self.0.WaveLaneCountMin
    }

    #[inline]
    pub fn wave_lane_count_max(&self) -> u32 {
        self.0.WaveLaneCountMax
    }

    #[inline]
    pub fn total_lane_count(&self) -> u32 {
        self.0.TotalLaneCount
    }

    #[inline]
    pub fn expanded_compute_resource_states(&self) -> bool {
        self.0.ExpandedComputeResourceStates.into()
    }

    #[inline]
    pub fn int64_shader_ops(&self) -> bool {
        self.0.Int64ShaderOps.into()
    }
}

impl __Sealed for Options1Feature {}

impl FeatureObject for Options1Feature {
    const TYPE: FeatureType = FeatureType::Options1;
}

/// Indicates the level of support for protected resource sessions.
///
/// For more information: [`D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_SUPPORT structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_protected_resource_session_support)
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct ProtectedResourceSessionSupportFeature(
    pub(crate) D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_SUPPORT,
);

impl ProtectedResourceSessionSupportFeature {
    #[inline]
    pub fn new(node_index: u32) -> Self {
        Self(D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_SUPPORT {
            NodeIndex: node_index,
            ..Default::default()
        })
    }

    #[inline]
    pub fn support(&self) -> ProtectedResourceSessionSupportFlags {
        self.0.Support.into()
    }
}

impl __Sealed for ProtectedResourceSessionSupportFeature {}

impl FeatureObject for ProtectedResourceSessionSupportFeature {
    const TYPE: FeatureType = FeatureType::ProtectedResourceSessionSupport;
}

/// Indicates root signature version support.
///
/// For more information: [`D3D12_FEATURE_DATA_ROOT_SIGNATURE structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_root_signature)
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(transparent)]
pub struct RootSignatureFeature(pub(crate) D3D12_FEATURE_DATA_ROOT_SIGNATURE);

impl RootSignatureFeature {
    #[inline]
    pub fn highest_version(&self) -> RootSignatureVersion {
        self.0.HighestVersion.into()
    }
}

impl __Sealed for RootSignatureFeature {}

impl FeatureObject for RootSignatureFeature {
    const TYPE: FeatureType = FeatureType::RootSignature;
}

/// Provides detail about each adapter's architectural details, so that your application can better optimize for certain adapter properties.
///
/// For more information: [`D3D12_FEATURE_DATA_ARCHITECTURE1 structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_architecture1)
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct Architecture1Feature(pub(crate) D3D12_FEATURE_DATA_ARCHITECTURE1);

impl Architecture1Feature {
    #[inline]
    pub fn new(node_index: u32) -> Self {
        Self(D3D12_FEATURE_DATA_ARCHITECTURE1 {
            NodeIndex: node_index,
            ..Default::default()
        })
    }

    #[inline]
    pub fn tile_based_renderer(&self) -> bool {
        self.0.TileBasedRenderer.into()
    }

    #[inline]
    pub fn cache_coherent_uma(&self) -> bool {
        self.0.CacheCoherentUMA.into()
    }

    #[inline]
    pub fn uma(&self) -> bool {
        self.0.UMA.into()
    }

    #[inline]
    pub fn isolated_mmu(&self) -> bool {
        self.0.IsolatedMMU.into()
    }
}

impl __Sealed for Architecture1Feature {}

impl FeatureObject for Architecture1Feature {
    const TYPE: FeatureType = FeatureType::Architecture1;
}

/// Indicates the level of support that the adapter provides for depth-bounds tests and programmable sample positions.
///
/// For more information: [`D3D12_FEATURE_DATA_D3D12_OPTIONS2 structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_d3d12_options2)
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(transparent)]
pub struct Options2Feature(pub(crate) D3D12_FEATURE_DATA_D3D12_OPTIONS2);

impl Options2Feature {
    #[inline]
    pub fn depth_bounds_test_supported(&self) -> bool {
        self.0.DepthBoundsTestSupported.into()
    }

    #[inline]
    pub fn programmable_sample_positions_tier(&self) -> ProgrammableSamplePositionsTier {
        self.0.ProgrammableSamplePositionsTier.into()
    }
}

impl __Sealed for Options2Feature {}

impl FeatureObject for Options2Feature {
    const TYPE: FeatureType = FeatureType::Options2;
}

/// Describes the level of shader caching supported in the current graphics driver.
///
/// For more information: [`D3D12_FEATURE_DATA_SHADER_CACHE structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_shader_cache)
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(transparent)]
pub struct ShaderCacheFeature(pub(crate) D3D12_FEATURE_DATA_SHADER_CACHE);

impl ShaderCacheFeature {
    #[inline]
    pub fn support_flags(&self) -> CacheSupportFlags {
        self.0.SupportFlags.into()
    }
}

impl __Sealed for ShaderCacheFeature {}

impl FeatureObject for ShaderCacheFeature {
    const TYPE: FeatureType = FeatureType::ShaderCache;
}

/// Details the adapter's support for prioritization of different command queue types.
///
/// For more information: [`D3D12_FEATURE_DATA_COMMAND_QUEUE_PRIORITY structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_command_queue_priority)
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct CommandQueuePriorityFeature(pub(crate) D3D12_FEATURE_DATA_COMMAND_QUEUE_PRIORITY);

impl CommandQueuePriorityFeature {
    #[inline]
    pub fn new(command_list_type: CommandListType, priority: CommandQueuePriority) -> Self {
        Self(D3D12_FEATURE_DATA_COMMAND_QUEUE_PRIORITY {
            CommandListType: command_list_type.as_raw(),
            Priority: priority.as_raw() as u32,
            ..Default::default()
        })
    }

    #[inline]
    pub fn is_supported(&self) -> bool {
        self.0.PriorityForTypeIsSupported.into()
    }
}

impl __Sealed for CommandQueuePriorityFeature {}

impl FeatureObject for CommandQueuePriorityFeature {
    const TYPE: FeatureType = FeatureType::CommandQueuePriority;
}

/// Indicates the level of support that the adapter provides for timestamp queries, format-casting, immediate write, view instancing, and barycentrics.
///
/// For more information: [`D3D12_FEATURE_DATA_D3D12_OPTIONS3 structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_d3d12_options3)
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(transparent)]
pub struct Options3Feature(pub(crate) D3D12_FEATURE_DATA_D3D12_OPTIONS3);

impl Options3Feature {
    #[inline]
    pub fn copy_queue_timestamp_queries_supported(&self) -> bool {
        self.0.CopyQueueTimestampQueriesSupported.into()
    }

    #[inline]
    pub fn casting_fully_typed_format_supported(&self) -> bool {
        self.0.CastingFullyTypedFormatSupported.into()
    }

    #[inline]
    pub fn write_buffer_immediate_support_flags(&self) -> CommandListSupportFlags {
        self.0.WriteBufferImmediateSupportFlags.into()
    }

    #[inline]
    pub fn view_instancing_tier(&self) -> ViewInstancingTier {
        self.0.ViewInstancingTier.into()
    }

    #[inline]
    pub fn barycentrics_supported(&self) -> bool {
        self.0.BarycentricsSupported.into()
    }
}

impl __Sealed for Options3Feature {}

impl FeatureObject for Options3Feature {
    const TYPE: FeatureType = FeatureType::Options3;
}

/// Provides detail about whether the adapter supports creating heaps from existing system memory.
/// Such heaps are not intended for general use, but are exceptionally useful for diagnostic purposes,
/// because they are guaranteed to persist even after the adapter faults or experiences a device-removal event.
///
/// For more information: [`D3D12_FEATURE_DATA_EXISTING_HEAPS structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_existing_heaps)
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(transparent)]
pub struct ExistingHeapsFeature(pub(crate) D3D12_FEATURE_DATA_EXISTING_HEAPS);

impl ExistingHeapsFeature {
    #[inline]
    pub fn supported(&self) -> bool {
        self.0.Supported.into()
    }
}

impl __Sealed for ExistingHeapsFeature {}

impl FeatureObject for ExistingHeapsFeature {
    const TYPE: FeatureType = FeatureType::ExistingHeaps;
}

/// Indicates the level of support for 64KB-aligned MSAA textures, cross-API sharing, and native 16-bit shader operations.
///
/// For more information: [`D3D12_FEATURE_DATA_D3D12_OPTIONS4 structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_d3d12_options4)
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(transparent)]
pub struct Options4Feature(pub(crate) D3D12_FEATURE_DATA_D3D12_OPTIONS4);

impl Options4Feature {
    #[inline]
    pub fn msaa_64kb_aligned_texture_supported(&self) -> bool {
        self.0.MSAA64KBAlignedTextureSupported.into()
    }

    #[inline]
    pub fn shared_resource_compatibility_tier(&self) -> SharedResourceCompatibilityTier {
        self.0.SharedResourceCompatibilityTier.into()
    }

    #[inline]
    pub fn native_16bit_shader_ops_supported(&self) -> bool {
        self.0.Native16BitShaderOpsSupported.into()
    }
}

impl __Sealed for Options4Feature {}

impl FeatureObject for Options4Feature {
    const TYPE: FeatureType = FeatureType::Options4;
}

/// Indicates the level of support for heap serialization.
///
/// For more information: [`D3D12_FEATURE_DATA_SERIALIZATION structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_serialization)
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct SerializationFeature(pub(crate) D3D12_FEATURE_DATA_SERIALIZATION);

impl SerializationFeature {
    #[inline]
    pub fn new(node_index: u32) -> Self {
        Self(D3D12_FEATURE_DATA_SERIALIZATION {
            NodeIndex: node_index,
            ..Default::default()
        })
    }

    #[inline]
    pub fn heap_serialization_tier(&self) -> HeapSerializationTier {
        self.0.HeapSerializationTier.into()
    }
}

impl __Sealed for SerializationFeature {}

impl FeatureObject for SerializationFeature {
    const TYPE: FeatureType = FeatureType::Serialization;
}

/// Indicates the level of support for the sharing of resources between different adapters—for example, multiple GPUs.
///
/// For more information: [`D3D12_FEATURE_DATA_CROSS_NODE structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_cross_node)
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(transparent)]
pub struct CrossNodeFeature(pub(crate) D3D12_FEATURE_DATA_CROSS_NODE);

impl CrossNodeFeature {
    #[inline]
    pub fn sharing_tier(&self) -> CrossNodeSharingTier {
        self.0.SharingTier.into()
    }

    #[inline]
    pub fn atomic_shader_instructions(&self) -> bool {
        self.0.AtomicShaderInstructions.into()
    }
}

impl __Sealed for CrossNodeFeature {}

impl FeatureObject for CrossNodeFeature {
    const TYPE: FeatureType = FeatureType::CrossNode;
}

/// Indicates the level of support that the adapter provides for render passes, ray tracing, and shader-resource view tier 3 tiled resources.
///
/// For more information: [`D3D12_FEATURE_DATA_D3D12_OPTIONS5 structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_d3d12_options5)
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(transparent)]
pub struct Options5Feature(pub(crate) D3D12_FEATURE_DATA_D3D12_OPTIONS5);

impl Options5Feature {
    #[inline]
    pub fn srv_only_tiled_resource_tier3(&self) -> bool {
        self.0.SRVOnlyTiledResourceTier3.into()
    }

    #[inline]
    pub fn render_passes_tier(&self) -> RenderPassTier {
        self.0.RenderPassesTier.into()
    }

    #[inline]
    pub fn raytracing_tier(&self) -> RaytracingTier {
        self.0.RaytracingTier.into()
    }
}

impl __Sealed for Options5Feature {}

impl FeatureObject for Options5Feature {
    const TYPE: FeatureType = FeatureType::Options5;
}

/// This feature is currently in preview.
///
/// For more information: [`D3D12_FEATURE_DATA_DISPLAYABLE structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_displayable)
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(transparent)]
pub struct DisplayableFeature(pub(crate) D3D12_FEATURE_DATA_DISPLAYABLE);

impl DisplayableFeature {
    #[inline]
    pub fn displayable_texture(&self) -> bool {
        self.0.DisplayableTexture.into()
    }

    #[inline]
    pub fn shared_resource_compatibility_tier(&self) -> SharedResourceCompatibilityTier {
        self.0.SharedResourceCompatibilityTier.into()
    }
}

impl __Sealed for DisplayableFeature {}

impl FeatureObject for DisplayableFeature {
    const TYPE: FeatureType = FeatureType::Displayable;
}

/// Indicates the level of support that the adapter provides for variable-rate shading (VRS), and indicates whether or not background processing is supported.
///
/// For more information: [`D3D12_FEATURE_DATA_D3D12_OPTIONS6 structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_d3d12_options6)
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(transparent)]
pub struct Options6Feature(pub(crate) D3D12_FEATURE_DATA_D3D12_OPTIONS6);

impl Options6Feature {
    #[inline]
    pub fn additional_shading_rates_supported(&self) -> bool {
        self.0.AdditionalShadingRatesSupported.into()
    }

    #[inline]
    pub fn per_primitive_shading_rate_supported_with_viewport_indexing(&self) -> bool {
        self.0
            .PerPrimitiveShadingRateSupportedWithViewportIndexing
            .into()
    }

    #[inline]
    pub fn variable_shading_rate_tier(&self) -> VariableShadingRateTier {
        self.0.VariableShadingRateTier.into()
    }

    #[inline]
    pub fn shading_rate_image_tile_size(&self) -> u32 {
        self.0.ShadingRateImageTileSize
    }

    #[inline]
    pub fn background_processing_supported(&self) -> bool {
        self.0.BackgroundProcessingSupported.into()
    }
}

impl __Sealed for Options6Feature {}

impl FeatureObject for Options6Feature {
    const TYPE: FeatureType = FeatureType::Options6;
}

/// Indicates the level of support that the adapter provides for mesh and amplification shaders, and for sampler feedback.
///
/// For more information: [`D3D12_FEATURE_DATA_D3D12_OPTIONS7 structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_d3d12_options7)
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(transparent)]
pub struct Options7Feature(pub(crate) D3D12_FEATURE_DATA_D3D12_OPTIONS7);

impl Options7Feature {
    #[inline]
    pub fn mesh_shader_tier(&self) -> MeshShaderTier {
        self.0.MeshShaderTier.into()
    }

    #[inline]
    pub fn sampler_feedback_tier(&self) -> SamplerFeedbackTier {
        self.0.SamplerFeedbackTier.into()
    }
}

impl __Sealed for Options7Feature {}

impl FeatureObject for Options7Feature {
    const TYPE: FeatureType = FeatureType::Options7;
}

/// Indicates the level of support that the adapter provides for mesh and amplification shaders, and for sampler feedback.
///
/// For more information: [`D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPE_COUNT structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_protected_resource_session_type_count)
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct ProtectedResourceSessionTypeCountFeature(
    pub(crate) D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPE_COUNT,
);

impl ProtectedResourceSessionTypeCountFeature {
    #[inline]
    pub fn new(node_index: u32) -> Self {
        Self(D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPE_COUNT {
            NodeIndex: node_index,
            ..Default::default()
        })
    }

    #[inline]
    pub fn count(&self) -> u32 {
        self.0.Count
    }
}

impl __Sealed for ProtectedResourceSessionTypeCountFeature {}

impl FeatureObject for ProtectedResourceSessionTypeCountFeature {
    const TYPE: FeatureType = FeatureType::ProtectedResourceSessionTypeCount;
}

/// Indicates a list of protected resource session types.
///
/// For more information: [`D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPES structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_protected_resource_session_types)
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct ProtectedResourceSessionTypesFeature<'a>(
    pub(crate) D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPES,
    PhantomData<&'a ()>,
);

impl<'a> ProtectedResourceSessionTypesFeature<'a> {
    #[inline]
    pub fn new(node_index: u32, types: &'a mut [u128]) -> Self {
        Self(
            D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPES {
                NodeIndex: node_index,
                Count: types.len() as u32,
                pTypes: types.as_mut_ptr() as *mut _,
            },
            Default::default(),
        )
    }
}

impl __Sealed for ProtectedResourceSessionTypesFeature<'_> {}

impl FeatureObject for ProtectedResourceSessionTypesFeature<'_> {
    const TYPE: FeatureType = FeatureType::ProtectedResourceSessionTypes;
}

/// Indicates whether or not unaligned block-compressed textures are supported.
///
/// For more information: [`D3D12_FEATURE_DATA_D3D12_OPTIONS8 structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_d3d12_options8)
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(transparent)]
pub struct Options8Feature(pub(crate) D3D12_FEATURE_DATA_D3D12_OPTIONS8);

impl Options8Feature {
    #[inline]
    pub fn unaligned_block_textures_supported(&self) -> bool {
        self.0.UnalignedBlockTexturesSupported.into()
    }
}

impl __Sealed for Options8Feature {}

impl FeatureObject for Options8Feature {
    const TYPE: FeatureType = FeatureType::Options8;
}

/// Indicates whether or not support exists for mesh shaders, values of SV_RenderTargetArrayIndex that are 8 or greater,
/// typed resource 64-bit integer atomics, derivative and derivative-dependent texture sample operations, and the level of support for WaveMMA (wave_matrix) operations.
///
/// For more information: [`D3D12_FEATURE_DATA_D3D12_OPTIONS9 structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_d3d12_options9)
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(transparent)]
pub struct Options9Feature(pub(crate) D3D12_FEATURE_DATA_D3D12_OPTIONS9);

impl Options9Feature {
    #[inline]
    pub fn mesh_shader_pipeline_stats_supported(&self) -> bool {
        self.0.MeshShaderPipelineStatsSupported.into()
    }

    #[inline]
    pub fn mesh_shader_supports_full_range_render_target_array_index(&self) -> bool {
        self.0
            .MeshShaderSupportsFullRangeRenderTargetArrayIndex
            .into()
    }

    #[inline]
    pub fn atomic_int64_on_typed_resource_supported(&self) -> bool {
        self.0.AtomicInt64OnTypedResourceSupported.into()
    }

    #[inline]
    pub fn atomic_int64_on_group_shared_supported(&self) -> bool {
        self.0.AtomicInt64OnGroupSharedSupported.into()
    }

    #[inline]
    pub fn derivatives_in_mesh_and_amplification_shaders_supported(&self) -> bool {
        self.0
            .DerivativesInMeshAndAmplificationShadersSupported
            .into()
    }

    #[inline]
    pub fn wave_mma_tier(&self) -> WaveMmaTier {
        self.0.WaveMMATier.into()
    }
}

impl __Sealed for Options9Feature {}

impl FeatureObject for Options9Feature {
    const TYPE: FeatureType = FeatureType::Options9;
}

/// Indicates whether or not the SUM combiner can be used, and whether or not SV_ShadingRate can be set from a mesh shader.
///
/// For more information: [`D3D12_FEATURE_DATA_D3D12_OPTIONS10 structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_d3d12_options10)
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(transparent)]
pub struct Options10Feature(pub(crate) D3D12_FEATURE_DATA_D3D12_OPTIONS10);

impl Options10Feature {
    #[inline]
    pub fn variable_rate_shading_sum_combiner_supported(&self) -> bool {
        self.0.VariableRateShadingSumCombinerSupported.into()
    }

    #[inline]
    pub fn mesh_shader_per_primitive_shading_rate_supported(&self) -> bool {
        self.0.MeshShaderPerPrimitiveShadingRateSupported.into()
    }
}

impl __Sealed for Options10Feature {}

impl FeatureObject for Options10Feature {
    const TYPE: FeatureType = FeatureType::Options10;
}

/// Indicates whether or not 64-bit integer atomics on resources in descriptor heaps are supported.
///
/// For more information: [`D3D12_FEATURE_DATA_D3D12_OPTIONS11 structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_feature_data_d3d12_options11)
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(transparent)]
pub struct Options11Feature(pub(crate) D3D12_FEATURE_DATA_D3D12_OPTIONS11);

impl Options11Feature {
    #[inline]
    pub fn atomic_int64_on_descriptor_heap_resource_supported(&self) -> bool {
        self.0.AtomicInt64OnDescriptorHeapResourceSupported.into()
    }
}

impl __Sealed for Options11Feature {}

impl FeatureObject for Options11Feature {
    const TYPE: FeatureType = FeatureType::Options11;
}
