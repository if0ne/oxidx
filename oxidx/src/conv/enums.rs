use windows::Win32::Graphics::Direct3D12::*;

use super::*;

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

impl CommandListType {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_COMMAND_LIST_TYPE {
        D3D12_COMMAND_LIST_TYPE(*self as i32)
    }
}

impl CpuPageProperty {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_CPU_PAGE_PROPERTY {
        D3D12_CPU_PAGE_PROPERTY(*self as i32)
    }
}

impl From<D3D12_CPU_PAGE_PROPERTY> for CpuPageProperty {
    fn from(value: D3D12_CPU_PAGE_PROPERTY) -> Self {
        match value {
            D3D12_CPU_PAGE_PROPERTY_UNKNOWN => CpuPageProperty::Unknown,
            D3D12_CPU_PAGE_PROPERTY_NOT_AVAILABLE => CpuPageProperty::NotAvailable,
            D3D12_CPU_PAGE_PROPERTY_WRITE_COMBINE => CpuPageProperty::WriteCombine,
            D3D12_CPU_PAGE_PROPERTY_WRITE_BACK => CpuPageProperty::WriteBack,
            _ => unreachable!(),
        }
    }
}

impl From<D3D12_COMMAND_LIST_TYPE> for CommandListType {
    #[inline]
    fn from(value: D3D12_COMMAND_LIST_TYPE) -> Self {
        match value {
            D3D12_COMMAND_LIST_TYPE_DIRECT => CommandListType::Direct,
            D3D12_COMMAND_LIST_TYPE_BUNDLE => CommandListType::Bundle,
            D3D12_COMMAND_LIST_TYPE_COMPUTE => CommandListType::Compute,
            D3D12_COMMAND_LIST_TYPE_COPY => CommandListType::Copy,
            D3D12_COMMAND_LIST_TYPE_VIDEO_DECODE => CommandListType::VideoDecode,
            D3D12_COMMAND_LIST_TYPE_VIDEO_PROCESS => CommandListType::VideoProcess,
            D3D12_COMMAND_LIST_TYPE_VIDEO_ENCODE => CommandListType::VideoEncode,
            _ => unreachable!(),
        }
    }
}

impl DescriptorHeapType {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_DESCRIPTOR_HEAP_TYPE {
        D3D12_DESCRIPTOR_HEAP_TYPE(*self as i32)
    }
}

impl From<D3D12_DESCRIPTOR_HEAP_TYPE> for DescriptorHeapType {
    #[inline]
    fn from(value: D3D12_DESCRIPTOR_HEAP_TYPE) -> Self {
        match value {
            D3D12_DESCRIPTOR_HEAP_TYPE_RTV => DescriptorHeapType::Rtv,
            D3D12_DESCRIPTOR_HEAP_TYPE_DSV => DescriptorHeapType::Dsv,
            D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV => DescriptorHeapType::CbvSrvUav,
            D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER => DescriptorHeapType::Sampler,
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

impl HeapType {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_HEAP_TYPE {
        D3D12_HEAP_TYPE(*self as i32)
    }
}

impl From<D3D12_HEAP_TYPE> for HeapType {
    #[inline]
    fn from(value: D3D12_HEAP_TYPE) -> Self {
        match value {
            D3D12_HEAP_TYPE_DEFAULT => HeapType::Default,
            D3D12_HEAP_TYPE_UPLOAD => HeapType::Upload,
            D3D12_HEAP_TYPE_READBACK => HeapType::Readback,
            D3D12_HEAP_TYPE_CUSTOM => HeapType::Custom,
            D3D12_HEAP_TYPE_GPU_UPLOAD => HeapType::GpuUpload,
            _ => unreachable!(),
        }
    }
}

impl MemoryPool {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_MEMORY_POOL {
        D3D12_MEMORY_POOL(*self as i32)
    }
}

impl From<D3D12_MEMORY_POOL> for MemoryPool {
    #[inline]
    fn from(value: D3D12_MEMORY_POOL) -> Self {
        match value {
            D3D12_MEMORY_POOL_UNKNOWN => MemoryPool::Unknown,
            D3D12_MEMORY_POOL_L0 => MemoryPool::L0,
            D3D12_MEMORY_POOL_L1 => MemoryPool::L1,
            _ => unreachable!(),
        }
    }
}
