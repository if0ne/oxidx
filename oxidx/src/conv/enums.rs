use windows::Win32::Graphics::Direct3D12::*;

use super::{CommandListType, CommandQueuePriority, DescriptorHeapType};

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
