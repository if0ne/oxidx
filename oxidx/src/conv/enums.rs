use windows::Win32::Graphics::Direct3D12::*;

use crate::conv_enum;

use super::*;

conv_enum!(CommandListType to D3D12_COMMAND_LIST_TYPE);
conv_enum!(CpuPageProperty to D3D12_CPU_PAGE_PROPERTY);
conv_enum!(ConservativeRasterizationTier to D3D12_CONSERVATIVE_RASTERIZATION_TIER);
conv_enum!(CrossNodeSharingTier to D3D12_CROSS_NODE_SHARING_TIER);
conv_enum!(DescriptorHeapType to D3D12_DESCRIPTOR_HEAP_TYPE);
conv_enum!(FeatureType to D3D12_FEATURE);
conv_enum!(FeatureLevel to D3D_FEATURE_LEVEL);
conv_enum!(HeapType to D3D12_HEAP_TYPE);
conv_enum!(MemoryPool to D3D12_MEMORY_POOL);
conv_enum!(MinPrecisionSupport to D3D12_SHADER_MIN_PRECISION_SUPPORT);
conv_enum!(ResourceBindingTier to D3D12_RESOURCE_BINDING_TIER);
conv_enum!(ResourceHeapTier to D3D12_RESOURCE_HEAP_TIER);
conv_enum!(ShaderModel to D3D_SHADER_MODEL);
conv_enum!(RootSignatureVersion to D3D_ROOT_SIGNATURE_VERSION);
conv_enum!(TiledResourcesTier to D3D12_TILED_RESOURCES_TIER);

impl CommandQueuePriority {
    #[inline]
    pub(crate) fn as_raw(&self) -> i32 {
        *self as i32
    }
}

impl From<i32> for CommandQueuePriority {
    #[inline]
    fn from(value: i32) -> Self {
        let value = D3D12_COMMAND_QUEUE_PRIORITY(value);
        match value {
            D3D12_COMMAND_QUEUE_PRIORITY_NORMAL => CommandQueuePriority::Normal,
            D3D12_COMMAND_QUEUE_PRIORITY_HIGH => CommandQueuePriority::High,
            D3D12_COMMAND_QUEUE_PRIORITY_GLOBAL_REALTIME => CommandQueuePriority::GlobalRealtime,
            _ => unreachable!(),
        }
    }
}

impl HeapAlignment {
    #[inline]
    pub(crate) fn as_raw(&self) -> u64 {
        *self as u64
    }
}

impl From<u64> for HeapAlignment {
    #[inline]
    fn from(value: u64) -> Self {
        match value as u32 {
            0 => HeapAlignment::Default,
            D3D12_DEFAULT_RESOURCE_PLACEMENT_ALIGNMENT => HeapAlignment::ResourcePlacement,
            D3D12_DEFAULT_MSAA_RESOURCE_PLACEMENT_ALIGNMENT => HeapAlignment::MsaaResourcePlacement,
            _ => unreachable!(),
        }
    }
}
