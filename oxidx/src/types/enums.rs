use windows::Win32::Graphics::Direct3D12::*;

/// Defines priority levels for a command queue.
///
/// For more information: [`D3D12_COMMAND_QUEUE_PRIORITY enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_command_queue_priority)
#[derive(Debug, Default, Clone, Copy)]
#[repr(i32)]
pub enum CommandQueuePriority {
    #[default]
    /// Normal priority.
    Normal = D3D12_COMMAND_QUEUE_PRIORITY_NORMAL.0,

    /// High priority.
    High = D3D12_COMMAND_QUEUE_PRIORITY_HIGH.0,

    /// Global realtime priority.
    GlobalRealtime = D3D12_COMMAND_QUEUE_PRIORITY_GLOBAL_REALTIME.0,
}

/// Specifies the type of a command list.
///
/// For more information: [`D3D12_COMMAND_LIST_TYPE enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_command_list_type)
#[derive(Debug, Default, Clone, Copy)]
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

/// Specifies a type of descriptor heap.
///
/// For more information: [`D3D12_DESCRIPTOR_HEAP_TYPE enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_descriptor_heap_type)
#[derive(Clone, Copy, Debug, Default)]
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
